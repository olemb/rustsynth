extern crate riff_wave;

mod audiofile;

struct Osc {
    note: f64,  // Fractional MIDI note.
    phase: f64, // 0..1
}

fn saw(phase: f64) -> f64 {
    1.0 - phase * 2.0
}

/*
fn square(phase: f64) -> f64 {
    if phase < 0.5 {
        1.0
    } else {
        -1.0
    }
}
*/

// Convert fractional MIDI note to frequency.
fn to_freq(note: f64) -> f64 {
    // MIDI note 69 is exactly 440Hz.
    // Use this as a base.
    440.0 * (2.0 as f64).powf((note - 69.0) / 12.0)
}

fn inc_phase(osc: &mut Osc, elapsed_time: f64) {
    osc.phase = (osc.phase + to_freq(osc.note) * elapsed_time).fract()
}

fn main() {
    let sample_rate = 44100;
    let elapsed_time = 1.0 / sample_rate as f64;
    let mut osc1 = Osc {
        note: 60.0,
        phase: 0.0,
    };
    let mut osc2 = Osc {
        note: 24.0,
        phase: 0.0,
    };
    let mut samples = Vec::new();

    for _ in 0..8 {
        for note in [60.0, 72.0, 60.0, 69.0, 60.0, 67.0, 60.0, 69.0].iter() {
            let transposed_note = *note - 12.0;
            osc1.note = transposed_note;
            osc2.note = transposed_note + 0.01;

            for _ in 0..10000 {
                samples.push((saw(osc1.phase) + saw(osc2.phase)) * 0.3);
                inc_phase(&mut osc1, elapsed_time);
                inc_phase(&mut osc2, elapsed_time);
            }
        }
    }

    audiofile::save_samples("out.wav", &samples);
}
