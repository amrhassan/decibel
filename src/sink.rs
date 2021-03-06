/**
  * Signal sinks
  */

use std::f32;
use std::collections::VecDeque;
use cpal::*;
use std::thread;
use std::sync::mpsc::channel;
use std::path::Path;
use image;
use mat::{Matrix, Index};
use std::fs::File;
use image::GenericImage;

pub fn min(signal: &[f32]) -> f32 {
    signal.iter().fold(f32::MAX, |acc, &s| if s < acc { s } else { acc })
}

pub fn max(signal: &[f32]) -> f32 {
    signal.iter().fold(f32::MIN, |acc, &s| if s > acc { s } else { acc })
}

pub fn l2_norm(ss: &[f32]) -> f32 {
    ss.iter().map(|&s| s.powi(2)).sum::<f32>().sqrt()
}

pub fn play(signal: &[f32], sample_rate: u32) {
    let format = Format {
        channels: 1,
        sample_rate: SampleRate(sample_rate),
        data_type: SampleFormat::F32,
    };

    let mut audio: VecDeque<f32> = signal.iter().clone().map(|s| *s).collect();

    let event_loop = EventLoop::new();
    let device = default_output_device().expect("Failed to get default output device");
    let stream_id = event_loop.build_output_stream(&device, &format).expect("Failed to build stream");

    event_loop.play_stream(stream_id);
    let (tx, rx) = channel();

    thread::spawn(move || {
        event_loop.run(move |_stream_id, _data_type| {
            match _data_type {
                StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                    for e in buffer.iter_mut() {
                        match audio.pop_front() {
                            Some(s) => *e = s,
                            None => tx.send(()).unwrap_or(()),
                        }
                    }
                }
                _ => ()
            }
        });
    });

    rx.recv().unwrap_or(())
}

pub fn and(ss: &[bool]) -> bool {
    ss.iter().all(|&b| b)
}

pub fn or(ss: &[bool]) -> bool {
    ss.iter().any(|&b| b)
}

pub fn xor(ss: &[bool]) -> bool {
    ss.iter().filter(|&&b| b).count() == 1
}

pub fn write_png_greyscale(m: &Matrix, path: &Path, scale_factor: u32) -> Result<(), image::ImageError> {
    assert!(m.shape.0.len() == 2, "Only 2x2 can be converted to images");

    let img_buffer = image::ImageBuffer::from_fn(m.shape.0[1] as u32, m.shape.0[0] as u32, |x, y| {
        let index = Index(vec![y as usize, x as usize]);
        image::Luma([m[&index] as u8])
    });

    let img = image::DynamicImage::ImageLuma8(img_buffer);
    let (width, height) = img.dimensions();

    let mut fout = File::create(path)?;
    image::DynamicImage::from(img).resize(width * scale_factor, height * scale_factor, image::imageops::FilterType::Nearest).save(&mut fout, image::PNG)
}
