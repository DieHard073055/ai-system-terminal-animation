use crossterm::cursor::MoveTo;
use crossterm::terminal::{self, Clear};
use crossterm::QueueableCommand;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use first::memory_defrag_animation::defragmentation_animation;
use std::env;
use std::fs;
use std::{
    fs::File,
    io::{prelude::*, stdout, BufReader, Write},
    thread,
    time::Duration,
};

fn loading_screen(stdout: &mut impl Write) {
    let spinner_characters = vec!['|', '/', '-', '\\'];
    let spinner_duration = Duration::from_millis(100);

    // Set the spinner color
    stdout.execute(SetForegroundColor(Color::Cyan)).unwrap();

    for _ in 0..10 {
        for &spinner_char in &spinner_characters {
            // Print the spinner character and flush stdout
            stdout.execute(Print(spinner_char)).unwrap();
            stdout.flush().unwrap();

            // Sleep for the specified duration
            thread::sleep(spinner_duration);
            // Move the cursor back one position
            execute!(stdout, crossterm::cursor::MoveLeft(100)).unwrap();
        }
    }
    // Reset the terminal color and disable raw mode
    stdout.execute(ResetColor).unwrap();
}

fn sleep(milliseconds: u64) {
    thread::sleep(Duration::from_millis(milliseconds));
}

fn print_message(stdout: &mut impl Write, message: &str, speed: u64) {
    let cursor_duration = Duration::from_millis(speed);
    let letters = message.to_string().chars().collect::<Vec<char>>();
    for letter in letters {
        // Sleep for the specified duration
        thread::sleep(cursor_duration);
        print!("{letter}");
        stdout.flush().unwrap();
    }
    println!();
    // Move the cursor back one position
    execute!(stdout, crossterm::cursor::MoveLeft(100)).unwrap();
}

fn execute_script_line(line: &str, stdout: &mut impl Write) {
    let command_parts: Vec<&str> = line.split_whitespace().collect();

    if command_parts.is_empty() {
        return;
    }

    match command_parts[0] {
        "sleep" => {
            if command_parts.len() >= 2 {
                if let Ok(duration) = command_parts[1].parse() {
                    sleep(duration);
                }
            }
        }
        "print_message" => {
            let (speed, message_parts) =
                if command_parts.len() >= 3 && command_parts[1].starts_with(';') {
                    (
                        command_parts[1][1..].parse().unwrap_or(50),
                        &command_parts[2..],
                    )
                } else {
                    (50, &command_parts[1..])
                };
            let message = message_parts.join(" ");
            print_message(stdout, &message, speed);
        }
        "loading_screen" => {
            loading_screen(stdout);
        }
        "defragmentation_animation" => {
            if command_parts.len() >= 5 {
                match (command_parts[2].parse(), command_parts[3].parse()) {
                    (Ok(steps), Ok(speed)) => {
                        let current_memory_file = command_parts[1];
                        let output_memory_file = command_parts[3];
                        thread::sleep(Duration::from_millis(5000));
                        defragmentation_animation(
                            &current_memory_file,
                            steps,
                            speed,
                            output_memory_file,
                        );
                    }
                    _ => {}
                }
            }
        }
        "clear" => {
            stdout.execute(Clear(terminal::ClearType::All)).unwrap();
            stdout.queue(MoveTo(0, 0)).unwrap();
        }
        "file" => {
            let filename = command_parts[1];
            let file_string = fs::read_to_string(filename)
                .expect(format!("Failed to read {:}", filename).as_str());
            execute!(stdout, crossterm::cursor::MoveLeft(100)).unwrap();
            for line in file_string.split('\n') {
                print_message(stdout, &line, 50);
                execute!(stdout, crossterm::cursor::MoveLeft(100)).unwrap();
            }
        }
        _ => {}
    }
}

fn main() {
    let mut stdout = stdout();

    // Enable raw mode to have direct control over the terminal
    enable_raw_mode().unwrap();

    let args: Vec<String> = env::args().collect();
    let script_filename = if args.len() > 1 {
        &args[1]
    } else {
        panic!("Please provide a script file as an argument.");
    };

    let script_file = File::open(script_filename).expect("Unable to open script file");
    let reader = BufReader::new(script_file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line from script file");
        execute_script_line(&line, &mut stdout);
    }

    // Reset the terminal color and disable raw mode
    stdout.execute(ResetColor).unwrap();
    disable_raw_mode().unwrap();
}
