mod framebuffer;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    framebuffer.set_background_color(0x1E1E2E);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFF00);

    // Algunos puntos de prueba
    framebuffer.point(50, 50);
    framebuffer.point(51, 50);
    framebuffer.point(52, 50);

    framebuffer.point(50, 51);
    framebuffer.point(50, 52);

    println!(
        "Color (50,50): {:?}",
        framebuffer.get_color(50, 50)
    );

    println!(
        "Color (10,10): {:?}",
        framebuffer.get_color(10, 10)
    );

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(
                &framebuffer.buffer,
                framebuffer.width,
                framebuffer.height,
            )
            .expect("No se pudo actualizar la ventana");
    }
}