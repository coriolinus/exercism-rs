extern crate rand;
use rand::{thread_rng, Rng};

mod modular_math;
use modular_math::pow_mod;

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow_mod(g, a, p)
}

pub fn secret(p: u64, public: u64, secret: u64) -> u64 {
    pow_mod(public, secret, p)
}
