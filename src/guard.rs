use std::io::{Write, stdout};
use termion::cursor;

/// RAII guard for terminal cursor visibility
///
/// Hides the cursor on creation and automatically shows it again
/// when the guard is dropped.
///
/// # Example
/// ```
/// use tattoo::guard::CursorGuard;
/// let _cursor_guard = CursorGuard::new().unwrap() /*<-- in final project dont use .unwrap()!!!*/;
/// ```
#[derive(Debug)]
pub struct CursorGuard;
impl CursorGuard {
    pub fn new() -> std::io::Result<CursorGuard> {
        let stdout = stdout();
        let mut out = stdout.lock();
        write!(out, "{}", cursor::Hide)?;
        out.flush()?;
        Ok(CursorGuard)
    }
}
impl Drop for CursorGuard {
    /// Return back cursor
    fn drop(&mut self) {
        let stdout = stdout();
        let mut out = stdout.lock();
        let _ = write!(out, "{}", cursor::Show);
        let _ = out.flush();
    }
}
