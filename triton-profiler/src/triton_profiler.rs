use std::cmp::max;
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Duration;
use std::time::Instant;
use std::vec;

use colored::Color;
use colored::ColoredString;
use colored::Colorize;
use criterion::profiler::Profiler;
use twenty_first::shared_math::other::log_2_floor;
use unicode_width::UnicodeWidthStr;

const GET_PROFILE_OUTPUT_AS_YOU_GO_ENV_VAR_NAME: &str = "PROFILE_AS_YOU_GO";

#[derive(Clone, Debug)]
struct Task {
    name: String,
    parent_index: Option<usize>,
    depth: usize,
    time: Duration,
    task_type: TaskType,

    /// The type of work the task is doing. Helps tracking time across specific tasks. For
    /// example, if the task is building a Merkle tree, then the category could be "hash".
    category: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TaskType {
    Generic,
    IterationZero,
    AnyOtherIteration,
}

pub struct TritonProfiler {
    name: String,
    timer: Instant,
    stack: Vec<(usize, String)>,
    profile: Vec<Task>,
    total_time: Duration,
}

impl TritonProfiler {
    #[allow(clippy::new_without_default)]
    pub fn new(name: &str) -> Self {
        TritonProfiler {
            name: name.to_owned(),
            timer: Instant::now(),
            stack: vec![],
            profile: vec![],
            total_time: Duration::ZERO,
        }
    }

    fn ignoring(&self) -> bool {
        if let Some(top) = self.stack.last() {
            if self.profile[top.0].task_type == TaskType::AnyOtherIteration {
                return true;
            }
        }
        false
    }

    fn has_younger_sibling(&self, index: usize) -> bool {
        for (idx, task) in self.profile.iter().enumerate() {
            if idx == index {
                continue;
            }
            if task.parent_index == self.profile[index].parent_index && idx > index {
                return true;
            }
        }
        false
    }
    pub fn finish(&mut self) {
        assert!(!self.profile.is_empty(), "Nothing to finish.");
        assert!(
            self.stack.is_empty(),
            "Cannot finish before stack is empty."
        );
        self.total_time = self.timer.elapsed();
    }

    pub fn report(
        &mut self,
        cycle_count: Option<usize>,
        padded_height: Option<usize>,
        fri_domain_len: Option<usize>,
    ) -> Report {
        assert!(!self.profile.is_empty(), "Nothing to report on.");
        assert!(
            self.stack.is_empty(),
            "Cannot generate report before stack is empty."
        );
        assert_ne!(
            Duration::ZERO,
            self.total_time,
            "Cannot generate report before profiler has finished. Call `.finish()` first."
        );

        let mut report: Vec<TaskReport> = vec![];
        let total_tracked_time = (self.total_time.as_nanos()
            - self
                .profile
                .iter()
                .filter(|t| t.task_type == TaskType::AnyOtherIteration)
                .map(|t| t.time.as_nanos())
                .sum::<u128>()) as f64
            / 1_000_000_000f64;
        println!(
            "total time: {}s and tracked: {}s",
            self.total_time.as_secs_f64(),
            total_tracked_time,
        );

        // collect all categories and their total times
        // todo: this can count the same category multiple times if it's nested
        let mut category_times = HashMap::new();
        for task in self.profile.iter() {
            if let Some(category) = &task.category {
                category_times
                    .entry(category.clone())
                    .and_modify(|e| *e += task.time)
                    .or_insert(task.time);
            }
        }

        for (task_index, task) in self.profile.iter().enumerate() {
            // compute this task's time relative to total duration
            let relative_time = task.time.as_secs_f64() / self.total_time.as_secs_f64();
            let weight = match task.task_type {
                TaskType::AnyOtherIteration => Weight::LikeNothing,
                _ => Weight::weigh(task.time.as_secs_f64() / self.total_time.as_secs_f64()),
            };

            let is_last_sibling = !self.has_younger_sibling(task_index);

            // compute this task's ancestors
            let mut ancestors: Vec<usize> = vec![];
            let mut current_ancestor_index = task.parent_index;
            while let Some(cai) = current_ancestor_index {
                ancestors.push(cai);
                current_ancestor_index = report[cai].parent_index;
            }
            ancestors.reverse();

            let relative_category_time = task.category.clone().map(|category| {
                let category_time = category_times.get(&category).unwrap();
                task.time.as_secs_f64() / category_time.as_secs_f64()
            });

            report.push(TaskReport {
                name: task.name.clone(),
                parent_index: task.parent_index,
                depth: task.depth,
                time: task.time,
                relative_time,
                category: task.category.clone(),
                relative_category_time,
                is_last_sibling,
                ancestors,
                weight,
                younger_max_weight: Weight::LikeNothing,
            });
        }

        // pass over report again to fix forward references
        for task_index in 0..report.len() {
            let task = &self.profile[task_index];
            let mut younger_siblings: Vec<usize> = vec![];
            for (tsk_idx, tsk) in self.profile.iter().enumerate() {
                if tsk.parent_index == task.parent_index && tsk_idx > task_index {
                    younger_siblings.push(tsk_idx);
                }
            }
            let mut younger_max_weight: Weight = Weight::LikeNothing;
            for sibling in younger_siblings.iter() {
                younger_max_weight = max(younger_max_weight, report[*sibling].weight);
            }

            report[task_index].younger_max_weight = younger_max_weight;
        }

        Report {
            tasks: report,
            name: self.name.clone(),
            total_time: self.total_time,
            category_times,
            cycle_count,
            padded_height,
            fri_domain_len,
        }
    }

