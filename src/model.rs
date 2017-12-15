use distribution::Distribution;

use std::collections::HashMap;

///
///
///
pub struct Model<'a> {
    pub named_vars: HashMap<&'a str, &'a Distribution>,
    //pub deterministics: HashMap<&'a str, &'a Deterministics>
}

///
///
///
impl<'a> Model<'a> {
    pub fn new() -> Self {
        Model {
            named_vars: HashMap::new(),
        }
    }

    // function `var` naming comes from pymc3 model
    // see: https://github.com/pymc-devs/pymc3/blob/391f5fd143b5a963daa869508adf1eaa051c346e/pymc3/model.py#L729
    pub fn var(&mut self, name: &'a str, dist: &'a Distribution) {
        self.named_vars.insert(name, dist);
    }

    // pub fn deterministic(&mut self, name: &'a str, var: Var) -> Var {}
}
