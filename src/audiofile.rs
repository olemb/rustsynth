use std::fs::File;
use std::io::BufWriter;

use riff_wave::WaveWriter; // , WriteResult};

fn encode_sample(sample: &f64) -> i16 {
    (sample * 32000.0) as i16
}

pub fn save_samples(filename: &str, samples: &[f64]) {
    let sample_rate = 44100;
    let file = File::create(&filename.to_string()).unwrap();
    let writer = BufWriter::new(file);
    let mut wave_writer = WaveWriter::new(2, sample_rate, 16, writer).unwrap();

    for sample in samples.iter() {
        // Just write the same sample for both channels.
        wave_writer.write_sample_i16(encode_sample(sample)).unwrap();
        wave_writer.write_sample_i16(encode_sample(sample)).unwrap();
    }
}
