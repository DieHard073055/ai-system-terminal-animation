use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

fn main() {
    let mut stdout = stdout();

    // Enable raw mode to have direct control over the terminal
    enable_raw_mode().unwrap();

    let animations = vec![
        ("Loading…", "▒▒▒▒▒▒▒▒▒▒"),
        ("10%", "█▒▒▒▒▒▒▒▒▒"),
        ("30%", "███▒▒▒▒▒▒▒"),
        ("50%", "█████▒▒▒▒▒"),
        ("70%", "███████▒▒▒"),
        ("100%", "██████████"),
    ];
    let animation_duration = Duration::from_millis(2000);

    // Set the animation color
    stdout.execute(SetForegroundColor(Color::Green)).unwrap();

    execute!(stdout, Clear(ClearType::All)).unwrap();
    execute!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
    for _ in 0..1 {
        for animation_frame in &animations {
            // Print the animation frame and flush stdout
            stdout.execute(Print(animation_frame.0)).unwrap();
            execute!(stdout, crossterm::cursor::MoveTo(0, 1)).unwrap();
            stdout.execute(Print(animation_frame.1)).unwrap();
            stdout.flush().unwrap();

            // Sleep for the specified duration
            thread::sleep(animation_duration);

            // Clear the current frame
            execute!(stdout, Clear(ClearType::All)).unwrap();
            execute!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
        }
    }

    // Reset the terminal color and disable raw mode
    stdout.execute(ResetColor).unwrap();
    disable_raw_mode().unwrap();
}
