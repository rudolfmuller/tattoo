use std::io::{self, Write};
use termion::{
    cursor,
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, IntoAlternateScreen},
};

/// Terminal error(s)
#[derive(thiserror::Error, Debug)]
pub enum TerminalError {
    /// Occurs when there is a problem with `stdout`
    #[error(transparent)]
    IoError(#[from] io::Error),
}

/// The terminal on which the entire rendering will take place
/// is handled by `CursorGuard` itself; `RawMode` and the alternative buffer
/// are all contained within a single structure
pub struct Terminal /*alias Screen*/ {
    /// Improved and safe `stdout`
    pub screen: AlternateScreen<RawTerminal<io::Stdout>>,
}
impl Terminal {
    /// Crate new terminal with alternative buffer, hide cursor and in raw mode
    pub fn new() -> Result<Self, TerminalError> {
        let raw = io::stdout().into_raw_mode()?;
        let mut alt = raw.into_alternate_screen()?;
        write!(alt, "{}", cursor::Hide)?;
        alt.flush()?;
        Ok(Self { screen: alt })
    }
    fn close(&mut self) {
        let _ = write!(self.screen, "{}", cursor::Show);
        let _ = self.screen.flush();
    }
}
impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.screen.write(buf)
    }
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.screen.write_all(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.screen.flush()
    }
}
impl Drop for Terminal {
    /// Return back cursor
    fn drop(&mut self) {
        self.close();
    }
}
