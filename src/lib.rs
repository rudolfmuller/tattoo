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
#[derive(PartialEq, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
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
#[derive(PartialEq, Debug)]
pub struct Scale {
    pub w: usize,
    pub h: usize,
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
