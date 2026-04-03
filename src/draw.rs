use crate::surface::Surface;
use crate::{Border, Position};

/// `label` Write massage on surface
/// # Example
/// ```rust
/// use tattoo::draw;
/// use tattoo::terminal::Terminal;
/// use tattoo::{Scale, Position};
/// use tattoo::surface::Surface;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let screen = Terminal::new()?;
///     let mut master = Surface::new('.', Scale { w: 20, h: 10 });
///     master.set_terminal(screen);
///     draw::label(&mut master, Position { x: 5, y: 5 }, "Hello, world");
///     master.flip()?;
///
///     Ok(())
/// }
/// ```
pub fn label(surface: &mut Surface, position: Position, msg: &str) {
    let mut x = position.x;
    let y = position.y;
    for c in msg.chars() {
        set(surface, Position { x, y }, c);
        x += 1;
    }
}
/// A safe way to write a `char` on the surface
///
/// Checks whether the position we want to draw at is within the surface bounds
pub fn set(surface: &mut Surface, position: Position, c: char) {
    let x = position.x;
    let y = position.y;

    if surface.is_in_bounds(position) {
        surface.surface[y][x] = c;
    }
}
/// `border_layout` Draw a border around `Surface`
/// # Example
/// ```
/// use tattoo::surface::Surface;
/// use tattoo::draw;
/// use tattoo::terminal::Terminal;
/// use tattoo::{Scale, Border};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let screen = Terminal::new()?;
///     let mut window = Surface::new(' ', Scale { w: 20, h: 5 });
///     window.set_terminal(screen);
///     draw::border_layout(
///         &mut window,
///         Border {
///             tl: '╭',
///             tr: '╮',
///             bl: '└',
///             br: '┘',
///             h: '─',
///             v: '│',
///         },
///     );
///
///     window.flip()?;
///
///     Ok(())
/// }
/// ```
pub fn border_layout(surface: &mut Surface, border: Border) {
    let width = surface.scale.w;
    let height = surface.scale.h;

    if width == 0 || height == 0 {
        return;
    }

    set(surface, Position { x: 0, y: 0 }, border.tl);
    set(surface, Position { x: width - 1, y: 0 }, border.tr);
    set(
        surface,
        Position {
            x: 0,
            y: height - 1,
        },
        border.bl,
    );
    set(
        surface,
        Position {
            x: width - 1,
            y: height - 1,
        },
        border.br,
    );

    for x in 1..(width - 1) {
        set(surface, Position { x, y: 0 }, border.h);
        set(surface, Position { x, y: height - 1 }, border.h);
    }

    for y in 1..(height - 1) {
        set(surface, Position { x: 0, y }, border.v);
        set(surface, Position { x: width - 1, y }, border.v);
    }
}

/// Draws a simple ASCII border
/// Uses `-` for all edges and spaces for the inside
pub fn border_ascii(surface: &mut Surface) {
    border_layout(
        surface,
        Border {
            tl: '-',
            tr: '-',
            bl: '-',
            br: '-',
            h: '-',
            v: ' ',
        },
    );
}

/// Draws a classic terminal-style border using box-drawing characters
/// Uses `┌┐└┘` for corners and `─│` for edges
pub fn border_classic(surface: &mut Surface) {
    border_layout(
        surface,
        Border {
            tl: '┌',
            tr: '┐',
            bl: '└',
            br: '┘',
            h: '─',
            v: '│',
        },
    );
}

/// Draws a rounded border with smooth corners
/// Uses `╭╮╰╯` for corners and `─│` for edges
pub fn border_rounded(surface: &mut Surface) {
    border_layout(
        surface,
        Border {
            tl: '╭',
            tr: '╮',
            bl: '╰',
            br: '╯',
            h: '─',
            v: '│',
        },
    );
}
