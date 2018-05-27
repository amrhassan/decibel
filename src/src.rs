/**
  * Signal sources
  */

use std::fs::File;
use std::path::Path;
use simplemad::*;
use rand::{random, Closed01};
use mat;
use mat::{Shape, Matrix, Index};
use sink;
use audio::*;
use image;
use image::{GenericImage, Pixel};


pub fn decode_audio(file: &File) -> Result<Audio, String> {
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

pub fn checkered(shape: &Shape) -> Matrix {
    let mut out = mat::zeros(shape);
    for index in shape.indices() {
        let singulared = index.0.iter().map(|ix| ix % 2 == 1).collect::<Vec<bool>>();
        if sink::xor(&singulared) {
            out[&index] = 255.0;
        }
    }
    out
}

pub fn decode_image_greyscale(path: &Path) -> Result<Matrix, String> {
    let img = image::open(path).map_err(|err| format!("{}", err))?;
    let (width, height) = img.dimensions();
    let mut mat = Matrix::new_2d(width as usize, height as usize);

    for (x, y, px) in img.pixels() {
        mat[&Index::of_2d(y as usize, x as usize)] = px.to_luma().data[0] as f32;
    }

    Ok(mat)
}

#[cfg(test)]
mod tests {
    use mat::*;
    use src;

    #[test]
    fn generates_checkered_images() {
        assert_eq!(src::checkered(&Shape(vec![4, 4])).unrolled,
                   vec![0.0, 255.0, 0.0, 255.0,
                        255.0, 0.0, 255.0, 0.0,
                        0.0, 255.0, 0.0, 255.0,
                        255.0, 0.0, 255.0, 0.0]
        )
    }
}
