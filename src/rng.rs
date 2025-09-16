use crate::prelude::*;
use rand::RngCore;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<RngSource>();
}

/// A resource that provides a source of randomness.
/// Will fall back to [`rand::rng()`] if no source is provided.
#[derive(Resource, Default)]
pub struct RngSource(pub Option<Box<dyn RngCore + Send + Sync>>);

impl RngCore for RngSource {
    fn next_u32(&mut self) -> u32 {
        self.run(|rng| rng.next_u32())
    }

    fn next_u64(&mut self) -> u64 {
        self.run(|rng| rng.next_u64())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.run(|rng| rng.fill_bytes(dest))
    }
}

impl RngSource {
    fn run<Out>(&mut self, f: impl FnOnce(&mut dyn RngCore) -> Out) -> Out {
        if let Some(rng) = self.0.as_mut() {
            f(rng)
        } else {
            f(&mut rand::rng())
        }
    }
}
