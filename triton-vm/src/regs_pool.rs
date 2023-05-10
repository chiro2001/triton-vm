use twenty_first::shared_math::b_field_element::BFieldElement;

#[derive(Debug, Clone)]
pub struct RegsPool {
    pub regs: Vec<BFieldElement>,
}

impl Default for RegsPool {
    fn default() -> Self {
        Self {
            regs: vec![BFieldElement::default(); 32],
        }
    }
}
