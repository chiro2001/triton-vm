//! Constraint circuits are a way to represent constraint polynomials in a way that is amenable
//! to optimizations. The constraint circuit is a directed acyclic graph (DAG) of
//! [`CircuitExpression`]s, where each `CircuitExpression` is a node in the graph. The edges of the
//! graph are labeled with [`BinOp`]s. The leafs of the graph are the inputs to the constraint
//! polynomial, and the (multiple) roots of the graph are the outputs of all the
//! constraint polynomials, with each root corresponding to a different constraint polynomial.
//! Because the graph has multiple roots, it is called a “multitree.”

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::iter::Sum;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::rc::Rc;

use ndarray::ArrayView2;
use num_traits::One;
use num_traits::Zero;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::x_field_element::XFieldElement;

use CircuitExpression::*;

use crate::table::challenges::ChallengeId;
use crate::table::challenges::Challenges;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
}

impl Eq for BinOp {}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
        }
    }
}

/// Describes the position of a variable in a constraint polynomial in the row layout applicable
/// for a certain kind of constraint polynomial.
///
/// The position of variable in a constraint polynomial is, in principle, a `usize`. However,
/// depending on the type of the constraint polynomial, this index may be an index into a single
/// row (for initial, consistency and terminal constraints), or a pair of adjacent rows (for
/// transition constraints). Additionally, the index may refer to a column in the base table, or
/// a column in the extension table. This trait abstracts over these possibilities, and provides
/// a uniform interface for accessing the index.
///
/// Having `Clone + Copy + Hash + PartialEq + Eq` helps putting `InputIndicator`s into containers.
pub trait InputIndicator: Debug + Clone + Copy + Hash + PartialEq + Eq + Display {
    /// `true` iff `self` refers to a column in the base table.
    fn is_base_table_column(&self) -> bool;

    fn base_col_index(&self) -> usize;
    fn ext_col_index(&self) -> usize;

    fn evaluate(
        &self,
        base_table: ArrayView2<BFieldElement>,
        ext_table: ArrayView2<XFieldElement>,
    ) -> XFieldElement;
}

/// The position of a variable in a constraint polynomial that operates on a single row of the
/// execution trace.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SingleRowIndicator {
    BaseRow(usize),
    ExtRow(usize),
}

impl Display for SingleRowIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SingleRowIndicator::*;
        let input_indicator: String = match self {
            BaseRow(i) => format!("base_row[{i}]"),
            ExtRow(i) => format!("ext_row[{i}]"),
        };

        writeln!(f, "{input_indicator}")
    }
}

impl InputIndicator for SingleRowIndicator {
    fn is_base_table_column(&self) -> bool {
        use SingleRowIndicator::*;
        matches!(self, BaseRow(_))
    }

    fn base_col_index(&self) -> usize {
        use SingleRowIndicator::*;
        match self {
            BaseRow(i) => *i,
            ExtRow(_) => panic!("not a base row"),
        }
    }

    fn ext_col_index(&self) -> usize {
        use SingleRowIndicator::*;
        match self {
            BaseRow(_) => panic!("not an ext row"),
            ExtRow(i) => *i,
        }
    }

    fn evaluate(
        &self,
        base_table: ArrayView2<BFieldElement>,
        ext_table: ArrayView2<XFieldElement>,
    ) -> XFieldElement {
        use SingleRowIndicator::*;
        match self {
            BaseRow(i) => base_table[[0, *i]].lift(),
            ExtRow(i) => ext_table[[0, *i]],
        }
    }
}

/// The position of a variable in a constraint polynomial that operates on two rows (current and
/// next) of the execution trace.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DualRowIndicator {
    CurrentBaseRow(usize),
    CurrentExtRow(usize),
    NextBaseRow(usize),
    NextExtRow(usize),
}

impl Display for DualRowIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DualRowIndicator::*;
        let input_indicator: String = match self {
            CurrentBaseRow(i) => format!("current_base_row[{i}]"),
            CurrentExtRow(i) => format!("current_ext_row[{i}]"),
            NextBaseRow(i) => format!("next_base_row[{i}]"),
            NextExtRow(i) => format!("next_ext_row[{i}]"),
        };

        writeln!(f, "{input_indicator}")
    }
}

impl InputIndicator for DualRowIndicator {
    fn is_base_table_column(&self) -> bool {
        use DualRowIndicator::*;
        matches!(self, CurrentBaseRow(_) | NextBaseRow(_))
    }

    fn base_col_index(&self) -> usize {
        use DualRowIndicator::*;
        match self {
            CurrentBaseRow(i) | NextBaseRow(i) => *i,
            CurrentExtRow(_) | NextExtRow(_) => panic!("not a base row"),
        }
    }

    fn ext_col_index(&self) -> usize {
        use DualRowIndicator::*;
        match self {
            CurrentBaseRow(_) | NextBaseRow(_) => panic!("not an ext row"),
            CurrentExtRow(i) | NextExtRow(i) => *i,
        }
    }

    fn evaluate(
        &self,
        base_table: ArrayView2<BFieldElement>,
        ext_table: ArrayView2<XFieldElement>,
    ) -> XFieldElement {
        use DualRowIndicator::*;
        match self {
            CurrentBaseRow(i) => base_table[[0, *i]].lift(),
            CurrentExtRow(i) => ext_table[[0, *i]],
            NextBaseRow(i) => base_table[[1, *i]].lift(),
            NextExtRow(i) => ext_table[[1, *i]],
        }
    }
}

/// A circuit expression is the recursive data structure that represents the constraint polynomials.
/// It is a directed, acyclic graph of binary operations on the variables of the constraint
/// polynomials, constants, and challenges. It has multiple roots, making it a “multitree.” Each
/// root corresponds to one constraint polynomial.
///
/// The leafs of the tree are
/// - constants in the base field, _i.e._, [`BFieldElement`]s,
/// - constants in the extension field, _i.e._, [`XFieldElement`]s,
/// - input variables, _i.e._, entries from the Algebraic Execution Trace, and
/// - challenges, _i.e._, (pseudo-)random values sampled through the Fiat-Shamir heuristic.
///
/// An inner node, representing some binary operation, is either addition, multiplication, or
/// subtraction. The left and right children of the node are the operands of the binary operation.
/// The left and right children are not themselves `CircuitExpression`s, but rather
/// [`ConstraintCircuit`]s, which is a wrapper around `CircuitExpression` that manages additional
/// bookkeeping information.
#[derive(Debug, Clone)]
pub enum CircuitExpression<II: InputIndicator> {
    XConstant(XFieldElement),
    BConstant(BFieldElement),
    Input(II),
    Challenge(ChallengeId),
    BinaryOperation(
        BinOp,
        Rc<RefCell<ConstraintCircuit<II>>>,
        Rc<RefCell<ConstraintCircuit<II>>>,
    ),
}

impl<II: InputIndicator> Hash for CircuitExpression<II> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            BConstant(bfe) => {
                "bfe".hash(state);
                bfe.hash(state);
            }
            XConstant(xfe) => {
                "xfe".hash(state);
                xfe.hash(state);
            }
            Input(index) => {
                "input".hash(state);
                index.hash(state);
            }
            Challenge(table_challenge_id) => {
                "challenge".hash(state);
                table_challenge_id.hash(state);
            }
            BinaryOperation(binop, lhs, rhs) => {
                "binop".hash(state);
                binop.hash(state);
                lhs.as_ref().borrow().hash(state);
                rhs.as_ref().borrow().hash(state);
            }
        }
    }
}

