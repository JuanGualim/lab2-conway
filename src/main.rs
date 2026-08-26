mod framebuffer;
mod conway;
mod patterns;

use framebuffer::Framebuffer;
use conway::next_generation;
use patterns::{
    draw_pattern,
    BLOCK,
    BEEHIVE,
    BLINKER,
    TOAD,
    BEACON,
    GLIDER,
    LWSS,
    PULSAR,
};
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

    // Still lifes
    draw_pattern(&mut framebuffer, BLOCK, 10, 10);
    draw_pattern(&mut framebuffer, BEEHIVE, 25, 10);

    // Oscillators
    draw_pattern(&mut framebuffer, BLINKER, 45, 10);
    draw_pattern(&mut framebuffer, TOAD, 60, 10);
    draw_pattern(&mut framebuffer, BEACON, 80, 10);

    // Spaceships
    draw_pattern(&mut framebuffer, GLIDER, 10, 40);
    draw_pattern(&mut framebuffer, LWSS, 30, 40);

    // Oscilador grande
    draw_pattern(&mut framebuffer, PULSAR, 60, 50);

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