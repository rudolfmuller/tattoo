use crate::Position;
use crate::surface::Surface;

/// Replace `char` from `from` to `to`
pub fn replace_char(surface: &mut Surface, from: Position, to: Position) {
    surface.set_char(to, surface.get_char(from));
    surface.set_char(from, surface.fill);
}