impl<II: InputIndicator> PartialEq for CircuitExpression<II> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            XConstant(self_xfe) => match other {
                XConstant(other_xfe) => self_xfe == other_xfe,
                _ => false,
            },
            BConstant(self_bfe) => match other {
                BConstant(other_bfe) => self_bfe == other_bfe,
                _ => false,
            },
            Input(self_input) => match other {
                Input(other_input) => self_input == other_input,
                _ => false,
            },
            Challenge(self_challenge_id) => match other {
                Challenge(other_challenge_id) => self_challenge_id == other_challenge_id,
                _ => false,
            },
            BinaryOperation(binop_self, lhs_self, rhs_self) => {
                match other {
                    BinaryOperation(binop_other, lhs_other, rhs_other) => {
                        // a = b `op0` c,
                        // d = e `op1` f =>
                        // a = d <= op0 == op1 && b == e && c ==f
                        binop_self == binop_other && lhs_self == lhs_other && rhs_self == rhs_other
                    }

                    _ => false,
                }
            }
        }
    }
}

impl<II: InputIndicator> Hash for ConstraintCircuit<II> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.expression.hash(state)
    }
}

impl<II: InputIndicator> Hash for ConstraintCircuitMonad<II> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.circuit.as_ref().borrow().hash(state)
    }
}

/// A wrapper around a [`CircuitExpression`] that manages additional bookkeeping information.
#[derive(Clone, Debug)]
pub struct ConstraintCircuit<II: InputIndicator> {
    pub id: usize,
    pub visited_counter: usize,
    pub expression: CircuitExpression<II>,
}

impl<II: InputIndicator> Eq for ConstraintCircuit<II> {}

impl<II: InputIndicator> PartialEq for ConstraintCircuit<II> {
    /// Calculate equality of circuits. In particular, this function does *not* attempt to
    /// simplify or reduce neutral terms or products. So this comparison will return false for
    /// `a == a + 0`. It will also return false for `XFieldElement(7) == BFieldElement(7)`
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression
    }
}

impl<II: InputIndicator> Display for ConstraintCircuit<II> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.expression {
            XConstant(xfe) => {
                write!(f, "{xfe}")
            }
            BConstant(bfe) => {
                write!(f, "{bfe}")
            }
            Input(input) => write!(f, "{input} "),
            Challenge(self_challenge_id) => {
                write!(f, "#{self_challenge_id}")
            }
            BinaryOperation(operation, lhs, rhs) => {
                write!(
                    f,
                    "({}) {} ({})",
                    lhs.as_ref().borrow(),
                    operation,
                    rhs.as_ref().borrow()
                )
            }
        }
    }
}

impl<II: InputIndicator> ConstraintCircuit<II> {
    /// Increment `visited_counter` by one for each reachable node
    fn traverse_single(&mut self) {
        self.visited_counter += 1;
        if let BinaryOperation(_, lhs, rhs) = self.expression.borrow_mut() {
            lhs.as_ref().borrow_mut().traverse_single();
            rhs.as_ref().borrow_mut().traverse_single();
        }
    }

    /// Count how many times each reachable node is reached when traversing from
    /// the starting points that are given as input. The result is stored in the
    /// `visited_counter` field in each node.
    pub fn traverse_multiple(ccs: &mut [ConstraintCircuit<II>]) {
        for cc in ccs.iter_mut() {
            assert!(
                cc.visited_counter.is_zero(),
                "visited counter must be zero before starting count"
            );
            cc.traverse_single();
        }
    }

    /// Reset the visited counters for the entire subtree
    fn reset_visited_counters(&mut self) {
        self.visited_counter = 0;

        if let BinaryOperation(_, lhs, rhs) = &self.expression {
            lhs.as_ref().borrow_mut().reset_visited_counters();
            rhs.as_ref().borrow_mut().reset_visited_counters();
        }
    }

    /// Verify that all IDs in the subtree are unique. Panics otherwise.
    fn inner_has_unique_ids(&mut self, ids: &mut HashMap<usize, ConstraintCircuit<II>>) {
        let node_with_repeated_id = ids.insert(self.id, self.clone());
        assert!(
            !self.visited_counter.is_zero() || node_with_repeated_id.is_none(),
            "ID = {} was repeated. Self was: {self:?}, other node: {:?}",
            self.id,
            node_with_repeated_id.unwrap(),
        );
        self.visited_counter += 1;
        if let BinaryOperation(_, lhs, rhs) = &self.expression {
            lhs.as_ref().borrow_mut().inner_has_unique_ids(ids);
            rhs.as_ref().borrow_mut().inner_has_unique_ids(ids);
        }
    }

    /// Verify that a multicircuit has unique IDs. Panics otherwise.
    pub fn assert_has_unique_ids(constraints: &mut [ConstraintCircuit<II>]) {
        let mut ids: HashMap<usize, ConstraintCircuit<II>> = HashMap::new();

        for circuit in constraints.iter_mut() {
            circuit.inner_has_unique_ids(&mut ids);
        }

        for circuit in constraints.iter_mut() {
            circuit.reset_visited_counters();
        }
    }

    /// Return max degree after evaluating the circuit with an input of specified degree
    pub fn symbolic_degree_bound(
        &self,
        max_base_degrees: &[Degree],
        max_ext_degrees: &[Degree],
    ) -> Degree {
        match &self.expression {
            BinaryOperation(binop, lhs, rhs) => {
                let lhs_degree = lhs
                    .borrow()
                    .symbolic_degree_bound(max_base_degrees, max_ext_degrees);
                let rhs_degree = rhs
                    .borrow()
                    .symbolic_degree_bound(max_base_degrees, max_ext_degrees);
                match binop {
                    BinOp::Add | BinOp::Sub => cmp::max(lhs_degree, rhs_degree),
                    BinOp::Mul => {
                        // If either operand is zero, product is zero so degree is -1
                        if lhs_degree == -1 || rhs_degree == -1 {
                            -1
                        } else {
                            lhs_degree + rhs_degree
                        }
                    }
                }
            }
            Input(input) => match input.is_base_table_column() {
                true => max_base_degrees[input.base_col_index()],
                false => max_ext_degrees[input.ext_col_index()],
            },
            XConstant(xfe) => {
                if xfe.is_zero() {
                    -1
                } else {
                    0
                }
            }
            BConstant(bfe) => {
                if bfe.is_zero() {
                    -1
                } else {
                    0
                }
            }
            Challenge(_) => 0,
        }
    }

    /// Return degree of the multivariate polynomial represented by this circuit
    pub fn degree(&self) -> Degree {
        match &self.expression {
            BinaryOperation(binop, lhs, rhs) => {
                let lhs_degree = lhs.borrow().degree();
                let rhs_degree = rhs.borrow().degree();
                match binop {
                    BinOp::Add | BinOp::Sub => cmp::max(lhs_degree, rhs_degree),
                    BinOp::Mul => {
                        if lhs_degree == -1 || rhs_degree == -1 {
                            -1
                        } else {
                            lhs_degree + rhs_degree
                        }
                    }
                }
            }
            Input(_) => 1,
            XConstant(xfe) => {
                if xfe.is_zero() {
                    -1
                } else {
                    0
                }
            }
            BConstant(bfe) => {
                if bfe.is_zero() {
                    -1
                } else {
                    0
                }
            }
            Challenge(_) => 0,
        }
    }

    /// Return all visited counters in the subtree
    pub fn get_all_visited_counters(&self) -> Vec<usize> {
        // Maybe this could be solved smarter with dynamic programming
        // but we probably don't need that as our circuits aren't too big.
        match &self.expression {
            // The highest number will always be in a leaf so we only
            // need to check those.
            BinaryOperation(_, lhs, rhs) => {
                let lhs_counters = lhs.as_ref().borrow().get_all_visited_counters();
                let rhs_counters = rhs.as_ref().borrow().get_all_visited_counters();
                let own_counter = self.visited_counter;
                let mut all = vec![vec![own_counter], lhs_counters, rhs_counters].concat();
                all.sort_unstable();
                all.dedup();
                all.reverse();
                all
            }
            _ => vec![self.visited_counter],
        }
    }

