use super::*;
use markov::Chain;
use rand::Rng;
use std::collections::HashSet;

/// Name generator which generates based on names given in training data.
pub struct NameGen {
    cache: HashSet<String>,
    chain: Chain<String>,
}

impl NameGen {
    /// Maximum tries to generate an unique name.
    const MAX_TRIES: usize = 1000;

    /// Create a new name generator.
    pub fn new() -> NameGen {
        NameGen {
            cache: HashSet::new(),
            chain: Chain::new(),
        }
    }

    /// Train the underlying model using the given name.
    pub fn train(&mut self, name: &str) {
        self.chain.feed_str(name);
    }

    /// Generate a new name.
    /// Attempt to generate a new unique name, running a maximum number of tries before returning none.
    fn generate<R: Rng>(&mut self, _gen: &mut R) -> Result<String, ()> {
        for _ in 0..Self::MAX_TRIES {
            let name = self.chain.generate_str();
            if !self.cache.contains(&name) {
                self.cache.insert(name.clone());
                return Ok(name);
            }
        }
        Err(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_namegen() {
        let data = "elizabeth";
        let mut gen = NameGen::new();
        let mut rng = ChaChaRng::seed_from_u64(42);
        gen.train(data);
        assert_eq!(gen.generate(&mut rng), Ok(String::from(data)));
        assert!(gen.generate(&mut rng).is_err());
    }
}
