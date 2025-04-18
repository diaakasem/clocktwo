use chrono::Timelike;
use chrono::{DateTime, Local};
use number_to_words::number_to_words;
use rand::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use termion::style::{Bold, Reset};

/// Enum representing the different types of time words.
enum TimeWords {
    Hour,
    Minute,
    AMPM,
    Others,
}

/// Renders the given character with styling based on the type of time word.
///
/// # Arguments
///
/// * `text` - The character to render.
/// * `time_words` - The type of time word to determine the styling.
fn render_text(text: char, time_words: TimeWords) {
    let white = termion::color::Fg(termion::color::White);
    let red = termion::color::Fg(termion::color::Red);
    match time_words {
        TimeWords::Hour => {
            print!("{}{}{}{}", Bold, red, text, Reset);
        }
        TimeWords::Minute => {
            print!("{}{}{}{}", Bold, red, text, Reset);
        }
        TimeWords::AMPM => {
            print!("{}{}{}{}", Bold, red, text, Reset);
        }
        TimeWords::Others => {
            print!("{}{}{}", white, text, Reset);
        }
    }
}

/// Main function that continuously updates and displays the current time.
fn main() {
    let cycle_time = 1;
    let mut first = true;
    let mut last_time: DateTime<Local> = Local::now();
    loop {
        let now: DateTime<Local> = Local::now();
        if now.minute() != last_time.minute() || first {
            first = false;
            print_time(now);
            last_time = now;
        }
        sleep(Duration::from_secs(cycle_time));
    }
}

/// Prints the current time in a grid format with styled text.
///
/// # Arguments
///
/// * `now` - The current date and time.
fn print_time(now: DateTime<Local>) {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    let mut hour = now.hour() as u32;
    let minute = now.minute() as u32;
    let am_pm = if hour < 12 { "AM" } else { "PM" };

    if hour > 12 {
        hour -= 12;
    }

    let hour_text = number_to_words(hour, false).to_uppercase();
    let minute_text = number_to_words(minute, false).to_uppercase();

    let alphabets = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut rendering: TimeWords = TimeWords::Others;
    let grid_size = 14;
    let max_len = hour_text.len().max(minute_text.len());
    let mut random_start_row = thread_rng().gen_range(0..(grid_size as f32 / 3 as f32) as usize);
    let mut random_start = thread_rng().gen_range(0..(grid_size - max_len));
    let mut is_done: [u8; 3] = [0; 3];
    for i in 0..grid_size {
        for j in 0..grid_size {
            if is_done[0] > 0 && is_done[0] < hour_text.len() as u8 {
                rendering = TimeWords::Hour;
            } else if is_done[1] > 0 && is_done[1] < minute_text.len() as u8 {
                rendering = TimeWords::Minute;
            } else if is_done[2] > 0 && is_done[2] < 2 {
                rendering = TimeWords::AMPM;
            } else if i == random_start_row && j == random_start {
                if is_done[0] == 0 {
                    rendering = TimeWords::Hour;
                } else if is_done[1] == 0 {
                    rendering = TimeWords::Minute;
                } else if is_done[2] == 0 {
                    rendering = TimeWords::AMPM;
                }
            }

            match rendering {
                TimeWords::Hour => {
                    if is_done[0] < hour_text.len() as u8 {
                        let letter = hour_text.chars().nth(is_done[0] as usize).unwrap();
                        render_text(letter, TimeWords::Hour);
                        is_done[0] += 1;
                    } else {
                        random_start = thread_rng().gen_range(0..(grid_size - max_len));
                        random_start_row = thread_rng().gen_range(
                            (random_start_row + 1)
                                ..(grid_size - (grid_size as f32 / 3 as f32) as usize) - 1,
                        );
                        rendering = TimeWords::Others;
                    }
                }
                TimeWords::Minute => {
                    if is_done[1] < minute_text.len() as u8 {
                        let letter = minute_text.chars().nth(is_done[1] as usize).unwrap();
                        render_text(letter, TimeWords::Minute);
                        is_done[1] += 1;
                    } else {
                        random_start = thread_rng().gen_range(0..(grid_size - max_len));
                        random_start_row =
                            thread_rng().gen_range((random_start_row + 1)..grid_size);
                        rendering = TimeWords::Others;
                    }
                }
                TimeWords::AMPM => {
                    if is_done[2] < 2 {
                        let letter = am_pm.chars().nth(is_done[2] as usize).unwrap();
                        render_text(letter, TimeWords::AMPM);
                        is_done[2] += 1;
                    } else {
                        rendering = TimeWords::Others;
                    }
                }
                _ => {}
            }
            match rendering {
                TimeWords::Others => {
                    let letter = alphabets
                        .chars()
                        .nth(thread_rng().gen_range(0..alphabets.len()))
                        .unwrap();
                    render_text(letter, TimeWords::Others);
                }
                _ => {}
            }
            print!(" ");
        }
        println!("");
    }
}

