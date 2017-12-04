
pub mod display_elements {

    extern crate termion;
    use std::io::{Write};
    fn draw_bar<W: Write>(stdout: &mut W, used:u64) {

        for _ in 0..used {
            write!(stdout,
                   "{} ",
                   termion::color::Bg(termion::color::Rgb(255, 0, 0)))
                .unwrap();
        }

        for _ in used..100 {
            write!(stdout,
                   "{} ",
                   termion::color::Bg(termion::color::Rgb(0, 255, 0)))
                .unwrap();
        }
    }

    pub struct LabelledBar {
        used: Option<u64>,
        used_fn: Box<Fn() -> u64>,
        label: String
    }

    impl LabelledBar {
        pub fn new(label: String, used_fn: Box<Fn() -> u64>) -> LabelledBar {
            LabelledBar {
                label: label,
                used: None,
                used_fn: used_fn
            }
        }


        pub fn update<W: Write>(&mut self, stdout: &mut W) {
            let new_used = Some((self.used_fn)());
            if self.used != new_used {
                self.used = new_used;
                write!(stdout, "{}{}", termion::style::Reset, self.label).unwrap();
                draw_bar(stdout, self.used.unwrap());
                write!(stdout, "\n\r").unwrap();
            }
        }
    }

}