extern crate gl;

pub struct Graphics {
    clear_color: (f32, f32, f32, f32),
}

impl Graphics {
    pub fn new() -> Self {
        unsafe {
            gl::ClearColor(0.1, 0.2, 0.3, 1.0);
        }
        Self {
            clear_color: (0.1, 0.2, 0.3, 1.0),
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    // Opdaterer baggrundsfarven
    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.clear_color = (r, g, b, a);
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }
}