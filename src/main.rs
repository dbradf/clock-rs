use std::io::{Stdout, Write, stdout};
use crossterm::{QueueableCommand, cursor, style::Colorize, style, terminal};
use chrono::{Timelike, Local, DateTime};
use std::{thread, time};

fn main() {
    ctrlc::set_handler(move || {
        let mut stdout = stdout();
        stdout.queue(cursor::Show).unwrap();
        std::process::exit(0);
    }).unwrap();

    let mut stdout = stdout();
    stdout.queue(cursor::Hide).unwrap();

    loop {
        let (cols, rows) = terminal::size().unwrap();
        let total_size = cols * rows;
        let now = Local::now();
        let squares = time_to_squares(now, total_size);

        display_time(&stdout, squares, cols);
        thread::sleep(time::Duration::from_secs(1));
    }
}


fn time_to_squares(time: DateTime<Local>, total_size: u16) -> u16 {
    const MINUTES_PER_DAY: u16 = 24 * 60;
    let hour = time.hour() as u16;
    let minute = time.minute() as u16;
    let minute_of_day = hour * 60 + minute;

    let total_units = minute_of_day as u32 * total_size as u32;

    (total_units / MINUTES_PER_DAY as u32) as u16
}

fn display_time(mut stdout: &Stdout, squares: u16, cols: u16) {
    stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();

    // println!("{}, {}", squares, cols);

    for s in 0..squares {
        let x = s % cols; 
        let y = s / cols;

        stdout.queue(cursor::MoveTo(x, y)).unwrap();
        stdout.queue(style::PrintStyledContent("█".yellow())).unwrap();
    }
    stdout.flush().unwrap();
}
