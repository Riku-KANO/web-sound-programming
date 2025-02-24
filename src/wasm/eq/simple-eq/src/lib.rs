use wasm_bindgen::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex};

#[wasm_bindgen]
pub struct SimpleEq {
    gains: Vec<f32>,
    sample_rate: f32,
}

#[wasm_bindgen]
impl SimpleEq {
    #[wasm_bindgen(constructor)]
    pub fn new(bands: usize) -> SimpleEq {
        SimpleEq {
            gains: vec![1.0; bands],
            sample_rate: 44100.0,
        }
    }

    #[wasm_bindgen]
    pub fn set_gain(&mut self, band: usize, gain: f32) {
        if band < self.gains.len() {
            self.gains[band] = gain;
        }
    }

    #[wasm_bindgen]
    pub fn get_gain(&self, band: usize) -> f32 {
        if band < self.gains.len() {
            self.gains[band]
        } else {
            1.0
        }
    }

    pub fn apply(&self, input: &[f32], output: &mut [f32]) {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(input.len());
        let ifft = planner.plan_fft_inverse(input.len());

        let mut buffer: Vec<Complex<f32>> = input.iter().map(|&x| Complex { re: x, im: 0.0 }).collect();
        fft.process(&mut buffer);

        for (i, freq) in buffer.iter_mut().enumerate() {
            let band = i % self.gains.len();
            freq.re *= self.gains[band];
            freq.im *= self.gains[band];
        }

        ifft.process(&mut buffer);

        for (i, sample) in buffer.iter().enumerate() {
            output[i] = sample.re;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_eq_no_gain_change() {
        let mut eq = SimpleEq::new(8);
        let input = vec![0.0; 128];
        let mut output = vec![0.0; 128];
        eq.apply(&input, &mut output);
        assert_eq!(input, output);
    }

    #[test]
    fn test_simple_eq_gain_change() {
        let mut eq = SimpleEq::new(8);
        eq.set_gain(0, 2.0);
        let input = vec![1.0; 128];
        let mut output = vec![0.0; 128];
        eq.apply(&input, &mut output);
        assert_eq!(output[0], 2.0);
    }

    #[test]
    fn test_simple_eq_multiple_bands() {
        let mut eq = SimpleEq::new(8);
        eq.set_gain(0, 2.0);
        eq.set_gain(1, 0.5);
        let input = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let mut output = vec![0.0; 8];
        eq.apply(&input, &mut output);
        assert_eq!(output[0], 2.0);
        assert_eq!(output[1], 0.5);
    }
}