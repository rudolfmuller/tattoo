use crate::Position;
use crate::surface::Surface;

fn is_valid_range(surface: &mut Surface, x: usize, y: usize) -> bool {
    y < surface.surface.len() && x < surface.surface[y].len()
}

/// `label` Write massage on surface
/// # Example
/// ```
/// use tattoo::draw;
/// use tattoo::{Scale, Position};
/// use tattoo::surface::Surface;
///
/// let mut master = Surface::new('.', Scale { w: 20, h: 10 });
/// draw::label(&mut master, Position { x: 5, y: 5 }, "Hello, world");
/// master.flip();
/// ```
pub fn label(surface: &mut Surface, position: Position, msg: &str) {
    let mut x = position.x;
    let y = position.y;
    for c in msg.chars() {
        if is_valid_range(surface, x, y) {
            surface.surface[y][x] = c;
        }
        x += 1;
    }
}
/// `set` Write single `char` on surface
pub fn set(surface: &mut Surface, position: Position, c: char) {
    let x = position.x;
    let y = position.y;

    if is_valid_range(surface, x, y) {
        surface.surface[y][x] = c;
    }
}
