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
        framebuffer.point(start_x + dx, start_y + dy);
    }
}

pub fn initialize_world(framebuffer: &mut Framebuffer) {
    framebuffer.clear();

    // -----------------------------
    // Still Lifes
    // -----------------------------
    draw_pattern(framebuffer, BLOCK, 8, 8);
    draw_pattern(framebuffer, BEEHIVE, 25, 8);

    draw_pattern(framebuffer, BLOCK, 75, 8);
    draw_pattern(framebuffer, BEEHIVE, 88, 20);

    // -----------------------------
    // Oscillators
    // -----------------------------
    draw_pattern(framebuffer, BLINKER, 45, 8);
    draw_pattern(framebuffer, TOAD, 58, 12);
    draw_pattern(framebuffer, BEACON, 72, 25);

    draw_pattern(framebuffer, BLINKER, 15, 30);
    draw_pattern(framebuffer, TOAD, 30, 32);
    draw_pattern(framebuffer, BEACON, 50, 28);

    // -----------------------------
    // Spaceships
    // -----------------------------
    draw_pattern(framebuffer, GLIDER, 8, 50);
    draw_pattern(framebuffer, GLIDER, 25, 55);
    draw_pattern(framebuffer, GLIDER, 75, 55);

    draw_pattern(framebuffer, LWSS, 10, 75);
    draw_pattern(framebuffer, LWSS, 65, 78);

    // -----------------------------
    // Osciladores grandes
    // -----------------------------
    draw_pattern(framebuffer, PULSAR, 40, 45);
    draw_pattern(framebuffer, PULSAR, 80, 75);
}