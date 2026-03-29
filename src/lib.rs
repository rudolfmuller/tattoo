pub mod draw;
pub mod guard;
pub mod surface;

/// The `Position` is a structure composed of variables of type `usize` named `x` and `y`
/// # Example
/// ```
/// use tattoo::Position;
///
/// let position = Position { x: 5, y: 5 };
/// assert_eq!(position, Position { x: 5, y: 5 });
///
/// ```
#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    /// Return the value as tuple `(x, y)` of type `usize`
    pub fn to_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

/// The `Scale` is a structure composed of variables of type `usize` named `w` and `h`
/// # Example
/// ```
/// use tattoo::Scale;
///
/// let scale = Scale { w: 6, h: 7 };
/// assert_eq!(scale, Scale { w: 6, h: 7 });
///
/// ```
#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub struct Scale {
    pub w: usize,
    pub h: usize,
}
impl Scale {
    /// Return the value as tuple `(w, h)` of type `usize`
    pub fn to_tuple(&self) -> (usize, usize) {
        (self.w, self.h)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{guard::CursorGuard, surface::Surface};

    #[test]
    fn it_works() -> std::io::Result<()> {
        // init
        let _cursor_guard = CursorGuard::new()?;
        let mut master = Surface::new('.', Scale { w: 20, h: 10 });
        let mut screen = Surface::new(' ', Scale { w: 10, h: 5 });
        draw::label(&mut screen, Position { x: 0, y: 0 }, "Hello");
        master.blit(&screen, Position { x: 10, y: 5 });
        for _ in 0..20 {
            master.flip()?;
            std::thread::sleep(std::time::Duration::from_millis(42));
        }
        Ok(())
    }
}