    pub fn start(&mut self, name: &str, category: Option<String>) {
        if !self.ignoring() {
            self.plain_start(name, TaskType::Generic, category);
        }
    }

    fn plain_start(&mut self, name: &str, task_type: TaskType, category: Option<String>) {
        let parent_index = self.stack.last().map(|(u, _)| *u);
        let now = self.timer.elapsed();

        self.stack.push((self.profile.len(), name.to_owned()));

        self.profile.push(Task {
            name: name.to_owned(),
            parent_index,
            depth: self.stack.len(),
            time: now,
            task_type,
            category,
        });

        if std::env::var(GET_PROFILE_OUTPUT_AS_YOU_GO_ENV_VAR_NAME).is_ok() {
            println!("start: {name}");
        }
    }

    pub fn iteration_zero(&mut self, name: &str, category: Option<String>) {
        if self.ignoring() {
            return;
        }

        assert!(
            !self.stack.is_empty(),
            "Profiler stack is empty; can't iterate."
        );

        let top_index = self.stack[self.stack.len() - 1].0;
        let top_type = self.profile[top_index].task_type.clone();

        if top_type != TaskType::IterationZero && top_type != TaskType::AnyOtherIteration {
            // start
            self.plain_start("iteration 0", TaskType::IterationZero, category);
            return;
        }

        assert!(
            self.stack.len() >= 2,
            "To profile zeroth iteration, stack must be at least 2-high, but got height of {}",
            self.stack.len()
        );

        let runner_up = self.stack[self.stack.len() - 2].1.clone();

        assert_eq!(
            runner_up, name,
            "To profile zeroth iteration, name must match with top of stack."
        );

        if top_type == TaskType::IterationZero {
            // switch
            // stop iteration zero
            self.plain_stop();

            // start all other iterations
            self.plain_start(
                "all other iterations",
                TaskType::AnyOtherIteration,
                category,
            );
        }

        // top == *"all other iterations"
        // in this case we do nothing
    }

    fn plain_stop(&mut self) {
        let (index, name) = self.stack.pop().unwrap();
        let now = self.timer.elapsed();
        let duration = now - self.profile[index].time;
        self.profile[index].time = duration;

        if std::env::var(GET_PROFILE_OUTPUT_AS_YOU_GO_ENV_VAR_NAME).is_ok() {
            println!("stop:  {name} – took {duration:.2?}");
        }
    }

    pub fn stop(&mut self, name: &str) {
        assert!(
            !self.stack.is_empty(),
            "Cannot stop any tasks when stack is empty.",
        );

        let top = self.stack.last().unwrap().1.clone();
        if top == *"iteration 0" || top == *"all other iterations" {
            assert!(
                self.stack.len() >= 2,
                "To close profiling of zeroth iteration, stack must be at least 2-high, \
                but got stack of height {}.",
                self.stack.len(),
            );
            if self.stack[self.stack.len() - 2].1 == *name {
                self.plain_stop();
                self.plain_stop();
            }
        } else {
            assert_eq!(
                top,
                name.to_owned(),
                "Profiler stack is LIFO; can't pop in disorder."
            );

            self.plain_stop();
        }
    }
}

impl Profiler for TritonProfiler {
    fn start_profiling(&mut self, benchmark_id: &str, benchmark_dir: &std::path::Path) {
        let dir = benchmark_dir
            .to_str()
            .expect("Directory must be valid unicode");
        let name = format!("{dir}{benchmark_id}");
        let category = None;
        self.start(&name, category);
    }

