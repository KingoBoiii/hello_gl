extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};

mod graphics;
use graphics::Graphics;

fn main() {
    println!("Hello, world!");

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events)  = glfw
        .create_window(800, 600, "GLFW window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut graphics = Graphics::new();

    while !window.should_close() {
        graphics.clear();

        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::Key(Key::C, _, Action::Press, _) => {
                    // Skift baggrundsfarve til rød, når 'C' trykkes
                    graphics.set_clear_color(1.0, 0.0, 0.0, 1.0);
                }
                _ => {}
            }
        }

        window.swap_buffers();
    }
}
