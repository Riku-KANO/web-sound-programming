use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

// web-sysは無くても行けそう。多分いらない
use web_sys::console;

#[wasm_bindgen]
pub struct Oscillator {
    sample_rate: f32,
    frequency: f32,
    phase: f32,
    buffer: Box<[f32]>,
}

#[wasm_bindgen]
impl Oscillator {
    #[wasm_bindgen(constructor)]
    pub fn new(sample_rate: f32, frequency: f32) -> Result<Oscillator, JsValue> {


        Ok(Oscillator {
            sample_rate,
            frequency,
            phase: 0.0,
            buffer: vec![0.0; 128].into_boxed_slice(),
        })
    }

    #[wasm_bindgen]
    pub fn generate_samples(&mut self) -> Vec<f32> {
        for sample in self.buffer.iter_mut() {
            *sample = (2.0 * PI * self.phase).sin();
            self.phase += self.frequency / self.sample_rate;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }
        self.buffer.to_vec()
    }
}
