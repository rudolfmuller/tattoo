use crate::{Position, Scale, terminal::Terminal};
use std::io::{self, Write};
use termion::{clear, cursor};

/// Surface error(s)
#[derive(thiserror::Error, Debug)]
pub enum SurfaceError {
    #[error("terminal missing")]
    TerminalMissing,
    /// Occurs when there is a problem with `stdout`
    #[error(transparent)]
    IoError(#[from] io::Error),
}

pub struct Surface {
    /// I know `Vec<Vec<T>>` is not good but is simple to code and more readble for me
    pub surface: Vec<Vec<char>>,
    /// The terminal to which .`flip()` will render
    pub terminal: Option<Terminal>,
    /// Scale of surface
    pub scale: Scale,
}
impl Surface {
    /// Create a new surface
    /// # Example
    /// ```
    /// use tattoo::Scale;
    /// use tattoo::terminal::Terminal;
    /// use tattoo::surface::Surface;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let screen = Terminal::new()?;
    ///     let mut master = Surface::new(' ', Scale { w: 10, h: 10 });
    ///     master.set_terminal(screen);
    ///     master.flip()?;
    ///
    ///     Ok(())
    /// }    
    /// ```
    pub fn new(fill: char, scale: Scale) -> Self {
        Self {
            surface: vec![vec![fill; scale.w]; scale.h],
            scale,
            terminal: None,
        }
    }
    /// Draw a other surface on this surface
    ///
    /// Anchored on top-left
    /// # Example
    /// ```
    /// use tattoo::{Position, Scale};
    /// use tattoo::terminal::Terminal;
    /// use tattoo::surface::Surface;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let screen = Terminal::new()?;
    ///     let mut master = Surface::new(' ', Scale { w: 10, h: 10 });
    ///     master.set_terminal(screen);
    ///     let surface = Surface::new('.', Scale { w: 5, h: 5 });
    ///     
    ///     master.blit(&surface, Position { x: 5, y: 5 });
    ///     
    ///     master.flip()?;
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

    /// Set terminal to which `.flip()` will render
    pub fn set_terminal(&mut self, terminal: Terminal) {
        self.terminal = Some(terminal);
    }

    /// Write surface and clear terminal
    /// Example
    /// ```rust
    /// use tattoo::Scale;
    /// use tattoo::terminal::Terminal;
    /// use tattoo::surface::Surface;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let screen = Terminal::new()?;
    ///     let mut master = Surface::new(' ', Scale { w: 10, h: 10 });
    ///     master.set_terminal(screen);
    ///     master.flip()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn flip(&mut self) -> Result<(), SurfaceError> {
        let terminal = self
            .terminal
            .as_mut()
            .ok_or(SurfaceError::TerminalMissing)?;

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