    /// Return true if the contained multivariate polynomial consists of only a single term. This
    /// means that it can be pretty-printed without parentheses.
    pub fn print_without_parentheses(&self) -> bool {
        !matches!(&self.expression, BinaryOperation(_, _, _))
    }

    /// Return true if this node represents a constant value of zero, does not catch composite
    /// expressions that will always evaluate to zero.
    pub fn is_zero(&self) -> bool {
        match self.expression {
            BConstant(bfe) => bfe.is_zero(),
            XConstant(xfe) => xfe.is_zero(),
            _ => false,
        }
    }

    /// Return true if this node represents a constant value of one, does not catch composite
    /// expressions that will always evaluate to one.
    pub fn is_one(&self) -> bool {
        match self.expression {
            XConstant(xfe) => xfe.is_one(),
            BConstant(bfe) => bfe.is_one(),
            _ => false,
        }
    }

    /// Return true iff the evaluation value of this node depends on a challenge.
    pub fn is_randomized(&self) -> bool {
        match &self.expression {
            Challenge(_) => true,
            BinaryOperation(_, lhs, rhs) => {
                lhs.as_ref().borrow().is_randomized() || rhs.as_ref().borrow().is_randomized()
            }
            _ => false,
        }
    }

    /// Panics if two nodes evaluate to the same value
    pub fn assert_all_evaluate_different(
        constraints: &[Self],
        challenges: &Challenges,
        base_table: ArrayView2<BFieldElement>,
        ext_table: ArrayView2<XFieldElement>,
    ) {
        let mut evaluated_values = HashMap::default();
        for constraint in constraints.iter() {
            Self::evaluate_and_store_and_assert_unique(
                constraint,
                challenges,
                base_table,
                ext_table,
                &mut evaluated_values,
            );
        }
    }

    /// Return own value and whether own value was seen before. Stores own value in hash map.
    fn evaluate_and_store_and_assert_unique(
        &self,
        challenges: &Challenges,
        base_table: ArrayView2<BFieldElement>,
        ext_table: ArrayView2<XFieldElement>,
        evaluated_values: &mut HashMap<XFieldElement, (usize, ConstraintCircuit<II>)>,
    ) -> XFieldElement {
        // assert_eq!(
        //     self.var_count,
        //     input.len(),
        //     "Input length match circuit's var count"
        // );
        let value = match &self.expression {
            XConstant(xfe) => {
                if self.id == 107 || self.id == 454 {
                    println!("{}: XFE", self.id);
                }
                xfe.to_owned()
            }
            BConstant(bfe) => {
                if self.id == 107 || self.id == 454 {
                    println!("{}: BFE", self.id);
                }
                bfe.lift()
            }
            Input(s) => {
                s.evaluate(base_table, ext_table)
                // if s.is_base_table_row() {
                //     base_input[s.base_row_index()].lift()
                // } else {
                //     ext_input[s.ext_row_index()]
                // }
            }
            Challenge(cid) => challenges.get_challenge(*cid),
            BinaryOperation(binop, lhs, rhs) => {
                let lhs = lhs.as_ref().borrow().evaluate_and_store_and_assert_unique(
                    challenges,
                    base_table,
                    ext_table,
                    evaluated_values,
                );
                let rhs = rhs.as_ref().borrow().evaluate_and_store_and_assert_unique(
                    challenges,
                    base_table,
                    ext_table,
                    evaluated_values,
                );
                match binop {
                    BinOp::Add => lhs + rhs,
                    BinOp::Sub => lhs - rhs,
                    BinOp::Mul => lhs * rhs,
                }
            }
        };

        let self_evaluated_is_unique =
            evaluated_values.insert(value, (self.id.to_owned(), self.clone()));
        if let Some((collided_circuit_id, collided_circuit)) = self_evaluated_is_unique {
            let own_id = self.id.to_owned();
            if collided_circuit_id != self.id {
                panic!(
                    "Circuit ID {collided_circuit_id} and circuit ID {own_id} are not unique. \
                    Collission on:\n \
                    {collided_circuit_id}: {collided_circuit}\n {own_id}: {self}. \
                    Value was {value}",
                );
            }
        }
        value
    }

    pub fn evaluate(
        &self,
        base_table: ArrayView2<BFieldElement>,
        ext_table: ArrayView2<XFieldElement>,
        challenges: &Challenges,
    ) -> XFieldElement {
        match self.clone().expression {
            XConstant(xfe) => xfe,
            BConstant(bfe) => bfe.lift(),
            Input(input) => input.evaluate(base_table, ext_table),
            Challenge(challenge_id) => challenges.get_challenge(challenge_id),
            BinaryOperation(binop, lhs, rhs) => {
                let lhs_value = lhs
                    .as_ref()
                    .borrow()
                    .evaluate(base_table, ext_table, challenges);
                let rhs_value = rhs
                    .as_ref()
                    .borrow()
                    .evaluate(base_table, ext_table, challenges);
                match binop {
                    BinOp::Add => lhs_value + rhs_value,
                    BinOp::Sub => lhs_value - rhs_value,
                    BinOp::Mul => lhs_value * rhs_value,
                }
            }
        }
    }
}

/// Constraint expressions, with context needed to ensure that two equal nodes are not added to
/// the multicircuit.
#[derive(Clone)]
pub struct ConstraintCircuitMonad<II: InputIndicator> {
    pub circuit: Rc<RefCell<ConstraintCircuit<II>>>,
    pub builder: ConstraintCircuitBuilder<II>,
}

impl<II: InputIndicator> Debug for ConstraintCircuitMonad<II> {
    // We cannot derive `Debug` as `all_nodes` contains itself which a derived `Debug` will
    // attempt to print as well, thus leading to infinite recursion.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstraintCircuitMonad")
            .field("id", &self.circuit)
            .field(
                "all_nodes length: ",
                &self.builder.all_nodes.as_ref().borrow().len(),
            )
            .field(
                "id_counter_ref value: ",
                &self.builder.id_counter.as_ref().borrow(),
            )
            .finish()
    }
}

impl<II: InputIndicator> Display for ConstraintCircuitMonad<II> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.circuit.as_ref().borrow())
    }
}

impl<II: InputIndicator> PartialEq for ConstraintCircuitMonad<II> {
    // Equality for the ConstraintCircuitMonad is defined by the circuit, not the
    // other metadata (e.g. ID) that it carries around.
    fn eq(&self, other: &Self) -> bool {
        self.circuit == other.circuit
    }
}

impl<II: InputIndicator> Eq for ConstraintCircuitMonad<II> {}

