use rand::Rng;
use rand_core::{Error, impls, RngCore};

struct CountingRng(u64);

impl RngCore for CountingRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

#[test]
fn counting_rng_generates_fixed_sequence() {
    let mut rng = CountingRng(10);
    assert_eq!(11, rng.gen());
    assert_eq!(12, rng.gen());

    assert_eq!(1, rng.gen_range(1, 1000));
    assert_eq!(2, rng.gen_range(2, 1000));
    assert_eq!(3, rng.gen_range(3, 1000));
}