# AI System Terminal Animation

AI System Terminal Animation is a Rust-based terminal animation simulating an AI system initialization, memory defragmentation, and reboot process.

## Features

- Realistic AI system simulation with glitch effects
- Memory defragmentation process with sector-by-sector visualization
- Performance script for easy customization of the animation sequence
- Cross-platform support, works on Windows, macOS, and Linux

## Installation

1. Install Rust programming language: https://www.rust-lang.org/tools/install
2. Clone this repository: `git clone git@github.com:DieHard073055/ai-system-terminal-animation.git`
3. Change directory: `cd ai-system-terminal-animation`

## Usage

1. Compile the project: `cargo build --release`
2. create a `script.txt` with one of the supported commands (eg: `echo "print_message hello world" > script.txt` )
2. Run the compiled binary: `./target/release/ai_system_terminal_animation script.txt`

## Customization

Edit the `script.txt` file to change the animation sequence or add new steps to the simulation. Use the provided commands and parameters to create a unique experience.

### Supported Commands

- `sleep [milliseconds]`: Pauses the script execution for the specified duration in milliseconds.
- `print_message [message]`: Prints the specified message with a typewriter effect.
- `loading_screen`: Displays a loading screen animation.
- `defragmentation_animation [input_memory_file] [steps] [output_memory_file]`: Runs the memory defragmentation animation with the specified input memory file, number of steps, and output memory file.
- `clear`: Clears the terminal screen.
- `file [filename]`: Reads and prints the contents of the specified file line by line.

## License

This project is licensed under the Apache 2.0. [LICENSE](LICENSE)
