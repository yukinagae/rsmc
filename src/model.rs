use distributions::distribution::Distribution;

use std::cell::RefCell;

///
///
///
pub struct Model {
    dists: RefCell<Vec<Distribution>>,
}

///
///
///
impl Model {
    fn new() -> Self {
        Model {
            dists: RefCell::new(Vec::new()),
        }
    }

    fn add_dist(&self, dist: Distribution) {
        self.dists.borrow_mut().push(dist);
    }
}
