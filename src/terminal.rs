use std::io::{self, Write};

use termion::{
    cursor,
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, IntoAlternateScreen},
};
/// The terminal on which the entire rendering will take place
/// is handled by `CursorGuard` itself; `RawMode` and the alternative buffer
/// are all contained within a single structure
pub struct Terminal /*alias Screen*/ {
    /// Improved and safe `stdout`
    pub screen: AlternateScreen<RawTerminal<io::Stdout>>,
}
impl Terminal {
    /// Crate new terminal with alternative buffer, hide cursor and in raw mode
    pub fn new() -> anyhow::Result<Self> {
        let raw = io::stdout().into_raw_mode()?;
        let mut alt = raw.into_alternate_screen()?;
        write!(alt, "{}", cursor::Hide)?;
        alt.flush()?;
        Ok(Terminal { screen: alt })
    }
    /// Return field screen, for use in `.flip()` from surface module
    pub fn writer(&mut self) -> &mut impl Write {
        &mut self.screen
    }
}
impl Drop for Terminal {
    /// Return back cursor
    fn drop(&mut self) {
        let _ = write!(self.screen, "{}", cursor::Show);
        let _ = self.screen.flush();
    }
}
