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
    let clr: Box<Fn() -> f32> = Box::new(move || mem_info.avail as f32 / one_slot as f32);
    LabelledBar::new("memory : ".to_string(), clr)
}

fn swap_bar(swap_info:SwapInfo) -> LabelledBar {
    let one_slot = swap_info.total / 100;
    let clr: Box<Fn() -> f32> = Box::new(move || swap_info.free as f32 / one_slot as f32);
    LabelledBar::new("swap   : ".to_string(), clr)
}

fn disk_bar(disk_info:DiskInfo) -> LabelledBar {
    let one_slot = disk_info.total / 100;
    let clr: Box<Fn() -> f32> = Box::new(move || disk_info.free as f32 / one_slot as f32);
    LabelledBar::new("disk   : ".to_string(), clr)
}

//fn cpu_bar(cpu_info:CpuInfo) -> LabelledBar {
//    let one_slot = cpu_info.total / 100;
//    let clr: Box<Fn() -> u64> = Box::new(move || {
//        let nr_slots = disk_info.free / one_slot;
//        100 - nr_slots
//    });
//    LabelledBar::new("disk (used/free)  : ".to_string(), clr)
//}

fn print_cpu<W: Write>(stdout: &mut W, cpu_info:CpuInfo, load_info:LoadInfo) {

    let one_color =
        if load_info.one > cpu_info.num as f64 {termion::color::Rgb(255, 0, 0)}
        else if load_info.one >= (cpu_info.num as f64 * 0.8){termion::color::Rgb(255, 255, 0)}
        else {termion::color::Rgb(0, 255, 0)};

    let five_color =
        if load_info.five > cpu_info.num as f64 {termion::color::Rgb(255, 0, 0)}
        else if load_info.five >= (cpu_info.num as f64 * 0.8){termion::color::Rgb(255, 255, 0)}
        else {termion::color::Rgb(0, 255, 0)};

    let fiveteen_color =
        if load_info.fiveteen > cpu_info.num as f64 {termion::color::Rgb(255, 0, 0)}
        else if load_info.fiveteen >= (cpu_info.num as f64 * 0.8){termion::color::Rgb(255, 255, 0)}
        else {termion::color::Rgb(0, 255, 0)};


    write!(stdout,
            "{}[ #CPU: {} ][ spd: {} ][ 1: {}{:.2}{} 5: {}{:.2}{} 15: {}{:.2}{} ]",
            termion::color::Fg(termion::color::Rgb(255, 255, 255)),
            cpu_info.num,
            cpu_info.speed,
            termion::color::Fg(one_color),
            load_info.one,
            termion::color::Fg(termion::color::Rgb(255, 255, 255)),
            termion::color::Fg(five_color),
            load_info.five,
            termion::color::Fg(termion::color::Rgb(255, 255, 255)),
            termion::color::Fg(fiveteen_color),
            load_info.fiveteen,
            termion::color::Fg(termion::color::Rgb(255, 255, 255)))
        .unwrap();
    write!(stdout, "\n\r").unwrap();
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
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
        let (width,height) = termion::terminal_size().unwrap();

        print_cpu(&mut stdout, sysinfo.cpu_info, sysinfo.load_info);
        memory_bar(sysinfo.mem_info).update(&mut stdout, width);
        disk_bar(sysinfo.disk_info).update(&mut stdout, width);
//        cpu_bar(sysinfo.cpu_info).update(&mut stdout);
        write!(stdout, "{} ", termion::style::Reset).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Got it! Exiting...");

}