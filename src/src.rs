
/**
  * Signal sources
  */

use std::fs::File;
use simplemad::*;
use rand::{random, Closed01};
use signal;

pub struct Audio {
    pub sample_rate: u32,
    pub channels: Vec<Vec<f32>>,
}

impl Audio {
    pub fn decode_file(file: &File) -> Result<Audio, String> {
        let decoder = Decoder::decode(file).expect("Failed to decode");

        let mut channels = Vec::new();
        let mut sr = None;

        for decoding_result in decoder {
            match decoding_result {
                Err(_) => (),
                Ok(mut frame) => {
                    if sr.is_none() {
                        sr = Some(frame.sample_rate);
                    }
                    channels.resize(frame.samples.len(), Vec::new());
                    for (chan, ss) in channels.iter_mut().zip(frame.samples.iter_mut()) {
                        for s in ss {
                            chan.push(s.to_f32())
                        }
                    }
                }
            }
        }

        match sr {
            None => Err("Could not find a sample rate!".to_string()),
            Some(srv) => Ok(Audio {
                sample_rate: srv,
                channels,
            })
        }
    }
}

pub fn uniform(size: usize, lower_bound: f32, upper_bound: f32) -> Vec<f32> {
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        let v = random::<Closed01<f32>>().0;
        let ranged_v = lower_bound + v * (upper_bound - lower_bound);
        vec.push(ranged_v);
    }
    vec
}

pub fn karplus_strong(ss: &[f32], alpha: f32, output_size: usize) -> Vec<f32> {
    let m = ss.len() as f32;
    (0..output_size).map(|n| {
        let nf = n as f32;
        alpha.powf((nf / m).ceil()) * ss[n % ss.len()]
    }).collect()
}
