extern crate termion;
extern crate sys_info;
extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};
mod rust_top;
use rust_top::rust_top::*;
use std::time::Duration;
use std::thread;
mod display_elements;
use display_elements::display_elements::*;

fn memory_bar(mem_info:MemInfo) -> LabelledBar {
    let one_slot = mem_info.total / 100;
    let clr: Box<Fn() -> u64> = Box::new(move || {
        let nr_slots = mem_info.free / one_slot;
        100 - nr_slots
    });
    LabelledBar::new("memory (used/free): ".to_string(), clr)
}

fn swap_bar(swap_info:SwapInfo) -> LabelledBar {
    let one_slot = swap_info.total / 100;
    let clr: Box<Fn() -> u64> = Box::new(move || {
        let nr_slots = swap_info.free / one_slot;
        100 - nr_slots
    });
    LabelledBar::new("swap (used/free)  : ".to_string(), clr)
}

fn disk_bar(disk_info:DiskInfo) -> LabelledBar {
    let one_slot = disk_info.total / 100;
    let clr: Box<Fn() -> u64> = Box::new(move || {
        let nr_slots = disk_info.free / one_slot;
        100 - nr_slots
    });
    LabelledBar::new("disk (used/free)  : ".to_string(), clr)
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    writeln!(stdout,
             "{}{}{}Use the up/down arrow keys to change the blue in the rainbow.",
             termion::clear::All,
             termion::cursor::Goto(1, 1),
             termion::cursor::Hide)
        .unwrap();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 1),
               termion::clear::All)
            .unwrap();
        let sysinfo = SysInfo::new();
        writeln!(stdout, "{}", sysinfo.swap_info.total);
        memory_bar(sysinfo.mem_info).update(&mut stdout);
        disk_bar(sysinfo.disk_info).update(&mut stdout);
        write!(stdout, "{} ", termion::style::Reset).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
    println!("Got it! Exiting...");

}