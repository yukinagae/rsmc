use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TreeList<T> {
    pub parent: Option<Rc<TreeList<T>>>,
    pub values: Vec<T>,
}

impl<T> TreeList<T> {
    pub fn new() -> Self {
        TreeList {
            parent: None,
            values: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }
}

#[derive(Debug)]
pub struct TreeDict<T> {
    pub parent: Option<Rc<TreeDict<T>>>,
    pub values: HashMap<String, T>,
}

impl<T> TreeDict<T> {
    pub fn new() -> Self {
        TreeDict {
            parent: None,
            values: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: T) {
        self.values.insert(key, value);
    }
}
