mod framebuffer;
mod conway;
mod patterns;

use framebuffer::Framebuffer;
use conway::next_generation;
use patterns::initialize_world;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::{Duration, Instant};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    // -----------------------------
    // Configuración del framebuffer
    // -----------------------------

    framebuffer.set_background_color(0x1E1E2E);
    framebuffer.set_current_color(0xFFFF00);

    initialize_world(&mut framebuffer);

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

    // -----------------------------
    // Estado de la simulación
    // -----------------------------

    let mut paused = false;
    let mut generation: u64 = 0;

    let mut generation_time = Duration::from_millis(150);
    let mut last_update = Instant::now();

    // -----------------------------
    // Game loop
    // -----------------------------

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Pause / Resume
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            paused = !paused;
            last_update = Instant::now();
        }

        // Reset
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            initialize_world(&mut framebuffer);

            generation = 0;
            last_update = Instant::now();
        }

        // Avanzar manualmente una generación
        if paused && window.is_key_pressed(Key::N, KeyRepeat::No) {
            next_generation(&mut framebuffer);
            generation += 1;
        }

        // Aumentar velocidad
        if window.is_key_pressed(Key::Up, KeyRepeat::No) {
            let current_ms = generation_time.as_millis() as u64;
            let new_ms = current_ms.saturating_sub(25).max(25);

            generation_time = Duration::from_millis(new_ms);
        }

        // Disminuir velocidad
        if window.is_key_pressed(Key::Down, KeyRepeat::No) {
            let current_ms = generation_time.as_millis() as u64;
            let new_ms = (current_ms + 25).min(1000);

            generation_time = Duration::from_millis(new_ms);
        }

        // Actualización automática
        if !paused && last_update.elapsed() >= generation_time {
            next_generation(&mut framebuffer);

            generation += 1;
            last_update = Instant::now();
        }

        // Estado mostrado en el título
        let status = if paused {
            "PAUSED"
        } else {
            "RUNNING"
        };

        let title = format!(
            "Conway's Game of Life - Generation: {} | {} | {} ms",
            generation,
            status,
            generation_time.as_millis()
        );

        window.set_title(&title);

        // Mostrar framebuffer
        window
            .update_with_buffer(
                &framebuffer.buffer,
                framebuffer.width,
                framebuffer.height,
            )
            .expect("No se pudo actualizar la ventana");
    }
}