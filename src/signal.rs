
use std::f32;
use std::iter::*;
use sink;
use std::ops::{Sub, Add};
use src;
use num_complex::Complex;

pub fn diff(lhs: &[f32], rhs: &[f32]) -> Vec<f32> {
    element_wise(lhs, rhs, |&a, &b| a - b)
}

pub fn add(lhs: &[f32], rhs: &[f32]) -> Vec<f32> {
    element_wise(lhs, rhs, |&a, &b| a + b )
}

pub fn element_wise<F>(lhs: &[f32], rhs: &[f32], op: F) -> Vec<f32> where F: Fn(&f32, &f32) -> f32 {
    let longest_length = lhs.len().max(rhs.len());
    let lhs_iter = lhs.iter().chain(repeat(&0.0).take(longest_length - lhs.len()));
    let rhs_iter = rhs.iter().chain(repeat(&0.0).take(longest_length - rhs.len()));
    lhs_iter.zip(rhs_iter).map(|(a, b)| op(a, b)).collect()
}

/** Wrapper for element-wise ops on sequences */
pub struct Seq<'a>(pub &'a [f32]);

impl <'a> Sub for Seq<'a> {
    type Output = Vec<f32>;
    fn sub(self, rhs: Seq<'a>) -> Vec<f32> {
        diff(self.0, rhs.0)
    }
}

impl <'a> Add for Seq<'a> {
    type Output = Vec<f32>;
    fn add(self, rhs: Seq<'a>) -> Vec<f32> {
        add(self.0, rhs.0)
    }
}

/** Scales to -1.0,1.0 */
pub fn bound(ss: &[f32]) -> Vec<f32> {
    let smin = sink::min(ss);
    let smax = sink::max(ss);
    let max_mag = smin.abs().max(smax.abs());
    let f = 1.0 / max_mag;
    ss.iter().map(|s| s * f).collect::<Vec<f32>>()
}

pub fn scale(factor: f32, ss: &[f32]) -> Vec<f32> {
    ss.iter().map(|&s| s * factor).collect()
}

pub fn repeater(ss: &[f32], noise_amplitude: f32, attenuation: f32) -> Vec<f32> {
    let noise = src::uniform(ss.len(), -noise_amplitude, noise_amplitude);
    scale(1.0 / attenuation, &add(&scale(attenuation, ss), &noise))
}

pub fn round(ss: &[f32]) -> Vec<f32> {
    ss.iter().map(|&s| s.round()).collect()
}

/// Discrete Fourier Transform -- analysis
pub fn dft_analyze(ss: &[f32]) -> Vec<Complex<f32>> {
    let len = ss.len();
    let mut out = Vec::with_capacity(len);
    for k in 0..len {
        out.push(ss.iter().enumerate().map(|(n, x)| {
            let re = 0.0;
            let im = - ((2.0 * f32::consts::PI * n as f32 * k as f32) / len as f32);
            Complex::new(re, im).exp() * x / len as f32
        }).sum());
    }
    out
}

/// Discrete Fourier Transform -- synthesis
pub fn dft_synthesize(ss: &[Complex<f32>]) -> Vec<Complex<f32>> {
    let len = ss.len();
    let mut out = Vec::with_capacity(len);
    for n in 0..len {
        out.push(ss.iter().enumerate().map(|(k, x)| {
            let re = 0.0;
            let im = (2.0 * f32::consts::PI * k as f32 * n as f32) / len as f32;
            Complex::new(re, im).exp() * x
        }).sum());
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dft() {
        let equals = |x: &f32, y: &Complex<f32>| { let Complex { re, im } = x - y; re.abs() < 0.001 && im.abs() < 0.001 };

        let xs = vec![1.0, 2.0, 3.0, 4.0, 3.0, 2.0, 1.0];
        let analyzed = dft_analyze(&xs);
        let synthesized = dft_synthesize(&analyzed);

        assert!(xs.iter().zip(synthesized.iter()).all(|(x, y)| equals(x, y)))
    }
}