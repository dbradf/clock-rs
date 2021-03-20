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
    let mut prev_squares = 0;
    let mut prev_cols = 0;
    let mut prev_rows = 0;


    loop {
        let (cols, rows) = terminal::size().unwrap();
        let total_size = cols * rows;
        let now = Local::now();
        let squares = time_to_squares(now, total_size);

        if squares != prev_squares || cols != prev_cols || rows != prev_rows {
            display_time(&stdout, squares, cols);
            prev_squares = squares;
            prev_cols = cols;
            prev_rows = rows;
        }
            thread::sleep(time::Duration::from_millis(100));
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
    stdout.queue(cursor::MoveTo(0, 0)).unwrap();

    let display_rows = squares / cols;
    let full_line = "█".repeat(cols as usize);
    for _ in 0..display_rows - 1 {
        stdout.queue(style::PrintStyledContent(full_line.to_string().yellow())).unwrap();
    }

    let partial_line = "█".repeat((squares % cols) as usize);
    stdout.queue(style::PrintStyledContent(partial_line.yellow())).unwrap();

    stdout.flush().unwrap();
}
