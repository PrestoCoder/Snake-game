use rand::Rng;

pub struct RandomGenerator;

impl RandomGenerator {
     pub fn new() -> Self {
          Self
     }

     pub fn gen_range(&self, min: u16, max: u16) -> u16 {
          rand::thread_rng().gen_range(min..max)
     }

     pub fn gen_bool(&self, probability: f64) -> bool {
          rand::thread_rng().gen_bool(probability)
     }
}