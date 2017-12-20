extern crate autograd as ag;
extern crate petgraph;

use distribution::Distribution;
use var::Variable;

use self::petgraph::Graph;

//use tree::{TreeDict, TreeList};

///
///
///
#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub parent: Option<String>,
    pub named_vars: Vec<String>,     // all variables
    pub deterministics: Vec<String>, // TODO: deterministic variables
    pub free_rvs: Vec<String>,       // stochastic variables
    pub observed_rvs: Vec<String>,   // stochastic + observed variables

    pub graph: Graph<String, String>, // TODO: petgraph
}

///
///
///
#[allow(unused_variables)]
impl Model {
    pub fn new(name: String) -> Self {
        Model {
            name: name,
            parent: None,
            named_vars: Vec::new(),
            deterministics: Vec::new(),
            free_rvs: Vec::new(),
            observed_rvs: Vec::new(),
            graph: Graph::<String, String>::new(),
        }
    }

    pub fn new_with_parent(name: String, parent: String) -> Self {
        Model {
            name: name,
            parent: Some(parent),
            named_vars: Vec::new(),
            deterministics: Vec::new(),
            free_rvs: Vec::new(),
            observed_rvs: Vec::new(),
            graph: Graph::<String, String>::new(),
        }
    }

    pub fn with_parent(&mut self, parent: String) -> &mut Self {
        self.parent = Some(parent);
        self
    }

    // function `var` naming comes from pymc3 model
    // see: https://github.com/pymc-devs/pymc3/blob/391f5fd143b5a963daa869508adf1eaa051c346e/pymc3/model.py#L729
    pub fn var_with_options(
        &mut self,
        name: String,
        dist: &Distribution,
        data: Option<String>,
        total_size: Option<usize>,
    ) -> Variable {
        if data == None {
            println!("data is None");
        }

        if total_size == None {
            println!("total_size is None");
        }

        self.named_vars.push(name.to_string());
        self.free_rvs.push(name.to_string());

        self.graph.add_node(name.to_string());

        Variable::FreeRV(ag::placeholder(&[-1]))
    }

    pub fn var(&mut self, name: String, dist: &Distribution) -> Variable {
        self.var_with_options(name, dist, None, None)
    }

    pub fn deterministic(&mut self, name: String, var: String) {
        unimplemented!();
    }
}
