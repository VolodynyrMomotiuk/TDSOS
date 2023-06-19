mod null_console;

use crate::synchronization::{self, NullLock};
pub mod interface {
    use core::fmt;
    pub trait Write {
        fn write_char(&self, c: char);
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
        fn flush(&self);
    }
    pub trait Read {
        fn read_char(&self) -> char {
            ' '
        }
        fn clear_rx(&self);
    }
    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }
        fn chars_read(&self) -> usize {
            0
        }
    }
    pub trait All: Write + Read + Statistics {}
}
static CUR_CONSOLE: NullLock<&'static (dyn interface::All + Sync)> =
    NullLock::new(&null_console::NULL_CONSOLE);
use synchronization::interface::Mutex;
pub fn register_console(new_console: &'static (dyn interface::All + Sync)) {
    CUR_CONSOLE.lock(|con| *con = new_console);
}
pub fn console() -> &'static dyn interface::All {
    CUR_CONSOLE.lock(|con| *con)
}
