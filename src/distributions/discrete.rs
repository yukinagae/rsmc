// pub struct Bernoulli {
//     pub p: f64,
//     pub mode: f64,
// }

// impl Distribution for Bernoulli {
//     fn random(&self) -> f64 {
//         // rand::thread_rng().gen_range(self.lower, self.upper)
//         1.0 // TODO: not yet
//     }
// }

// impl Discrete for Bernoulli {
//     type Value = bool;

//     fn mass(&self, x: bool) -> f64 {
//         if x {
//             self.p
//         } else {
//             1.0 - self.p
//         }
//     }
// }
