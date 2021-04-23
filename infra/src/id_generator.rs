use rand::{distributions::Standard, prelude::Distribution, Rng};

pub(crate) struct IdGenerator;

impl IdGenerator {
    pub(crate) fn gen<T>() -> T
    where
        Standard: Distribution<T>,
    {
        // TODO 重複しないID生成
        let mut rng = rand::thread_rng();
        rng.gen()
    }
}
