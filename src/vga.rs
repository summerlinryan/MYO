use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// repr(C) so that the memory of the Character type is laid out exactly how it looks
/// (i.e. ascii_character first and color_code next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Character {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

/// Lazy static to initialize the Writer on its first usage and Mutex because we want to avoid
/// multiple threads trying to write to the VGA buffer at the same time.
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        row_position: 0,
        column_position: 0,
        color_code: ColorCode::new(Color::LightCyan, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// repr(transparent) so that this Buffer type is treated just like the multidimensional
/// character array that it contains.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    buffer: &'static mut Buffer,
    color_code: ColorCode,
    row_position: usize,
    column_position: usize,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                self.buffer.chars[self.row_position][self.column_position].write(Character {
                    ascii_character: byte,
                    color_code: self.color_code,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // We can handle printable (0x20..=0x7e) and LF (\n newline) characters.
                0x20..=0x7e | 0x0a => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        self.column_position = 0;
        self.row_position += 1;
        let last_row_index = BUFFER_HEIGHT - 1;

        // We passed the end of the visible screen region so we need to shift all rows up and clear
        // the last row.
        if self.row_position > last_row_index {
            self.row_position = last_row_index;
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(last_row_index);
        }
    }

    pub fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(Character {
                color_code: self.color_code,
                ascii_character: b' ',
            })
        }
    }

    pub fn set_position(&mut self, row: usize, col: usize) {
        self.row_position = row;
        self.column_position = col;
    }
}

/// Gives us the ability to use write! with our VGA Writer.
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Marked as pub so that the calls to println! and print! outside of this module can still
/// invoke it. Marked as doc(hidden) though and prefixed with underscore so that calling code
/// outside of this module knows that it shouldn't use this function directly.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::vga::{Writer, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};
    use crate::{print, println};
    use crate::{serial_print, serial_println};

    #[test_case]
    fn println_no_panic() {
        WRITER.lock().set_position(0, 0);
        println!("test");
    }

    #[test_case]
    fn println_off_screen_no_panic() {
        WRITER.lock().set_position(0, 0);
        for _ in 0..BUFFER_HEIGHT + 1 {
            println!("test")
        }
    }

    #[test_case]
    fn println_non_ascii_character_no_panic() {
        WRITER.lock().set_position(0, 0);
        println!("ö");
    }

    #[test_case]
    fn println_output() {
        WRITER.lock().set_position(0, 0);
        let s = "Some test string that fits on a single line";
        print!("{}", s);
        for (index, char) in s.chars().enumerate() {
            let vga_character = WRITER.lock().buffer.chars[0][index].read();
            assert_eq!(char::from(vga_character.ascii_character), char);
        }
    }
}
