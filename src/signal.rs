
use std::f32;
use std::iter::*;
use sink;
use std::ops::{Sub, Add};
use src;

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