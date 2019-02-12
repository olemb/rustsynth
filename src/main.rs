extern crate riff_wave;
    
mod audio;

struct Osc {
    freq: f64,  // Hz.
    amp: f64,  // 0..1
    phase: f64,  // 0..1
}

fn saw(phase: f64) -> f64 {
    1.0 - phase * 2.0
}

fn inc_phase(phase: f64, freq: f64, elapsed_time: f64) -> f64 {
    (phase + freq * elapsed_time).fract()}

fn main() {
    let sample_rate = 44100;
    let elapsed_time = 1.0 / sample_rate as f64;
    let mut osc = Osc{freq: 440.0, amp: 1.0, phase: 0.0};
    let mut samples = Vec::new();

    for n in 0..44100 {
        if n > 0 && n % 8000 == 0 {
            osc.freq /= 2.0;
        }
        samples.push(saw(osc.phase) * osc.amp);
        osc.phase = inc_phase(osc.phase, osc.freq, elapsed_time);
    }

    audio::save_samples(&String::from("out.wav"), &samples);
}
