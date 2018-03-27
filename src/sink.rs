
/**
  * Signal sinks
  */

use std::f32;
use std::collections::VecDeque;
use cpal::*;
use std::thread;
use std::sync::mpsc::channel;

pub fn min(signal: &[f32]) -> f32 { signal.iter().fold(f32::MAX, |acc, &s| if s < acc { s } else { acc })
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
                },
                _ => ()
            }
        });
    });

    rx.recv().unwrap_or(())
}
