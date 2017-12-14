use distributions::distribution::Distribution;

///
///
///
pub struct Model<'a> {
    pub dists: Vec<&'a Distribution>,
}

///
///
///
impl<'a> Model<'a> {
    pub fn new() -> Self {
        Model { dists: Vec::new() }
    }

    pub fn add_dist(&mut self, dist: &'a Distribution) {
        self.dists.push(dist);
    }
}