/// Helper function for binary operations that are used to generate new parent nodes in the
/// multitree that represents the algebraic circuit. Ensures that each newly created node has a
/// unique ID.
fn binop<II: InputIndicator>(
    binop: BinOp,
    lhs: ConstraintCircuitMonad<II>,
    rhs: ConstraintCircuitMonad<II>,
) -> ConstraintCircuitMonad<II> {
    // Get ID for the new node
    let new_index = lhs.builder.id_counter.as_ref().borrow().to_owned();
    let lhs = Rc::new(RefCell::new(lhs));
    let rhs = Rc::new(RefCell::new(rhs));

    let new_node = ConstraintCircuitMonad {
        circuit: Rc::new(RefCell::new(ConstraintCircuit {
            visited_counter: 0,
            expression: BinaryOperation(
                binop,
                Rc::clone(&lhs.as_ref().borrow().circuit),
                Rc::clone(&rhs.as_ref().borrow().circuit),
            ),
            id: new_index,
        })),
        builder: lhs.as_ref().borrow().builder.clone(),
    };

    // check if node already exists
    let contained = lhs
        .as_ref()
        .borrow()
        .builder
        .all_nodes
        .as_ref()
        .borrow()
        .contains(&new_node);
    if contained {
        let ret0 = &lhs.as_ref().borrow();
        let ret1 = ret0.builder.all_nodes.as_ref().borrow();
        let ret2 = &(*ret1.get(&new_node).as_ref().unwrap()).clone();
        return ret2.to_owned();
    }

    // If the operator commutes, check if the inverse node has already been constructed.
    // If it has, return this instead. Do not allow a new one to be built.
    if matches!(binop, BinOp::Add | BinOp::Mul) {
        let new_node_inverted = ConstraintCircuitMonad {
            circuit: Rc::new(RefCell::new(ConstraintCircuit {
                visited_counter: 0,
                expression: BinaryOperation(
                    binop,
                    // Switch rhs and lhs for symmetric operators to check membership in hash set
                    Rc::clone(&rhs.as_ref().borrow().circuit),
                    Rc::clone(&lhs.as_ref().borrow().circuit),
                ),
                id: new_index,
            })),
            builder: lhs.as_ref().borrow().builder.clone(),
        };

        // check if node already exists
        let inverted_contained = lhs
            .as_ref()
            .borrow()
            .builder
            .all_nodes
            .as_ref()
            .borrow()
            .contains(&new_node_inverted);
        if inverted_contained {
            let ret0 = &lhs.as_ref().borrow();
            let ret1 = ret0.builder.all_nodes.as_ref().borrow();
            let ret2 = &(*ret1.get(&new_node_inverted).as_ref().unwrap()).clone();
            return ret2.to_owned();
        }
    }

    // Increment counter index
    *lhs.as_ref()
        .borrow()
        .builder
        .id_counter
        .as_ref()
        .borrow_mut() = new_index + 1;

    // Store new node in HashSet
    let inserted_value_was_new = new_node
        .builder
        .all_nodes
        .as_ref()
        .borrow_mut()
        .insert(new_node.clone());
    assert!(inserted_value_was_new, "Binop-created value must be new");

    new_node
}

impl<II: InputIndicator> Add for ConstraintCircuitMonad<II> {
    type Output = ConstraintCircuitMonad<II>;

    fn add(self, rhs: Self) -> Self::Output {
        binop(BinOp::Add, self, rhs)
    }
}

impl<II: InputIndicator> Sub for ConstraintCircuitMonad<II> {
    type Output = ConstraintCircuitMonad<II>;

    fn sub(self, rhs: Self) -> Self::Output {
        binop(BinOp::Sub, self, rhs)
    }
}

impl<II: InputIndicator> Mul for ConstraintCircuitMonad<II> {
    type Output = ConstraintCircuitMonad<II>;

    fn mul(self, rhs: Self) -> Self::Output {
        binop(BinOp::Mul, self, rhs)
    }
}

/// This will panic if the iterator is empty because the neutral element needs a unique ID, and
/// we have no way of getting that here.
impl<II: InputIndicator> Sum for ConstraintCircuitMonad<II> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|accum, item| accum + item)
            .expect("ConstraintCircuitMonad Iterator was empty")
    }
}

impl<II: InputIndicator> ConstraintCircuitMonad<II> {
    /// Unwrap a ConstraintCircuitMonad to reveal its inner ConstraintCircuit
    pub fn consume(self) -> ConstraintCircuit<II> {
        self.circuit.try_borrow().unwrap().to_owned()
    }

    pub fn max_id(&self) -> usize {
        let max_from_hash_map = self
            .builder
            .all_nodes
            .as_ref()
            .borrow()
            .iter()
            .map(|x| x.circuit.as_ref().borrow().id)
            .max()
            .unwrap();

        let id_ref_value = *self.builder.id_counter.borrow();
        assert_eq!(id_ref_value - 1, max_from_hash_map);
        max_from_hash_map
    }

    fn replace_references(&self, old_id: usize, new: Rc<RefCell<ConstraintCircuit<II>>>) {
        for node in self.builder.all_nodes.as_ref().borrow().clone().into_iter() {
            if node.circuit.as_ref().borrow().id == old_id {
                continue;
            }

            if let BinaryOperation(_, ref mut lhs, ref mut rhs) =
                node.circuit.as_ref().borrow_mut().expression
            {
                if lhs.as_ref().borrow().id == old_id {
                    *lhs = new.clone();
                }
                if rhs.as_ref().borrow().id == old_id {
                    *rhs = new.clone();
                }
            }
        }
    }

