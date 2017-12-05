//! UI elements for my top.
//!
//! # Example
//!
//! ```rust
//! mod display_elements;
//! use display_elements::*;
//!
//! fn main() {
//!     let one_slot = mem_info.total / 100;
//!     let clr: Box<Fn() -> f32> = Box::new(move || mem_info.avail as f32 / one_slot as f32);
//!     LabelledBar::new("memory : ".to_string(), clr)
//! }
//! ```

extern crate termion;
use termion::color;

pub const YELLOW:color::Rgb = color::Rgb(255, 255, 0);
pub const RED:color::Rgb = color::Rgb(255, 0, 0);
pub const GREEN:color::Rgb = color::Rgb(0, 255, 0);
pub const WHITE:color::Rgb = color::Rgb(255, 255, 255);

use std::io::{Write};
fn draw_bar<W: Write>(stdout: &mut W, free:f32, width:u16) {

    let taken: u16 = 21; //amount of space used by static content, should be more dynamic ...

    write!(stdout,
           "{}[ ",
           color::Fg(WHITE))
        .unwrap();

    if width > taken {
        let scale = (width - taken) as u32;

        let used:u32 = scale - (free / 100.0 * scale as f32) as u32;

        for _ in 0..used {
            write!(stdout,
                   "{}|",
                   color::Fg(RED))
                .unwrap();
        }

        for _ in used..scale {
            write!(stdout,
                   "{}|",
                   color::Fg(GREEN))
                .unwrap();
        }
    }

    write!(stdout,
           "{} {:.2} % ]",
           color::Fg(WHITE),
           100.0 - free )
        .unwrap();
}

pub struct LabelledBar {
    used: Option<f32>,
    used_fn: Box<Fn() -> f32>,
    label: String
}

impl LabelledBar {
    pub fn new(label: String, used_fn: Box<Fn() -> f32>) -> LabelledBar {
        LabelledBar {
            label: label,
            used: None,
            used_fn: used_fn
        }
    }

    pub fn update<W: Write>(&mut self, stdout: &mut W, width:u16) {
        let new_used = Some((self.used_fn)());
        if self.used != new_used {
            self.used = new_used;
            write!(stdout, "{}{}", termion::style::Reset, self.label).unwrap();
            draw_bar(stdout, self.used.unwrap(), width);
            write!(stdout, "\n\r").unwrap();
        }
    }
}
