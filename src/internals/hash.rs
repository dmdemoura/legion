use core::hash::Hasher;

use super::id::ID;

/// A hasher optimized for hashing component type IDs.
#[derive(Default)]
pub struct ComponentTypeIdHasher(ID);

impl Hasher for ComponentTypeIdHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    #[inline]
    #[cfg(feature = "adven-gba")]
    fn write_u32(&mut self, seed: u32) {
        // This must only be used to hash one value.
        debug_assert_eq!(self.0, 0);
        self.0 = seed;
    }

    #[inline]
    #[cfg(not(feature = "adven-gba"))]
    fn write_u64(&mut self, seed: u64) {
        // This must only be used to hash one value.
        debug_assert_eq!(self.0, 0);
        self.0 = seed;
    }

    fn write(&mut self, _bytes: &[u8]) {
        // This should not be called, only write_u64.
        unimplemented!()
    }
}

/// A hasher optimized for hashing types that are represented as a u64.
#[derive(Default)]
pub struct U64Hasher(u64);

impl Hasher for U64Hasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write_u64(&mut self, seed: u64) {
        // This must only be used to hash one value.
        debug_assert_eq!(self.0, 0);

        let max_prime = 11_400_714_819_323_198_549u64;
        self.0 = max_prime.wrapping_mul(seed);
    }

    fn write(&mut self, _bytes: &[u8]) {
        // This should not be called, only write_u64.
        unimplemented!()
    }
}

/// A hasher optimized for hashing types that are represented as a u32.
#[derive(Default)]
pub struct U32Hasher(u32);

impl Hasher for U32Hasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    #[inline]
    fn write_u32(&mut self, seed: u32) {
        // This must only be used to hash one value.
        debug_assert_eq!(self.0, 0);

        let max_prime = 2_654_404_609u32; //TODO: Is this correct?
        self.0 = max_prime.wrapping_mul(seed);
    }

    fn write(&mut self, _bytes: &[u8]) {
        // This should not be called, only write_u32.
        unimplemented!()
    }
}

#[test]
fn hasher() {
    fn verify<T: 'static + ?Sized>() {
        use core::{any::TypeId, hash::Hash};

        let mut hasher = ComponentTypeIdHasher::default();
        let type_id = TypeId::of::<T>();
        type_id.hash(&mut hasher);
        assert_eq!(hasher.finish(), unsafe {
            core::mem::transmute::<TypeId, u64>(type_id)
        });
    }

    verify::<usize>();
    verify::<()>();
    verify::<str>();
    verify::<&'static str>();
    verify::<[u8; 20]>();
}
