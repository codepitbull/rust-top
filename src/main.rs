extern crate sys_info;

use std::sync::atomic::{AtomicBool, Ordering};
use std::io::{Write, stdout, stdin};
use std::time::Duration;
use std::sync::Arc;
use std::thread;

extern crate termion;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::color;
use termion::event::{Key, Event};

mod display_elements;
use display_elements::*;

mod rust_top;
use rust_top::rust_top::*;


fn memory_bar(mem_info:MemInfo) -> LabelledBar {
    let one_slot = mem_info.total / 100;
    let clr: Box<Fn() -> f32> = Box::new(move || mem_info.avail as f32 / one_slot as f32);
    LabelledBar::new("memory : ".to_string(), clr)
}

fn disk_bar(disk_info:DiskInfo) -> LabelledBar {
    let one_slot = disk_info.total / 100;
    let clr: Box<Fn() -> f32> = Box::new(move || disk_info.free as f32 / one_slot as f32);
    LabelledBar::new("disk   : ".to_string(), clr)
}

fn print_cpu<W: Write>(stdout: &mut W, cpu_info:CpuInfo, load_info:LoadInfo) {

    let one_color =
        if load_info.one > cpu_info.num as f64 { RED }
        else if load_info.one >= (cpu_info.num as f64 * 0.8){ YELLOW }
        else { GREEN };

    let five_color =
        if load_info.five > cpu_info.num as f64 { RED }
        else if load_info.five >= (cpu_info.num as f64 * 0.8){ YELLOW }
        else { GREEN };

    let fiveteen_color =
        if load_info.fiveteen > cpu_info.num as f64 { RED }
        else if load_info.fiveteen >= (cpu_info.num as f64 * 0.8){ YELLOW }
        else { GREEN };


    write!(stdout,
           "{}[ #CPU: {} ][ spd: {} ][ 1: {}{:.2}{} 5: {}{:.2}{} 15: {}{:.2}{} ]",
           color::Fg(WHITE),
           cpu_info.num,
           cpu_info.speed,
           color::Fg(one_color),
           load_info.one,
           color::Fg(WHITE),
           color::Fg(five_color),
           load_info.five,
           color::Fg(WHITE),
           color::Fg(fiveteen_color),
           load_info.fiveteen,
           color::Fg(WHITE))
        .unwrap();
    write!(stdout, "\n\r").unwrap();
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
             "{}{}{}",
             termion::clear::All,
             termion::cursor::Goto(1, 1),
             termion::cursor::Hide)
        .unwrap();

    let running = Arc::new(AtomicBool::new(true));
    let running_thread = running.clone();

    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Char('q')) => running_thread.store(false, Ordering::SeqCst),
                _ => {}
            }
        }
    });

    while running.load(Ordering::SeqCst) {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 1),
               termion::clear::All)
            .unwrap();

        let (width,_) = termion::terminal_size().unwrap();

        print_cpu(&mut stdout, CpuInfo::new().unwrap(), LoadInfo::new().unwrap());
        memory_bar(MemInfo::new().unwrap()).update(&mut stdout, width);
        disk_bar(DiskInfo::new().unwrap()).update(&mut stdout, width);
        write!(stdout, "{} ", termion::style::Reset).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    println!("Exiting...");
}