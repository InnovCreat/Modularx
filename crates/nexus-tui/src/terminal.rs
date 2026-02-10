// ════════════════════════════════════════════════════════════════
// NEXUS TUI - terminal.rs
// Contrôle ANSI bas niveau + raw mode + terminal size
// Zero dependencies - pure std + POSIX syscalls
// ════════════════════════════════════════════════════════════════

use std::io::{self, Write, Read};
use std::os::unix::io::AsRawFd;
use std::mem;

// ── POSIX termios structures (no libc crate needed) ──────────

const NCCS: usize = 32;

#[repr(C)]
#[derive(Clone, Copy)]
struct Termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; NCCS],
    c_ispeed: u32,
    c_ospeed: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

// termios constants
const ECHO: u32 = 0o000010;
const ICANON: u32 = 0o000002;
const ISIG: u32 = 0o000001;
const IEXTEN: u32 = 0o100000;
const IXON: u32 = 0o002000;
const ICRNL: u32 = 0o000400;
const BRKINT: u32 = 0o000002;
const INPCK: u32 = 0o000020;
const ISTRIP: u32 = 0o000040;
const OPOST: u32 = 0o000001;
const CS8: u32 = 0o000060;
const VMIN: usize = 6;
const VTIME: usize = 5;

// ioctl request codes (Linux x86_64)
const TCGETS: u64 = 0x5401;
const TCSETS: u64 = 0x5402;
const TIOCGWINSZ: u64 = 0x5413;

extern "C" {
    fn ioctl(fd: i32, request: u64, ...) -> i32;
}

// ── Saved terminal state for cleanup ─────────────────────────

static mut ORIGINAL_TERMIOS: Option<Termios> = None;

// ── Terminal API ─────────────────────────────────────────────

pub struct Terminal;

impl Terminal {
    /// Enter raw mode: disable echo, canonical mode, signals, etc.
    pub fn enable_raw_mode() -> io::Result<()> {
        let fd = io::stdin().as_raw_fd();
        let mut termios: Termios = unsafe { mem::zeroed() };

        if unsafe { ioctl(fd, TCGETS, &mut termios as *mut Termios) } != 0 {
            return Err(io::Error::last_os_error());
        }

        // Save original for restore
        unsafe { ORIGINAL_TERMIOS = Some(termios); }

        // Raw mode flags (cfmakeraw equivalent)
        termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
        termios.c_oflag &= !OPOST;
        termios.c_cflag |= CS8;
        termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
        termios.c_cc[VMIN] = 0;  // Non-blocking
        termios.c_cc[VTIME] = 1; // 100ms timeout

        if unsafe { ioctl(fd, TCSETS, &termios as *const Termios) } != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Restore original terminal mode
    pub fn disable_raw_mode() -> io::Result<()> {
        let fd = io::stdin().as_raw_fd();
        if let Some(termios) = unsafe { ORIGINAL_TERMIOS } {
            if unsafe { ioctl(fd, TCSETS, &termios as *const Termios) } != 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    /// Get terminal size via ioctl TIOCGWINSZ
    pub fn size() -> (u16, u16) {
        let fd = io::stdout().as_raw_fd();
        let mut ws: Winsize = unsafe { mem::zeroed() };

        if unsafe { ioctl(fd, TIOCGWINSZ, &mut ws as *mut Winsize) } == 0
            && ws.ws_row > 0
            && ws.ws_col > 0
        {
            (ws.ws_row, ws.ws_col)
        } else {
            (24, 80) // Fallback
        }
    }

    /// Clear the entire screen
    pub fn clear() -> io::Result<()> {
        print!("\x1B[2J");
        Self::flush()
    }

    /// Move cursor to position (row, col) - 1-based
    pub fn move_to(row: u16, col: u16) -> io::Result<()> {
        print!("\x1B[{};{}H", row, col);
        Self::flush()
    }

    /// Hide cursor
    pub fn hide_cursor() -> io::Result<()> {
        print!("\x1B[?25l");
        Self::flush()
    }

    /// Show cursor
    pub fn show_cursor() -> io::Result<()> {
        print!("\x1B[?25h");
        Self::flush()
    }

    /// Reset all styles
    pub fn reset() -> io::Result<()> {
        print!("\x1B[0m");
        Self::flush()
    }

    /// Flush stdout
    pub fn flush() -> io::Result<()> {
        io::stdout().flush()
    }

    /// Enter alternate screen buffer
    pub fn enter_alternate() -> io::Result<()> {
        print!("\x1B[?1049h");
        Self::flush()
    }

    /// Exit alternate screen buffer
    pub fn exit_alternate() -> io::Result<()> {
        print!("\x1B[?1049l");
        Self::flush()
    }

    /// Read a single byte from stdin (non-blocking with raw mode)
    pub fn read_byte() -> io::Result<Option<u8>> {
        let mut buf = [0u8; 1];
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read(&mut buf) {
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(buf[0])),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e),
        }
    }
}
