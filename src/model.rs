use distribution::Distribution;

use tree::{TreeDict, TreeList};

///
///
///
pub struct Model<'a> {
    pub parent: Option<&'a Model<'a>>,
    pub named_vars: TreeDict<&'a Distribution>, // all variables
    pub deterministics: Vec<&'a Distribution>,  // TODO: deterministic variables
    pub free_rvs: TreeList<&'a Distribution>,   // stochastic variables
    pub observed_rvs: Vec<&'a Distribution>,    // stochastic + observed variables
}

///c
///
///
impl<'a> Model<'a> {
    pub fn new() -> Self {
        Model {
            parent: None,
            named_vars: TreeDict::new(),
            deterministics: Vec::new(),
            free_rvs: TreeList::new(),
            observed_rvs: Vec::new(),
        }
    }

    pub fn set_parent(&mut self, parent: &'a Model) {
        self.parent = Option::Some(parent);
    }

    // function `var` naming comes from pymc3 model
    // see: https://github.com/pymc-devs/pymc3/blob/391f5fd143b5a963daa869508adf1eaa051c346e/pymc3/model.py#L729
    pub fn var_with_options(
        &mut self,
        name: &'a str,
        dist: &'a Distribution,
        data: Option<String>,
        total_size: Option<usize>,
    ) {
        if data == None {
            println!("data is None");
        }

        if total_size == None {
            println!("total_size is None");
        }

        self.named_vars.insert(name.to_string(), dist);
        self.free_rvs.push(dist);
    }

    pub fn var(&mut self, name: &'a str, dist: &'a Distribution) {
        self.var_with_options(name, dist, None, None);
    }

    // pub fn deterministic(&mut self, name: &'a str, var: Var) -> Var {}
}
