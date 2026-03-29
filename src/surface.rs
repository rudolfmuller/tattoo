use crate::{Position, Scale};
use std::io::{Write, stdout};
use termion::{clear, cursor};

#[derive(PartialEq, Debug)]
pub struct Surface {
    /// I know `Vec<Vec<T>>` is not good but is simple to code and more readble for me
    pub surface: Vec<Vec<char>>,
    pub scale: Scale,
}
impl Surface {
    /// Create a new surface
    /// # Example
    /// ```
    /// use tattoo::Scale;
    /// use tattoo::surface::Surface;
    /// let mut master = Surface::new(' ', Scale { w: 10, h: 10 });
    /// master.flip();
    /// ```
    pub fn new(fill: char, scale: Scale) -> Surface {
        Surface {
            surface: vec![vec![fill; scale.w]; scale.h],
            scale,
        }
    }
    /// Draw a other surface on this surface
    ///
    /// Anchored on top-left
    /// # Example
    /// ```
    /// use tattoo::{Position, Scale};
    /// use tattoo::surface::Surface;
    ///
    /// let mut master = Surface::new(' ', Scale { w: 10, h: 10 });
    /// let surface = Surface::new('.', Scale { w: 5, h: 5 });
    /// master.blit(&surface, Position { x: 5, y: 5 });
    /// master.flip();
    /// ```

    pub fn blit(&mut self, other: &Surface, position: Position) {
        for (dy, row) in other.surface.iter().enumerate() {
            for (dx, &c) in row.iter().enumerate() {
                let y = position.y + dy;
                let x = position.x + dx;
                if y < self.surface.len() && x < self.surface[y].len() {
                    self.surface[y][x] = c;
                }
            }
        }
    }
    /// Write surface and clear terminal
    /// Example
    /// ```
    /// use tattoo::Scale;
    /// use tattoo::surface::Surface;
    /// let mut master = Surface::new(' ', Scale { w: 10, h: 10});
    /// loop {
    ///     match master.flip() {
    ///       Err(_) => break,
    ///       Ok(_) => continue,
    ///     }
    /// }
    /// ```
    pub fn flip(&mut self) -> std::io::Result<()> {
        let stdout = stdout();
        let mut out = stdout.lock();
        write!(out, "{}{}", clear::All, cursor::Goto(1, 1))?;

        for row in &self.surface {
            let line: String = row.iter().collect();
            writeln!(out, "{}", line)?;
        }
        out.flush()?;
        Ok(())
    }
}
