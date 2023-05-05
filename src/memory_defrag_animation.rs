use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{self, Clear};
use crossterm::{ExecutableCommand, QueueableCommand};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::fs::File;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

const SWAP_ITERATIONS: usize = 10;

pub fn defragmentation_animation(memory_file_1: &str, steps: usize, memory_file_2: &str) {
    let mut visited_node = HashMap::new();
    let mut unvisited_nodes: VecDeque<(usize, usize)> = vec![(1, 1)].into();
    let maze_string = fs::read_to_string(memory_file_1).expect("Failed to read maze.txt");

    let mut maze: Vec<Vec<char>> = maze_string
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let random_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let mut stdout = stdout();
    stdout.execute(Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(Hide).unwrap();

    let mut row;
    let mut col;
    let mut spaces_cleared = 0;

    while spaces_cleared < steps {
        if let Some(node) = unvisited_nodes.pop_front() {
            (row, col) = node;
            visited_node.entry((row, col)).or_insert(0);
        } else {
            break;
        }
        if maze[row][col] != ' ' && maze[row][col] != '▓' {
            // Swap the character with random characters
            for _ in 0..SWAP_ITERATIONS {
                maze[row][col] = random_chars
                    .chars()
                    .nth(rand::random::<usize>() % random_chars.chars().count())
                    .unwrap();
                draw_maze(&maze, &mut visited_node, &row, &col, &mut stdout);
            }

            // Change the character to whitespace
            maze[row][col] = ' ';
            draw_maze(&maze, &mut visited_node, &row, &col, &mut stdout);
            spaces_cleared += 1;
        }

        // Move to the next corrupted character
        // check left
        if col > 0 && maze[row][col - 1] != '▓' && !visited_node.contains_key(&(row, col - 1)) {
            unvisited_nodes.push_back((row, col - 1));
        }
        // up
        if row > 0 && maze[row - 1][col] != '▓' && !visited_node.contains_key(&(row - 1, col)) {
            unvisited_nodes.push_back((row - 1, col));
        }
        // right
        if col + 1 < maze[row].len()
            && maze[row][col + 1] != '▓'
            && !visited_node.contains_key(&(row, col + 1))
        {
            unvisited_nodes.push_back((row, col + 1));
        }
        // down
        if row + 1 < maze.len()
            && maze[row + 1][col] != '▓'
            && !visited_node.contains_key(&(row + 1, col))
        {
            unvisited_nodes.push_back((row + 1, col));
        }
    }

    stdout.execute(Show).unwrap();
    let mut file = File::create(memory_file_2).unwrap();

    // Write the maze contents to the file.
    file.write_all(
        maze.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap();

    // Close the file (not strictly necessary, but a good practice).
    file.flush().unwrap();
}

fn draw_maze(
    maze: &[Vec<char>],
    visited_node: &mut HashMap<(usize, usize), i32>,
    row: &usize,
    col: &usize,
    stdout: &mut impl Write,
) {
    stdout.queue(MoveTo(0, 0)).unwrap();
    println!("Defragmentation progress: {:}%", visited_node.len());
    execute!(stdout, crossterm::cursor::MoveLeft(100)).unwrap();
    println!("Memory sectors fixed: {:}", visited_node.len() * 8);
    execute!(stdout, crossterm::cursor::MoveLeft(100)).unwrap();
    println!("Memory sectors remaining: 2#&!");
    execute!(stdout, crossterm::cursor::MoveLeft(100)).unwrap();
    println!("Current memory sector: ({:}, {:})", row, col);
    execute!(stdout, crossterm::cursor::MoveLeft(100)).unwrap();
    println!("Time elapsed: 0*:#!$");
    execute!(stdout, crossterm::cursor::MoveLeft(100)).unwrap();
    println!("Estimated time remaining: 0@:%$!");
    execute!(stdout, crossterm::cursor::MoveLeft(100)).unwrap();
    for row in maze {
        for ch in row {
            stdout.queue(Print(*ch)).unwrap();
        }
        stdout.queue(Print('\n')).unwrap();
        execute!(stdout, crossterm::cursor::MoveLeft(100)).unwrap();
    }

    stdout.flush().unwrap();
    visited_node.entry((*row, *col)).and_modify(|e| *e += 1);
    if visited_node.get(&(*row, *col)).unwrap() < &10 {
        thread::sleep(Duration::from_millis(100));
    }
}
