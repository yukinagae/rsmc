use distribution::Distribution;

use std::collections::HashMap;

///
///
///
pub struct Model<'a> {
    pub named_vars: HashMap<&'a str, &'a Distribution>, // all variables
    pub deterministics: Vec<&'a Distribution>,          // TODO: deterministic variables
    pub free_rvs: Vec<&'a Distribution>,                // stochastic variables
    pub observed_rvs: Vec<&'a Distribution>,            // stochastic + observed variables
}

///c
///
///
impl<'a> Model<'a> {
    pub fn new() -> Self {
        Model {
            named_vars: HashMap::new(),
            deterministics: Vec::new(),
            free_rvs: Vec::new(),
            observed_rvs: Vec::new(),
        }
    }

    // function `var` naming comes from pymc3 model
    // see: https://github.com/pymc-devs/pymc3/blob/391f5fd143b5a963daa869508adf1eaa051c346e/pymc3/model.py#L729
    pub fn var(&mut self, name: &'a str, dist: &'a Distribution) {
        self.named_vars.insert(name, dist);
        self.free_rvs.push(dist);
    }

    // pub fn deterministic(&mut self, name: &'a str, var: Var) -> Var {}
}
