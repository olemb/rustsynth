extern crate riff_wave;

use std::fs::File;
use std::io::BufWriter;

use riff_wave::{WaveWriter, WriteResult};

struct Osc {
    freq: f64,  // Hz.
    amp: f64,  // 0..1
    phase: f64,  // 0..1
}

fn saw(phase: f64) -> f64 {
    1.0 - phase * 2.0
}

fn inc_phase(phase: f64, freq: f64, elapsed_time: f64) -> f64 {
    (phase + freq * elapsed_time).fract()
}

fn write_file() -> WriteResult<()> {
    let sample_rate = 44100;
    let elapsed_time = 1.0 / 44100 as f64;
    let mut osc = Osc{freq: 440.0, amp: 1.0, phase: 0.0};

    let file = File::create("out.wav")?;
    let writer = BufWriter::new(file);
    let mut wave_writer = WaveWriter::new(2, sample_rate, 16, writer)?;

    for n in 0..44100 {
        // TODO: proper conversion.
        let sample = (saw(osc.phase) * osc.amp * 32000.0) as i16;

        if n > 0 && n % 8000 == 0 {
            osc.freq /= 2.0;
        }

        // Just write the same sample for both channels.
        try!(wave_writer.write_sample_i16(sample));
        try!(wave_writer.write_sample_i16(sample));
        osc.phase = inc_phase(osc.phase, osc.freq, elapsed_time);
    }

    Ok(())
}

fn main() {
    write_file().unwrap();
}
