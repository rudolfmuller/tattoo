use crate::{
    Position, Scale,
    terminal::{self, Terminal},
};
use std::io::Write;
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

    /// use tattoo::terminal::Terminal;
    /// use tattoo::Scale;
    /// use tattoo::surface::Surface;
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let mut screen = Terminal::new()?;
    ///     let mut master = Surface::new(' ', Scale { w: 10, h: 10 });
    ///     
    ///     master.flip(&mut screen.writer())?;
    ///
    ///     Ok(())
    /// }    
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
    /// use tattoo::terminal::Terminal;
    /// use tattoo::{Position, Scale};
    /// use tattoo::surface::Surface;
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let mut screen = Terminal::new()?;
    ///     let mut master = Surface::new(' ', Scale { w: 10, h: 10 });
    ///     let surface = Surface::new('.', Scale { w: 5, h: 5 });
    ///     
    ///     master.blit(&surface, Position { x: 5, y: 5 });
    ///     
    ///     master.flip(&mut screen.writer())?;
    ///
    ///     Ok(())
    /// }    
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
    /// ```rust
    /// use anyhow::Context;
    /// use tattoo::Scale;
    /// use tattoo::surface::Surface;
    /// use tattoo::terminal::Terminal;
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let mut screen = Terminal::new().context("failed to create terminal")?;
    ///     let mut master = Surface::new(' ', Scale { w: 10, h: 10 });
    ///
    ///     master.flip(&mut screen.writer()).context("failed to flip surface")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn flip(&mut self, terminal: &mut impl Write) -> std::io::Result<()> {
        write!(terminal, "{}", clear::All)?;

        for (i, row) in self.surface.iter().enumerate() {
            write!(terminal, "{}", cursor::Goto(1, (i + 1) as u16))?;
            let line: String = row.iter().collect();
            writeln!(terminal, "{}", line)?;
        }
        terminal.flush()?;
        Ok(())
    }
}
