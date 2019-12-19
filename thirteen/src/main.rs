mod intcode;

use std::collections::HashMap;
use std::fs;
use std::sync::mpsc;
use std::thread;

fn main() {
    let mut input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let program: Vec<i64> = input.trim_end().split(",").map(|n| n.parse().unwrap()).collect();

    let panels = painted_panels(program);
    print_panels(panels);
}

fn print_panels(panels: HashMap<Position, Color>) {
    let x_min = panels.keys().min_by_key(|&p| p.x).unwrap().x;
    let x_max = panels.keys().max_by_key(|&p| p.x).unwrap().x;
    let y_min = panels.keys().min_by_key(|&p| p.y).unwrap().y;
    let y_max = panels.keys().max_by_key(|&p| p.y).unwrap().y;

    // Reverse y in the range so we can start printing the highest y values first
    for negative_y in (-y_max)..(-y_min + 1) {
        let y = -negative_y;

        for x in x_min..(x_max + 1) {
            let color = panels.get(&Position { x, y }).unwrap_or(&Black);
            let output = match color {
                Black => ".",
                White => "w",
            };
            print!("{}", output);
        }
        print!("\n");
    }
}

fn painted_panels(program: Vec<i64>) -> HashMap<Position, Color> {
    let mut robot_direction = Direction::Up;
    let mut robot_position = Position { x: 0, y: 0 };
    let mut painted_panels: HashMap<Position, Color> = HashMap::new();

    // - Open channel to send input from main thread to robot thread
    let (robot_output, main_input) = mpsc::channel();

    // - Open channel to receive output from robot thread in main thread
    let (main_output, robot_input) = mpsc::channel();

    thread::spawn(move || {
      intcode::run_intcode_with_channels(program, robot_input, robot_output);
    });

    // - Provide input of 1 to program (representing 0,0 white panel)
    main_output.send(1).unwrap();

    loop {
        let paint_color_int = main_input.recv().unwrap();
        // 99 means we're done
        if paint_color_int == 99 { break; }

        // Paint the panel
        let paint_color =
            if paint_color_int == 0 {
                Color::Black
            } else {
                Color::White
            };
        painted_panels.insert(robot_position, paint_color);

        let direction_to_turn_int = main_input.recv().unwrap();
        let direction_to_turn =
            if direction_to_turn_int == 0 {
                Direction::Left
            } else {
                Direction::Right
            };

        robot_direction = robot_direction.turn(direction_to_turn);
        robot_position = robot_position.move_in_direction(robot_direction);

        // Unpainted panels are black
        let panel_color = painted_panels.get(&robot_position).unwrap_or(&Black);
        let panel_color_int =
            match panel_color {
                Black => 0,
                White => 1,
            };

        // Send the color of this position to the robot
        main_output.send(panel_color_int).unwrap();
    }

    return painted_panels;
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

impl Direction {
    fn turn(&self, direction_to_turn: Direction) -> Direction {
        return match direction_to_turn {
            Left => {
                match self {
                    Up => Left,
                    Left => Down,
                    Down => Right,
                    Right => Up,
                }
            },
            Right => {
                match self {
                    Up => Right,
                    Right => Down,
                    Down => Left,
                    Left => Up,
                }
            },
            d => panic!("Can't turn in direction: {:?}", d)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    // Turn in direction, then move one step in that direction
    fn move_in_direction(&self, direction: Direction) -> Position {
        return match direction {
            Up => Position { x: self.x, y: self.y + 1 },
            Right => Position { x: self.x + 1, y: self.y },
            Down => Position { x: self.x, y: self.y - 1 },
            Left => Position { x: self.x - 1, y: self.y },
        }
    }
}

#[derive(Debug)]
enum Color {
    Black,
    White,
}

use Color::*;
