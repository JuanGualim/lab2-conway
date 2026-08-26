use crate::framebuffer::Framebuffer;

/// Determina si una célula está viva.
///
/// Consideramos viva una célula cuyo color sea igual
/// al current_color del framebuffer.
pub fn is_alive(framebuffer: &Framebuffer, x: usize, y: usize) -> bool {
    framebuffer.get_color(x, y) == Some(framebuffer.current_color())
}

/// Cuenta los vecinos vivos de una célula.
///
/// El tablero utiliza wrap-around:
/// - salir por la izquierda lleva a la derecha
/// - salir por la derecha lleva a la izquierda
/// - salir por arriba lleva abajo
/// - salir por abajo lleva arriba
pub fn count_neighbors(
    framebuffer: &Framebuffer,
    x: usize,
    y: usize,
) -> u8 {
    let mut count = 0;

    let width = framebuffer.width as isize;
    let height = framebuffer.height as isize;

    let x = x as isize;
    let y = y as isize;

    for dy in -1..=1 {
        for dx in -1..=1 {

            // No contamos la propia célula.
            if dx == 0 && dy == 0 {
                continue;
            }

            // Wrap-around.
            let nx = (x + dx).rem_euclid(width);
            let ny = (y + dy).rem_euclid(height);

            if is_alive(framebuffer, nx as usize, ny as usize) {
                count += 1;
            }
        }
    }

    count
}