use crate::framebuffer::Framebuffer;

pub const BLOCK: &[(usize, usize)] = &[
    (0, 0),
    (1, 0),
    (0, 1),
    (1, 1),
];

pub const BEEHIVE: &[(usize, usize)] = &[
    (1, 0),
    (2, 0),
    (0, 1),
    (3, 1),
    (1, 2),
    (2, 2),
];

pub const BLINKER: &[(usize, usize)] = &[
    (0, 0),
    (0, 1),
    (0, 2),
];

pub const TOAD: &[(usize, usize)] = &[
    (1, 0),
    (2, 0),
    (3, 0),
    (0, 1),
    (1, 1),
    (2, 1),
];

pub const BEACON: &[(usize, usize)] = &[
    (0, 0),
    (1, 0),
    (0, 1),
    (1, 1),
    (2, 2),
    (3, 2),
    (2, 3),
    (3, 3),
];

pub const GLIDER: &[(usize, usize)] = &[
    (1, 0),
    (2, 1),
    (0, 2),
    (1, 2),
    (2, 2),
];

pub const LWSS: &[(usize, usize)] = &[
    (1, 0),
    (2, 0),
    (3, 0),
    (4, 0),
    (0, 1),
    (4, 1),
    (4, 2),
    (0, 3),
    (3, 3),
];

pub const PULSAR: &[(usize, usize)] = &[
    (2, 0), (3, 0), (4, 0),
    (8, 0), (9, 0), (10, 0),

    (0, 2), (5, 2), (7, 2), (12, 2),
    (0, 3), (5, 3), (7, 3), (12, 3),
    (0, 4), (5, 4), (7, 4), (12, 4),

    (2, 5), (3, 5), (4, 5),
    (8, 5), (9, 5), (10, 5),

    (2, 7), (3, 7), (4, 7),
    (8, 7), (9, 7), (10, 7),

    (0, 8), (5, 8), (7, 8), (12, 8),
    (0, 9), (5, 9), (7, 9), (12, 9),
    (0, 10), (5, 10), (7, 10), (12, 10),

    (2, 12), (3, 12), (4, 12),
    (8, 12), (9, 12), (10, 12),
];

pub fn draw_pattern(
    framebuffer: &mut Framebuffer,
    pattern: &[(usize, usize)],
    start_x: usize,
    start_y: usize,
) {
    for &(dx, dy) in pattern {
        let x = (start_x + dx) % framebuffer.width;
        let y = (start_y + dy) % framebuffer.height;

        framebuffer.point(x, y);
    }
}
pub fn initialize_world(framebuffer: &mut Framebuffer) {
    framebuffer.clear();

    // =========================================================
    // ZONA SUPERIOR
    // =========================================================

    // Glider con bastante espacio para desplazarse.
    draw_pattern(framebuffer, GLIDER, 5, 5);

    // Still lifes.
    draw_pattern(framebuffer, BLOCK, 40, 7);
    draw_pattern(framebuffer, BEEHIVE, 70, 8);

    // Osciladores.
    draw_pattern(framebuffer, BLINKER, 20, 18);
    draw_pattern(framebuffer, TOAD, 50, 17);
    draw_pattern(framebuffer, BEACON, 85, 18);

    // =========================================================
    // ZONA MEDIA SUPERIOR
    // =========================================================

    // Spaceship horizontal.
    draw_pattern(framebuffer, LWSS, 5, 30);

    // Algunos organismos estáticos/osciladores.
    draw_pattern(framebuffer, BLOCK, 30, 30);
    draw_pattern(framebuffer, BEEHIVE, 70, 30);

    // =========================================================
    // ZONA CENTRAL
    // =========================================================

    // Pulsar como pieza central de la simulación.
    draw_pattern(framebuffer, PULSAR, 43, 42);

    // Osciladores alrededor, pero suficientemente separados.
    draw_pattern(framebuffer, BLINKER, 20, 48);
    draw_pattern(framebuffer, BEACON, 75, 45);

    // =========================================================
    // ZONA MEDIA INFERIOR
    // =========================================================

    // Segundo Glider.
    draw_pattern(framebuffer, GLIDER, 68, 62);

    // Still lifes.
    draw_pattern(framebuffer, BLOCK, 15, 65);
    draw_pattern(framebuffer, BEEHIVE, 40, 68);

    // Oscilador.
    draw_pattern(framebuffer, TOAD, 85, 68);

    // =========================================================
    // ZONA INFERIOR
    // =========================================================

    // Segundo LWSS.
    draw_pattern(framebuffer, LWSS, 8, 82);

    // Tercer Glider.
    draw_pattern(framebuffer, GLIDER, 48, 82);

    // Otros organismos para llenar la parte inferior.
    draw_pattern(framebuffer, BEACON, 72, 84);
    draw_pattern(framebuffer, BLINKER, 92, 85);
}