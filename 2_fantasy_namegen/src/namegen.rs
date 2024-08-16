use rand::{rngs::StdRng, SeedableRng};

use crate::{first_name, last_name};

pub struct Namegen {
    rng: StdRng,
}

impl Namegen {
    pub fn new(seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);

        Namegen { rng }
    }

    pub fn first_name(&mut self) -> String {
        first_name(&mut self.rng)
    }

    pub fn last_name(&mut self) -> String {
        last_name(&mut self.rng)
    }
}
