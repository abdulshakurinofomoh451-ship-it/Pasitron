use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

#[wasm_bindgen]
pub struct StasisController {
    particle_count: u32,
    time: f32,
    magnetic_strength: f32,
}

#[wasm_bindgen]
impl StasisController {
    #[wasm_bindgen(constructor)]
    pub fn new(count: u32) -> Self {
        console_error_panic_hook::set_once();
        Self {
            particle_count: count,
            time: 0.0,
            magnetic_strength: 0.75,
        }
    }

    pub fn set_magnetic_strength(&mut self, value: f32) {
        self.magnetic_strength = value;
    }

    pub fn update_simulation(&mut self, delta_time: f32) -> String {
        self.time += delta_time;

        // Fake but heavy CPU-side math to represent control logic
        let oscillation = (self.time * 0.001 * PI).sin();
        let energy = oscillation * self.magnetic_strength * self.particle_count as f32;

        format!(
            "t={:.2}ms | particles={} | B={:.2} | energy={:.3e}",
            self.time,
            self.particle_count,
            self.magnetic_strength,
            energy
        )
    }

    pub fn particle_count(&self) -> u32 {
        self.particle_count
    }
}
