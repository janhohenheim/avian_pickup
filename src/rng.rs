use crate::prelude::*;
use rand::{Rng, TryRng};
use std::convert::Infallible;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<RngSource>();
}

/// A resource that provides a source of randomness.
/// Will fall back to [`rand::rng()`] if no source is provided.
#[derive(Resource, Default)]
pub struct RngSource(pub Option<Box<dyn Rng + Send + Sync>>);

impl TryRng for RngSource {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        Ok(self.run(|rng| rng.next_u32()))
    }

    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        Ok(self.run(|rng| rng.next_u64()))
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Infallible> {
        self.run(|rng| rng.fill_bytes(dest));
        Ok(())
    }
}

impl RngSource {
    fn run<Out>(&mut self, f: impl FnOnce(&mut dyn Rng) -> Out) -> Out {
        if let Some(rng) = self.0.as_mut() {
            f(rng)
        } else {
            f(&mut rand::rng())
        }
    }
}