    fn stop_profiling(&mut self, benchmark_id: &str, benchmark_dir: &std::path::Path) {
        let dir = benchmark_dir
            .to_str()
            .expect("Directory must be valid unicode");
        let name = format!("{dir}{benchmark_id}");
        self.stop(&name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Weight {
    LikeNothing,
    VeryLittle,
    Light,
    Noticeable,
    Heavy,
    Massive,
    SuperMassive,
}

impl Weight {
    /// Assign a weight based on a relative cost, which is a number between 0 and 1.
    fn weigh(relative_cost: f64) -> Weight {
        match relative_cost {
            rc if rc >= 0.4 => Weight::SuperMassive,
            rc if rc >= 0.3 => Weight::Massive,
            rc if rc >= 0.2 => Weight::Heavy,
            rc if rc >= 0.1 => Weight::Noticeable,
            rc if rc >= 0.07 => Weight::Light,
            rc if rc >= 0.04 => Weight::VeryLittle,
            _ => Weight::LikeNothing,
        }
    }

    fn color(&self) -> Color {
        use Color::*;
        use Weight::*;
        match self {
            LikeNothing => TrueColor {
                r: 120,
                g: 120,
                b: 120,
            },
            VeryLittle => TrueColor {
                r: 200,
                g: 200,
                b: 200,
            },
            Light => White,
            Noticeable => TrueColor {
                r: 255,
                g: 255,
                b: 120,
            },
            Heavy => TrueColor {
                r: 255,
                g: 150,
                b: 0,
            },
            Massive => TrueColor {
                r: 255,
                g: 75,
                b: 0,
            },
            SuperMassive => Red,
        }
    }
}

#[derive(Debug)]
struct TaskReport {
    name: String,
    parent_index: Option<usize>,
    depth: usize,
    time: Duration,
    relative_time: f64,
    category: Option<String>,
    relative_category_time: Option<f64>,
    is_last_sibling: bool,
    ancestors: Vec<usize>,
    weight: Weight,
    younger_max_weight: Weight,
}

#[derive(Debug)]
pub struct Report {
    name: String,
    tasks: Vec<TaskReport>,
    total_time: Duration,
    category_times: HashMap<String, Duration>,
    cycle_count: Option<usize>,
    padded_height: Option<usize>,
    fri_domain_len: Option<usize>,
}

impl Report {
    pub fn placeholder() -> Self {
        Report {
            name: "".to_string(),
            tasks: vec![],
            total_time: Duration::ZERO,
            category_times: HashMap::new(),
            cycle_count: None,
            padded_height: None,
            fri_domain_len: None,
        }
    }

    fn display_time_aligned(time: Duration) -> String {
        let unaligned_time = format!("{time:.2?}");
        let time_components: Vec<_> = unaligned_time.split('.').collect();
        if time_components.len() != 2 {
            return unaligned_time;
        }

        format!("{:>3}.{:<4}", time_components[0], time_components[1])
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_name_width = self
            .tasks
            .iter()
            .map(|t| t.name.width() + 2 * t.depth)
            .max()
            .expect("No tasks to generate report from.");
        let max_category_name_width = self
            .category_times
            .keys()
            .map(|k| k.width())
            .max()
            .unwrap_or(0);

        let title = format!("### {}", self.name).bold();
        let max_width = max(max_name_width, title.width());
        let total_time_string = Report::display_time_aligned(self.total_time).bold();
        let separation = String::from_utf8(vec![b' '; max_width - title.width()]).unwrap();
        let share_string = "Share".to_string().bold();
        let category_string = match self.category_times.is_empty() {
            true => "".to_string(),
            false => "Category".to_string(),
        }
        .bold();
        writeln!(
            f,
            "{title}{separation}   {total_time_string}   {share_string}  {category_string}"
        )?;

        for task in self.tasks.iter() {
            for ancestor_index in task.ancestors.iter() {
                let mut spacer: ColoredString = if self.tasks[*ancestor_index].is_last_sibling {
                    "  ".normal()
                } else {
                    "│ ".normal()
                };
                let uncle_weight = &self.tasks[*ancestor_index].younger_max_weight;
                spacer = spacer.color(uncle_weight.color());

                write!(f, "{spacer}")?;
            }
            let tracer = if task.is_last_sibling {
                "└".normal()
            } else {
                "├".normal()
            }
            .color(
                max(&task.weight, &task.younger_max_weight)
                    .to_owned()
                    .color(),
            );
            let dash = "─".color(task.weight.color());
            write!(f, "{tracer}{dash}")?;

            let padding_length = max_width - task.name.len() - 2 * task.depth;
            assert!(
                padding_length < (1 << 60),
                "max width: {max_name_width}, width: {}",
                task.name.len(),
            );
            let task_name_colored = task.name.color(task.weight.color());
            let mut task_time = Report::display_time_aligned(task.time);
            while task_time.width() < 10 {
                task_time.push(b' '.into());
            }
            let task_time_colored = task_time.color(task.weight.color());
            let padding = String::from_utf8(vec![b' '; padding_length]).unwrap();
            let relative_time_string =
                format!("{:>6}", format!("{:2.2}%", 100.0 * task.relative_time));
            let relative_time_string_colored = relative_time_string.color(task.weight.color());
            let category_name = task.category.as_deref().unwrap_or("");
            let relative_category_time = task
                .relative_category_time
                .map(|t| format!("{:>6}", format!("{:2.2}%", 100.0 * t)))
                .unwrap_or("".to_string());
            let category_and_relative_time = match task.category.is_some() {
                true => format!(
                    "({category_name:<max_category_name_width$} – {relative_category_time})"
                ),
                false => "".to_string(),
            };
            f.write_fmt(format_args!(
                "{task_name_colored}{padding}   \
                {task_time_colored}{relative_time_string_colored} \
                {category_and_relative_time}\n"
            ))?;
        }

        if !self.category_times.is_empty() {
            writeln!(f)?;
            let category_title = ("### Categories").to_string().bold();
            writeln!(f, "{category_title}")?;
            for (category, &category_time) in self.category_times.iter() {
                let category_relative_time =
                    category_time.as_secs_f64() / self.total_time.as_secs_f64();
                let category_color = Weight::weigh(category_relative_time).color();
                let category_relative_time =
                    format!("{:>6}", format!("{:2.2}%", 100.0 * category_relative_time));
                let category_time = Report::display_time_aligned(category_time);

                let padding_length = max_category_name_width - category.width();
                let padding = String::from_utf8(vec![b' '; padding_length]).unwrap();

                let category = category.color(category_color);
                let category_time = category_time.color(category_color);
                let category_relative_time = category_relative_time.color(category_color);
                writeln!(
                    f,
                    "{category}{padding}   {category_time} {category_relative_time}"
                )?;
            }
        }

        if self.cycle_count.is_some()
            || self.padded_height.is_some()
            || self.fri_domain_len.is_some()
        {
            writeln!(f)?;
        }

        let total_time = self.total_time.as_millis() as usize;
        if let Some(cycle_count) = self.cycle_count {
            if total_time != 0 {
                let freq = 1_000 * cycle_count / total_time;
                writeln!(
                    f,
                    "Clock frequency is {freq} Hz ({cycle_count} clock cycles / {total_time} ms)",
                )?;
            }
        }

        if let Some(padded_height) = self.padded_height {
            if total_time != 0 {
                let optimal_freq = 1_000 * padded_height / total_time;
                writeln!(
                    f,
                    "Optimal clock frequency is {optimal_freq} Hz \
                    ({padded_height} padded height / {total_time} ms)",
                )?;
            }
        }

        if let Some(fri_domain_length) = self.fri_domain_len {
            if fri_domain_length != 0 {
                let log_2_fri_domain_length = log_2_floor(fri_domain_length as u128);
                writeln!(f, "FRI domain length is 2^{log_2_fri_domain_length}")?;
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! prof_start {
    ($p: ident, $s : expr, $c : expr) => {
        if let Some(profiler) = $p.as_mut() {
            profiler.start($s, Some($c.to_string()));
        }
    };
    ($p: ident, $s : expr) => {
        if let Some(profiler) = $p.as_mut() {
            profiler.start($s, None);
        }
    };
}

#[macro_export]
macro_rules! prof_stop {
    ($p: ident, $s : expr) => {
        if let Some(profiler) = $p.as_mut() {
            profiler.stop($s);
        }
    };
}

#[macro_export]
macro_rules! prof_itr0 {
    ($p : ident, $s : expr, $c : expr) => {
        if let Some(profiler) = $p.as_mut() {
            profiler.iteration_zero($s, Some($c.to_string()));
        }
    };
    ($p : ident, $s : expr) => {
        if let Some(profiler) = $p.as_mut() {
            profiler.iteration_zero($s, None);
        }
    };
}

#[cfg(test)]
pub mod triton_profiler_tests {
    use std::thread::sleep;
    use std::time::Duration;

    use rand::rngs::ThreadRng;
    use rand::Rng;
    use rand::RngCore;

    use super::*;

    fn random_task_name(rng: &mut ThreadRng) -> String {
        let alphabet = "abcdefghijklmnopqrstuvwxyz_";
        let length = (rng.next_u32() as usize % 10) + 2;

        (0..length)
            .map(|_| (rng.next_u32() as usize) % alphabet.len())
            .map(|i| alphabet.get(i..=i).unwrap())
            .collect()
    }

    fn random_category(rng: &mut ThreadRng) -> String {
        let options = vec!["setup", "compute", "drop", "cleanup"];
        options[rng.next_u32() as usize % options.len()].to_string()
    }

    #[test]
    fn test_sanity() {
        let mut rng = rand::thread_rng();
        let mut stack = vec![];
        let steps = 100;

        let mut profiler = TritonProfiler::new("Sanity Test");

        for step in 0..steps {
            let steps_left = steps - step;
            assert!((1..=steps).contains(&steps_left));
            assert!(
                stack.len() <= steps_left,
                "stack len {} > steps left {}",
                stack.len(),
                steps_left
            );

            let pushable = rng.next_u32() % 2 == 1 && stack.len() + 1 < steps_left;
            let poppable = ((!pushable && rng.next_u32() % 2 == 1)
                || (stack.len() == steps_left && steps_left > 0))
                && !stack.is_empty();

            if pushable {
                let name = random_task_name(&mut rng);
                let category = match rng.gen() {
                    true => Some(random_category(&mut rng)),
                    false => None,
                };
                stack.push(name.clone());
                profiler.start(&name, category);
            }

            sleep(Duration::from_micros(
                (rng.next_u64() % 10) * (rng.next_u64() % 10) * (rng.next_u64() % 10),
            ));

            if poppable {
                let name = stack.pop().unwrap();
                profiler.stop(&name);
            }
        }

        profiler.finish();
        println!("{}", profiler.report(None, None, None));
    }

    #[test]
    fn clk_freq() {
        let mut profiler = Some(TritonProfiler::new("Clock Frequency Test"));
        prof_start!(profiler, "clk_freq_test");
        sleep(Duration::from_millis(3));
        prof_stop!(profiler, "clk_freq_test");
        let mut profiler = profiler.unwrap();
        profiler.finish();
        println!("{}", profiler.report(None, None, None));
        println!("{}", profiler.report(Some(0), Some(0), Some(0)));
        println!("{}", profiler.report(Some(10), Some(12), Some(13)));
    }

    #[test]
    fn macros() {
        let mut profiler = TritonProfiler::new("Macro Test");
        let mut profiler_ref = Some(&mut profiler);
        let mut rng = rand::thread_rng();
        let mut stack = vec![];
        let steps = 100;

        for step in 0..steps {
            let steps_left = steps - step;
            let pushable = rng.gen() && stack.len() + 1 < steps_left;
            let poppable = ((!pushable && rng.gen())
                || (stack.len() == steps_left && steps_left > 0))
                && !stack.is_empty();

            if pushable {
                let name = random_task_name(&mut rng);
                let category = random_category(&mut rng);
                stack.push(name.clone());
                match rng.gen() {
                    true => prof_start!(profiler_ref, &name, category),
                    false => prof_start!(profiler_ref, &name),
                }
            }

            sleep(Duration::from_micros(
                (rng.next_u64() % 10) * (rng.next_u64() % 10) * (rng.next_u64() % 10),
            ));

            if poppable {
                let name = stack.pop().unwrap();
                prof_stop!(profiler_ref, &name);
            }
        }

        profiler.finish();
        println!("{}", profiler.report(None, None, None));
    }
}