    fn find_equivalent_expression(&self) -> Option<Rc<RefCell<ConstraintCircuit<II>>>> {
        if let BinaryOperation(binop, lhs, rhs) = &self.circuit.as_ref().borrow().expression {
            // a + 0 = a ∧ a - 0 = a
            if matches!(binop, BinOp::Add | BinOp::Sub) && rhs.borrow().is_zero() {
                return Some(Rc::clone(lhs));
            }

            // 0 + a = a
            if *binop == BinOp::Add && lhs.borrow().is_zero() {
                return Some(Rc::clone(rhs));
            }

            if matches!(binop, BinOp::Mul) {
                // a * 1 = a
                if rhs.borrow().is_one() {
                    return Some(Rc::clone(lhs));
                }

                // 1 * a = a
                if lhs.borrow().is_one() {
                    return Some(Rc::clone(rhs));
                }

                // 0 * a = 0
                if lhs.borrow().is_zero() {
                    return Some(Rc::clone(lhs));
                }

                // a * 0 = 0
                if rhs.borrow().is_zero() {
                    return Some(Rc::clone(rhs));
                }
            }

            // if left and right hand sides are both constants
            if let XConstant(lhs_xfe) = lhs.borrow().expression {
                if let XConstant(rhs_xfe) = rhs.borrow().expression {
                    return match binop {
                        BinOp::Add => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(XConstant(lhs_xfe + rhs_xfe))
                                .consume(),
                        ))),
                        BinOp::Sub => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(XConstant(lhs_xfe - rhs_xfe))
                                .consume(),
                        ))),
                        BinOp::Mul => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(XConstant(lhs_xfe * rhs_xfe))
                                .consume(),
                        ))),
                    };
                }

                if let BConstant(rhs_bfe) = rhs.borrow().expression {
                    return match binop {
                        BinOp::Add => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(XConstant(lhs_xfe + rhs_bfe.lift()))
                                .consume(),
                        ))),
                        BinOp::Sub => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(XConstant(lhs_xfe - rhs_bfe.lift()))
                                .consume(),
                        ))),
                        BinOp::Mul => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(XConstant(lhs_xfe * rhs_bfe))
                                .consume(),
                        ))),
                    };
                }
            }

            if let BConstant(lhs_bfe) = lhs.borrow().expression {
                if let XConstant(rhs_xfe) = rhs.borrow().expression {
                    return match binop {
                        BinOp::Add => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(XConstant(lhs_bfe.lift() + rhs_xfe))
                                .consume(),
                        ))),
                        BinOp::Sub => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(XConstant(lhs_bfe.lift() - rhs_xfe))
                                .consume(),
                        ))),
                        BinOp::Mul => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(XConstant(rhs_xfe * lhs_bfe))
                                .consume(),
                        ))),
                    };
                }

                if let BConstant(rhs_bfe) = rhs.borrow().expression {
                    return match binop {
                        BinOp::Add => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(BConstant(lhs_bfe + rhs_bfe))
                                .consume(),
                        ))),
                        BinOp::Sub => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(BConstant(lhs_bfe - rhs_bfe))
                                .consume(),
                        ))),
                        BinOp::Mul => Some(Rc::new(RefCell::new(
                            self.builder
                                .make_leaf(BConstant(lhs_bfe * lhs_bfe))
                                .consume(),
                        ))),
                    };
                }
            }
            return None;
        }
        None
    }

    /// Apply constant folding to simplify the (sub)tree.
    /// If the subtree is a leaf (terminal), no change.
    /// If the subtree is a binary operation on:
    ///
    ///  - one constant x one constant   => fold
    ///  - one constant x one expr       => can't
    ///  - one expr x one constant       => can't
    ///  - one expr x one expr           => can't
    ///
    /// This operation mutates self and returns true if a change was
    /// applied anywhere in the tree.
    fn constant_fold_inner(&mut self) -> (bool, Option<Rc<RefCell<ConstraintCircuit<II>>>>) {
        let mut change_tracker = false;
        let self_expr = self.circuit.as_ref().borrow().expression.clone();
        if let BinaryOperation(_, lhs, rhs) = &self_expr {
            let mut lhs_as_monadic_value = ConstraintCircuitMonad {
                circuit: lhs.clone(),
                builder: self.builder.clone(),
            };
            let (change_in_lhs, _) = lhs_as_monadic_value.constant_fold_inner();
            change_tracker |= change_in_lhs;
            let mut rhs_as_monadic_value = ConstraintCircuitMonad {
                circuit: rhs.clone(),
                builder: self.builder.clone(),
            };
            let (change_in_rhs, _) = rhs_as_monadic_value.constant_fold_inner();
            change_tracker |= change_in_rhs;
        }

        let equivalent_circuit = self.find_equivalent_expression();
        change_tracker |= equivalent_circuit.is_some();

        if equivalent_circuit.is_some() {
            let equivalent_circuit = equivalent_circuit.as_ref().unwrap().clone();
            let id_of_node_to_be_deleted = self.circuit.borrow().id;
            self.replace_references(id_of_node_to_be_deleted, equivalent_circuit);
            self.builder.all_nodes.as_ref().borrow_mut().remove(self);
        }

        (change_tracker, equivalent_circuit)
    }

    /// Reduce size of multitree by simplifying constant expressions such as `1 * MPol(_,_)`
    pub fn constant_folding(circuits: &mut [ConstraintCircuitMonad<II>]) {
        for circuit in circuits.iter_mut() {
            let mut mutated = true;
            while mutated {
                let (mutated_inner, maybe_new_root) = circuit.constant_fold_inner();
                mutated = mutated_inner;
                if let Some(new_root) = maybe_new_root {
                    *circuit = ConstraintCircuitMonad {
                        circuit: new_root,
                        builder: circuit.builder.clone(),
                    };
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
/// Helper struct to construct new leaf nodes in the circuit multitree. Ensures that each newly
/// created node gets a unique ID.
pub struct ConstraintCircuitBuilder<II: InputIndicator> {
    id_counter: Rc<RefCell<usize>>,
    all_nodes: Rc<RefCell<HashSet<ConstraintCircuitMonad<II>>>>,
}

impl<II: InputIndicator> Default for ConstraintCircuitBuilder<II> {
    fn default() -> Self {
        Self::new()
    }
}

impl<II: InputIndicator> ConstraintCircuitBuilder<II> {
    pub fn new() -> Self {
        Self {
            id_counter: Rc::new(RefCell::new(0)),
            all_nodes: Rc::new(RefCell::new(HashSet::default())),
        }
    }

    pub fn get_node_by_id(&self, id: usize) -> Option<ConstraintCircuitMonad<II>> {
        for node in self.all_nodes.as_ref().borrow().iter() {
            if node.circuit.as_ref().borrow().id == id {
                return Some(node.clone());
            }
        }
        None
    }

    /// Create constant leaf node.
    pub fn x_constant(&self, xfe: XFieldElement) -> ConstraintCircuitMonad<II> {
        let expression = XConstant(xfe);
        self.make_leaf(expression)
    }

    /// Create constant leaf node.
    pub fn b_constant(&self, bfe: BFieldElement) -> ConstraintCircuitMonad<II> {
        let expression = BConstant(bfe);
        self.make_leaf(expression)
    }

    /// Create deterministic input leaf node.
    pub fn input(&self, input: II) -> ConstraintCircuitMonad<II> {
        let expression = Input(input);
        self.make_leaf(expression)
    }

    /// Create challenge leaf node.
    pub fn challenge(&self, challenge_id: ChallengeId) -> ConstraintCircuitMonad<II> {
        let expression = Challenge(challenge_id);
        self.make_leaf(expression)
    }

    fn make_leaf(&self, mut expression: CircuitExpression<II>) -> ConstraintCircuitMonad<II> {
        // Don't generate an X field leaf if it can be expressed as a B field leaf
        if let XConstant(xfe) = expression {
            if let Some(bfe) = xfe.unlift() {
                expression = BConstant(bfe);
            }
        }

        let new_id = self.id_counter.as_ref().borrow().to_owned();
        let new_node = ConstraintCircuitMonad {
            circuit: Rc::new(RefCell::new(ConstraintCircuit {
                visited_counter: 0usize,
                expression,
                id: new_id,
            })),
            builder: self.clone(),
        };

        // Check if node already exists, return the existing one if it does
        let contained = self.all_nodes.as_ref().borrow().contains(&new_node);
        if contained {
            let ret0 = &self.all_nodes.as_ref().borrow();
            let ret1 = &(*ret0.get(&new_node).as_ref().unwrap()).clone();
            return ret1.to_owned();
        }

        // If node did not already exist, increment counter and insert node into hash set
        *self.id_counter.as_ref().borrow_mut() = new_id + 1;
        self.all_nodes
            .as_ref()
            .borrow_mut()
            .insert(new_node.clone());

        new_node
    }
}

#[cfg(test)]
mod constraint_circuit_tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    use ndarray::Array2;
    use rand::random;
    use rand::thread_rng;
    use rand::RngCore;
    use twenty_first::shared_math::other::random_elements;

    use crate::table::cascade_table::ExtCascadeTable;
    use crate::table::challenges::ChallengeId::U32Indeterminate;
    use crate::table::challenges::Challenges;
    use crate::table::hash_table::ExtHashTable;
    use crate::table::jump_stack_table::ExtJumpStackTable;
    use crate::table::lookup_table::ExtLookupTable;
    use crate::table::master_table;
    use crate::table::op_stack_table::ExtOpStackTable;
    use crate::table::processor_table::ExtProcessorTable;
    use crate::table::program_table::ExtProgramTable;
    use crate::table::ram_table::ExtRamTable;
    use crate::table::u32_table::ExtU32Table;

    use super::*;

    fn node_counter_inner<II: InputIndicator>(
        constraint: &mut ConstraintCircuit<II>,
        counter: &mut usize,
    ) {
        if constraint.visited_counter == 0 {
            *counter += 1;
            constraint.visited_counter = 1;

            if let BinaryOperation(_, lhs, rhs) = &constraint.expression {
                node_counter_inner(&mut lhs.as_ref().borrow_mut(), counter);
                node_counter_inner(&mut rhs.as_ref().borrow_mut(), counter);
            }
        }
    }

    /// Count the total number of nodes in call constraints
    fn node_counter<II: InputIndicator>(constraints: &mut [ConstraintCircuit<II>]) -> usize {
        let mut counter = 0;

        for constraint in constraints.iter_mut() {
            node_counter_inner(constraint, &mut counter);
        }

        for constraint in constraints.iter_mut() {
            ConstraintCircuit::reset_visited_counters(constraint);
        }

        counter
    }

    fn random_circuit_builder() -> (
        ConstraintCircuitMonad<DualRowIndicator>,
        ConstraintCircuitBuilder<DualRowIndicator>,
    ) {
        let mut rng = thread_rng();
        let num_base_columns = 50;
        let num_ext_columns = 40;
        let var_count = 2 * (num_base_columns + num_ext_columns);
        let circuit_builder = ConstraintCircuitBuilder::new();
        let b_constants: Vec<BFieldElement> = random_elements(var_count);
        let x_constants: Vec<XFieldElement> = random_elements(var_count);
        let circuit_input =
            DualRowIndicator::NextBaseRow(rng.next_u64() as usize % num_base_columns);
        let mut ret_circuit = circuit_builder.input(circuit_input);
        for _ in 0..100 {
            let circuit = match rng.next_u64() % 6 {
                0 => {
                    // p(x, y, z) = x
                    circuit_builder.input(DualRowIndicator::CurrentBaseRow(
                        rng.next_u64() as usize % num_base_columns,
                    ))
                }
                1 => {
                    // p(x, y, z) = xfe
                    circuit_builder.x_constant(x_constants[rng.next_u64() as usize % var_count])
                }
                2 => {
                    // p(x, y, z) = rand_i
                    circuit_builder.challenge(U32Indeterminate)
                }
                3 => {
                    // p(x, y, z) = 0
                    circuit_builder.x_constant(XFieldElement::zero())
                }
                4 => {
                    // p(x, y, z) = bfe
                    circuit_builder.b_constant(b_constants[rng.next_u64() as usize % var_count])
                }
                5 => {
                    // p(x, y, z) = rand_i * x
                    let input_value =
                        DualRowIndicator::CurrentExtRow(rng.next_u64() as usize % num_ext_columns);
                    let challenge_id = U32Indeterminate;
                    circuit_builder.input(input_value) * circuit_builder.challenge(challenge_id)
                }
                _ => unreachable!(),
            };
            match rng.next_u32() % 3 {
                0 => ret_circuit = ret_circuit * circuit,
                1 => ret_circuit = ret_circuit + circuit,
                2 => ret_circuit = ret_circuit - circuit,
                _ => unreachable!(),
            }
        }

        (ret_circuit, circuit_builder)
    }

    // Make a deep copy of a Multicircuit and return it as a ConstraintCircuitMonad
    fn deep_copy_inner<II: InputIndicator>(
        val: &ConstraintCircuit<II>,
        builder: &mut ConstraintCircuitBuilder<II>,
    ) -> ConstraintCircuitMonad<II> {
        match &val.expression {
            BinaryOperation(op, lhs, rhs) => {
                let lhs_ref = deep_copy_inner(&lhs.as_ref().borrow(), builder);
                let rhs_ref = deep_copy_inner(&rhs.as_ref().borrow(), builder);
                binop(*op, lhs_ref, rhs_ref)
            }
            XConstant(xfe) => builder.x_constant(*xfe),
            BConstant(bfe) => builder.b_constant(*bfe),
            Input(input_index) => builder.input(*input_index),
            Challenge(challenge_id) => builder.challenge(*challenge_id),
        }
    }

    fn deep_copy<II: InputIndicator>(val: &ConstraintCircuit<II>) -> ConstraintCircuitMonad<II> {
        let mut builder = ConstraintCircuitBuilder::new();
        deep_copy_inner(val, &mut builder)
    }

    #[test]
    fn equality_and_hash_agree_test() {
        // The Multicircuits are put into a hash set. Hence, it is important that `Eq` and `Hash`
        // agree whether two nodes are equal: k1 == k2 => h(k1) == h(k2)
        for _ in 0..100 {
            let (circuit, circuit_builder) = random_circuit_builder();
            let mut hasher0 = DefaultHasher::new();
            circuit.hash(&mut hasher0);
            let hash0 = hasher0.finish();
            assert_eq!(circuit, circuit);

            let zero = circuit_builder.x_constant(0.into());
            let same_circuit = circuit.clone() + zero;
            let mut hasher1 = DefaultHasher::new();
            same_circuit.hash(&mut hasher1);
            let hash1 = hasher1.finish();
            let eq_eq = circuit == same_circuit;
            let hash_eq = hash0 == hash1;

            assert_eq!(eq_eq, hash_eq);
        }
    }

    #[test]
    fn multi_circuit_hash_is_unchanged_by_meta_data_test() {
        // From https://doc.rust-lang.org/std/collections/struct.HashSet.html
        // "It is a logic error for a key to be modified in such a way that the key’s hash, as
        // determined by the Hash trait, or its equality, as determined by the Eq trait, changes
        // while it is in the map. This is normally only possible through Cell, RefCell, global
        // state, I/O, or unsafe code. The behavior resulting from such a logic error is not
        // specified, but will be encapsulated to the HashSet that observed the logic error and not
        // result in undefined behavior. This could include panics, incorrect results, aborts,
        // memory leaks, and non-termination."
        // This means that the hash of a node may not depend on: `visited_counter`, `counter`,
        // `id_counter_ref`, or `all_nodes`. The reason for this constraint is that `all_nodes`
        // contains the digest of all nodes in the multi tree.
        let (circuit, _circuit_builder) = random_circuit_builder();
        let mut hasher0 = DefaultHasher::new();
        circuit.hash(&mut hasher0);
        let digest_prior = hasher0.finish();

        // Increase visited counter and verify digest is unchanged
        circuit.circuit.as_ref().borrow_mut().traverse_single();
        let mut hasher1 = DefaultHasher::new();
        circuit.hash(&mut hasher1);
        let digest_after = hasher1.finish();
        assert_eq!(
            digest_prior, digest_after,
            "Digest must be unchanged by traversal"
        );

        // id counter and verify digest is unchanged
        let _dummy = circuit.clone() + circuit.clone();
        let mut hasher2 = DefaultHasher::new();
        circuit.hash(&mut hasher2);
        let digest_after2 = hasher2.finish();
        assert_eq!(
            digest_prior, digest_after2,
            "Digest must be unchanged by Id counter increase"
        );
    }

    #[test]
    fn circuit_equality_check_and_constant_folding_test() {
        let circuit_builder: ConstraintCircuitBuilder<DualRowIndicator> =
            ConstraintCircuitBuilder::new();
        let var_0 = circuit_builder.input(DualRowIndicator::CurrentBaseRow(0));
        let var_4 = circuit_builder.input(DualRowIndicator::NextBaseRow(4));
        let four = circuit_builder.x_constant(4.into());
        let one = circuit_builder.x_constant(1.into());
        let zero = circuit_builder.x_constant(0.into());

        assert_ne!(var_0, var_4);
        assert_ne!(var_0, four);
        assert_ne!(one, four);
        assert_ne!(one, zero);
        assert_ne!(zero, one);

        // Verify that constant folding can handle a = a * 1
        let var_0_copy_0 = deep_copy(&var_0.circuit.as_ref().borrow());
        let var_0_mul_one_0 = var_0_copy_0.clone() * one.clone();
        assert_ne!(var_0_copy_0, var_0_mul_one_0);
        let mut circuits = [var_0_copy_0, var_0_mul_one_0];
        ConstraintCircuitMonad::constant_folding(&mut circuits);
        assert_eq!(
            circuits[0], circuits[1],
            "{} != {}",
            circuits[0], circuits[1]
        );
        assert_eq!(
            circuits[1], circuits[0],
            "{} != {}",
            circuits[1], circuits[0]
        );

        // Verify that constant folding can handle a = 1 * a
        let var_0_copy_1 = deep_copy(&var_0.circuit.as_ref().borrow());
        let var_0_one_mul_1 = one.clone() * var_0_copy_1.clone();
        assert_ne!(var_0_copy_1, var_0_one_mul_1);
        let mut circuits = [var_0_copy_1, var_0_one_mul_1];
        ConstraintCircuitMonad::constant_folding(&mut circuits);
        assert_eq!(
            circuits[0], circuits[1],
            "{} != {}",
            circuits[0], circuits[1]
        );
        assert_eq!(
            circuits[1], circuits[0],
            "{} != {}",
            circuits[1], circuits[0]
        );

        // Verify that constant folding can handle a = 1 * a * 1
        let var_0_copy_2 = deep_copy(&var_0.circuit.as_ref().borrow());
        let var_0_one_mul_2 = one.clone() * var_0_copy_2.clone() * one;
        assert_ne!(var_0_copy_2, var_0_one_mul_2);
        let mut circuits = [var_0_copy_2, var_0_one_mul_2];
        ConstraintCircuitMonad::constant_folding(&mut circuits);
        assert_eq!(
            circuits[0], circuits[1],
            "{} != {}",
            circuits[0], circuits[1]
        );
        assert_eq!(
            circuits[1], circuits[0],
            "{} != {}",
            circuits[1], circuits[0]
        );

        // Verify that constant folding handles a + 0 = a
        let var_0_copy_3 = deep_copy(&var_0.circuit.as_ref().borrow());
        let var_0_plus_zero_3 = var_0_copy_3.clone() + zero.clone();
        assert_ne!(var_0_copy_3, var_0_plus_zero_3);
        let mut circuits = [var_0_copy_3, var_0_plus_zero_3];
        ConstraintCircuitMonad::constant_folding(&mut circuits);
        assert_eq!(
            circuits[0], circuits[1],
            "{} != {}",
            circuits[0], circuits[1]
        );
        assert_eq!(
            circuits[1], circuits[0],
            "{} != {}",
            circuits[1], circuits[0]
        );

        // Verify that constant folding handles a + (a * 0) = a
        let var_0_copy_4 = deep_copy(&var_0.circuit.as_ref().borrow());
        let var_0_plus_zero_4 = var_0_copy_4.clone() + var_0_copy_4.clone() * zero.clone();
        assert_ne!(var_0_copy_4, var_0_plus_zero_4);
        let mut circuits = [var_0_copy_4, var_0_plus_zero_4];
        ConstraintCircuitMonad::constant_folding(&mut circuits);
        assert_eq!(
            circuits[0], circuits[1],
            "{} != {}",
            circuits[0], circuits[1]
        );
        assert_eq!(
            circuits[1], circuits[0],
            "{} != {}",
            circuits[1], circuits[0]
        );

        // Verify that constant folding does not equate `0 - a` with `a`
        let var_0_copy_5 = deep_copy(&var_0.circuit.as_ref().borrow());
        let zero_minus_var_0 = zero - var_0_copy_5.clone();
        assert_ne!(var_0_copy_5, zero_minus_var_0);
        let mut circuits = [var_0_copy_5, zero_minus_var_0];
        ConstraintCircuitMonad::constant_folding(&mut circuits);
        assert_ne!(
            circuits[0], circuits[1],
            "{} == {}",
            circuits[0], circuits[1]
        );
        assert_ne!(
            circuits[1], circuits[0],
            "{} == {}",
            circuits[1], circuits[0]
        );
    }

    #[test]
    fn constant_folding_pbt() {
        for _ in 0..200 {
            let (circuit, circuit_builder) = random_circuit_builder();
            let one = circuit_builder.x_constant(1.into());
            let zero = circuit_builder.x_constant(0.into());

            // Verify that constant folding can handle a = a * 1
            let copy_0 = deep_copy(&circuit.circuit.as_ref().borrow());
            let copy_0_alt = copy_0.clone() * one.clone();
            assert_ne!(copy_0, copy_0_alt);
            let mut circuits = [copy_0.clone(), copy_0_alt.clone()];
            ConstraintCircuitMonad::constant_folding(&mut circuits);
            assert_eq!(
                circuits[0], circuits[1],
                "{} != {}",
                circuits[0], circuits[1]
            );
            assert_eq!(
                circuits[1], circuits[0],
                "{} != {}",
                circuits[1], circuits[0]
            );

            // Verify that constant folding can handle a = 1 * a
            let copy_1 = deep_copy(&circuit.circuit.as_ref().borrow());
            let copy_1_alt = one.clone() * copy_1.clone();
            assert_ne!(copy_1, copy_1_alt);
            let mut circuits = [copy_1, copy_1_alt];
            ConstraintCircuitMonad::constant_folding(&mut circuits);
            assert_eq!(
                circuits[0], circuits[1],
                "{} != {}",
                circuits[0], circuits[1]
            );
            assert_eq!(
                circuits[1], circuits[0],
                "{} != {}",
                circuits[1], circuits[0]
            );

            // Verify that constant folding can handle a = 1 * a * 1
            let copy_2 = deep_copy(&circuit.circuit.as_ref().borrow());
            let copy_2_alt = one.clone() * copy_2.clone() * one.clone();
            assert_ne!(copy_2, copy_2_alt);
            let mut circuits = [copy_2, copy_2_alt];
            ConstraintCircuitMonad::constant_folding(&mut circuits);
            assert_eq!(
                circuits[0], circuits[1],
                "{} != {}",
                circuits[0], circuits[1]
            );
            assert_eq!(
                circuits[1], circuits[0],
                "{} != {}",
                circuits[1], circuits[0]
            );

            // Verify that constant folding handles a + 0 = a
            let copy_3 = deep_copy(&circuit.circuit.as_ref().borrow());
            let copy_3_alt = copy_3.clone() + zero.clone();
            assert_ne!(copy_3, copy_3_alt);
            let mut circuits = [copy_3, copy_3_alt];
            ConstraintCircuitMonad::constant_folding(&mut circuits);
            assert_eq!(
                circuits[0], circuits[1],
                "{} != {}",
                circuits[0], circuits[1]
            );
            assert_eq!(
                circuits[1], circuits[0],
                "{} != {}",
                circuits[1], circuits[0]
            );

            // Verify that constant folding handles a + (a * 0) = a
            let copy_4 = deep_copy(&circuit.circuit.as_ref().borrow());
            let copy_4_alt = copy_4.clone() + copy_4.clone() * zero.clone();
            assert_ne!(copy_4, copy_4_alt);
            let mut circuits = [copy_4, copy_4_alt];
            ConstraintCircuitMonad::constant_folding(&mut circuits);
            assert_eq!(
                circuits[0], circuits[1],
                "{} != {}",
                circuits[0], circuits[1]
            );
            assert_eq!(
                circuits[1], circuits[0],
                "{} != {}",
                circuits[1], circuits[0]
            );

            // Verify that constant folding handles a + (0 * a) = a
            let copy_5 = deep_copy(&circuit.circuit.as_ref().borrow());
            let copy_5_alt = copy_5.clone() + copy_5.clone() * zero.clone();
            assert_ne!(copy_5, copy_5_alt);
            let mut circuits = [copy_5, copy_5_alt];
            ConstraintCircuitMonad::constant_folding(&mut circuits);
            assert_eq!(
                circuits[0], circuits[1],
                "{} != {}",
                circuits[0], circuits[1]
            );
            assert_eq!(
                circuits[1], circuits[0],
                "{} != {}",
                circuits[1], circuits[0]
            );

            // Verify that constant folding does not equate `0 - a` with `a`
            // But only if `a != 0`
            let copy_6 = deep_copy(&circuit.circuit.as_ref().borrow());
            let zero_minus_copy_6 = zero.clone() - copy_6.clone();
            assert_ne!(copy_6, zero_minus_copy_6);
            let mut circuits = [copy_6, zero_minus_copy_6];
            ConstraintCircuitMonad::constant_folding(&mut circuits);
            let copy_6_is_zero = circuits[0].circuit.as_ref().borrow().is_zero();
            let copy_6_expr = circuits[0].circuit.as_ref().borrow().expression.clone();
            let zero_minus_copy_6_expr = circuits[1].circuit.as_ref().borrow().expression.clone();

            // An X field and a B field leaf will never be equal
            if copy_6_is_zero
                && (matches!(copy_6_expr, CircuitExpression::BConstant(_))
                    && matches!(zero_minus_copy_6_expr, CircuitExpression::BConstant(_))
                    || matches!(copy_6_expr, CircuitExpression::XConstant(_))
                        && matches!(zero_minus_copy_6_expr, CircuitExpression::XConstant(_)))
            {
                assert_eq!(
                    circuits[0], circuits[1],
                    "{} != {}",
                    circuits[0], circuits[1]
                );
                assert_eq!(
                    circuits[1], circuits[0],
                    "{} != {}",
                    circuits[1], circuits[0]
                );
            } else {
                assert_ne!(
                    circuits[0], circuits[1],
                    "{} == {}",
                    circuits[0], circuits[1]
                );
                assert_ne!(
                    circuits[1], circuits[0],
                    "{} == {}",
                    circuits[1], circuits[0]
                );
            }

            // Verify that constant folding handles a - 0 = a
            let copy_7 = deep_copy(&circuit.circuit.as_ref().borrow());
            let copy_7_alt = copy_7.clone() - zero.clone();
            assert_ne!(copy_7, copy_7_alt);
            let mut circuits = [copy_7, copy_7_alt];
            ConstraintCircuitMonad::constant_folding(&mut circuits);
            assert_eq!(
                circuits[0], circuits[1],
                "{} != {}",
                circuits[0], circuits[1]
            );
            assert_eq!(
                circuits[1], circuits[0],
                "{} != {}",
                circuits[1], circuits[0]
            );
        }
    }

    fn table_constraints_prop<II: InputIndicator>(
        mut constraints: Vec<ConstraintCircuit<II>>,
        challenges: &Challenges,
        table_name: &str,
    ) {
        ConstraintCircuit::assert_has_unique_ids(&mut constraints);

        // Verify that all nodes evaluate to a unique value when given a randomized input.
        // If this is not the case two nodes that are not equal evaluate to the same value.
        // let input: Vec<XFieldElement> = random_elements(constraints[0].var_count);
        // let base_input: Vec<BFieldElement> = random_elements(master_table::NUM_BASE_COLUMNS);
        // let ext_input: Vec<XFieldElement> = random_elements(master_table::NUM_EXT_COLUMNS);
        let base_table = Array2::from_shape_simple_fn(
            [2, master_table::NUM_BASE_COLUMNS],
            random::<BFieldElement>,
        );
        let ext_table = Array2::from_shape_simple_fn(
            [2, master_table::NUM_EXT_COLUMNS],
            random::<XFieldElement>,
        );
        ConstraintCircuit::assert_all_evaluate_different(
            &constraints,
            challenges,
            base_table.view(),
            ext_table.view(),
        );

        println!("nodes in {table_name}: {}", node_counter(&mut constraints));
        let circuit_degree = constraints.iter().map(|c| c.degree()).max().unwrap_or(-1);
        println!("Max degree constraint for {table_name} table: {circuit_degree}");
    }

    #[test]
    fn constant_folding_processor_table_test() {
        let challenges = Challenges::placeholder(&[], &[]);
        let constraint_circuits = ExtProcessorTable::ext_initial_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "processor initial");
        let constraint_circuits = ExtProcessorTable::ext_consistency_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "processor consistency");
        let constraint_circuits = ExtProcessorTable::ext_transition_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "processor transition");
        let constraint_circuits = ExtProcessorTable::ext_terminal_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "processor terminal");
    }

    #[test]
    fn constant_folding_program_table_test() {
        let challenges = Challenges::placeholder(&[], &[]);
        let constraint_circuits = ExtProgramTable::ext_initial_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "program initial");
        let constraint_circuits = ExtProgramTable::ext_consistency_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "program consistency");
        let constraint_circuits = ExtProgramTable::ext_transition_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "program transition");
        let constraint_circuits = ExtProgramTable::ext_terminal_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "program terminal");
    }

    #[test]
    fn constant_folding_jump_stack_table_test() {
        let challenges = Challenges::placeholder(&[], &[]);
        let constraint_circuits = ExtJumpStackTable::ext_initial_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "jump stack initial");
        let constraint_circuits = ExtJumpStackTable::ext_consistency_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "jump stack consistency");
        let constraint_circuits = ExtJumpStackTable::ext_transition_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "jump stack transition");
        let constraint_circuits = ExtJumpStackTable::ext_terminal_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "jump stack terminal");
    }

    #[test]
    fn constant_folding_op_stack_table_test() {
        let challenges = Challenges::placeholder(&[], &[]);
        let constraint_circuits = ExtOpStackTable::ext_initial_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "op stack initial");
        let constraint_circuits = ExtOpStackTable::ext_consistency_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "op stack consistency");
        let constraint_circuits = ExtOpStackTable::ext_transition_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "op stack transition");
        let constraint_circuits = ExtOpStackTable::ext_terminal_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "op stack terminal");
    }

    #[test]
    fn constant_folding_ram_table_test() {
        let challenges = Challenges::placeholder(&[], &[]);
        let constraint_circuits = ExtRamTable::ext_initial_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "ram initial");
        let constraint_circuits = ExtRamTable::ext_consistency_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "ram consistency");
        let constraint_circuits = ExtRamTable::ext_transition_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "ram transition");
        let constraint_circuits = ExtRamTable::ext_terminal_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "ram terminal");
    }

    #[test]
    fn constant_folding_hash_table_test() {
        let challenges = Challenges::placeholder(&[], &[]);
        let constraint_circuits = ExtHashTable::ext_initial_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "hash initial");
        let constraint_circuits = ExtHashTable::ext_consistency_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "hash consistency");
        let constraint_circuits = ExtHashTable::ext_transition_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "hash transition");
        let constraint_circuits = ExtHashTable::ext_terminal_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "hash terminal");
    }

    #[test]
    fn constant_folding_u32_table_test() {
        let challenges = Challenges::placeholder(&[], &[]);
        let constraint_circuits = ExtU32Table::ext_initial_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "u32 initial");
        let constraint_circuits = ExtU32Table::ext_consistency_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "u32 consistency");
        let constraint_circuits = ExtU32Table::ext_transition_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "u32 transition");
        let constraint_circuits = ExtU32Table::ext_terminal_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "u32 terminal");
    }

    #[test]
    fn constant_folding_cascade_table_test() {
        let challenges = Challenges::placeholder(&[], &[]);
        let constraint_circuits = ExtCascadeTable::ext_initial_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "cascade initial");
        let constraint_circuits = ExtCascadeTable::ext_consistency_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "cascade consistency");
        let constraint_circuits = ExtCascadeTable::ext_transition_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "cascade transition");
        let constraint_circuits = ExtCascadeTable::ext_terminal_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "cascade terminal");
    }

    #[test]
    fn constant_folding_lookup_table_test() {
        let challenges = Challenges::placeholder(&[], &[]);
        let constraint_circuits = ExtLookupTable::ext_initial_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "lookup initial");
        let constraint_circuits = ExtLookupTable::ext_consistency_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "lookup consistency");
        let constraint_circuits = ExtLookupTable::ext_transition_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "lookup transition");
        let constraint_circuits = ExtLookupTable::ext_terminal_constraints_as_circuits();
        table_constraints_prop(constraint_circuits, &challenges, "lookup terminal");
    }
}
