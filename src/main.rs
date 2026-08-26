mod framebuffer;
mod conway;

use framebuffer::Framebuffer;
use conway::next_generation;

use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    // Colores
    framebuffer.set_background_color(0x1E1E2E);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFF00);

    // -----------------------------
    // Patrón inicial: Blinker
    // -----------------------------

    framebuffer.point(49, 50);
    framebuffer.point(50, 50);
    framebuffer.point(51, 50);

    // -----------------------------
    // Ventana
    // -----------------------------

    let mut window = Window::new(
        "Conway's Game of Life - Lab 2",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X8,
            ..WindowOptions::default()
        },
    )
    .expect("No se pudo crear la ventana");

    // Cada cuánto tiempo avanzamos una generación.
    let generation_time = Duration::from_millis(150);

    let mut last_update = Instant::now();

    // -----------------------------
    // Game loop
    // -----------------------------

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if last_update.elapsed() >= generation_time {
            next_generation(&mut framebuffer);

            last_update = Instant::now();
        }

        window
            .update_with_buffer(
                &framebuffer.buffer,
                framebuffer.width,
                framebuffer.height,
            )
            .expect("No se pudo actualizar la ventana");
    }
}