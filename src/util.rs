
use sink::*;
use std::f32;
use signal::*;

pub fn decibel(v: f32) -> f32 {
    10.0 * f32::log10(v)
}

pub fn snr(ss: &[f32], ss_noisy: &[f32]) -> f32 {
    let noise = Seq(ss) - Seq(ss_noisy);
    l2_norm(ss) / l2_norm(&noise)
}
