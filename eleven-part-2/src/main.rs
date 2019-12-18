mod intcode;

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

fn main() {
    let program = vec![3,8,1005,8,320,1106,0,11,0,0,0,104,1,104,0,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,102,1,8,29,2,1005,1,10,1006,0,11,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,102,1,8,57,1,8,15,10,1006,0,79,1,6,3,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,101,0,8,90,2,103,18,10,1006,0,3,2,105,14,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,123,2,9,2,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1001,8,0,150,1,2,2,10,2,1009,6,10,1,1006,12,10,1006,0,81,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,102,1,8,187,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,101,0,8,209,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,231,1,1008,11,10,1,1001,4,10,2,1104,18,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1001,8,0,264,1,8,14,10,1006,0,36,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,293,1006,0,80,1006,0,68,101,1,9,9,1007,9,960,10,1005,10,15,99,109,642,104,0,104,1,21102,1,846914232732,1,21102,1,337,0,1105,1,441,21102,1,387512115980,1,21101,348,0,0,1106,0,441,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,209533824219,1,1,21102,1,395,0,1106,0,441,21101,0,21477985303,1,21102,406,1,0,1106,0,441,3,10,104,0,104,0,3,10,104,0,104,0,21101,868494234468,0,1,21101,429,0,0,1106,0,441,21102,838429471080,1,1,21102,1,440,0,1106,0,441,99,109,2,21201,-1,0,1,21101,0,40,2,21102,472,1,3,21101,0,462,0,1106,0,505,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,467,468,483,4,0,1001,467,1,467,108,4,467,10,1006,10,499,1102,1,0,467,109,-2,2106,0,0,0,109,4,2101,0,-1,504,1207,-3,0,10,1006,10,522,21101,0,0,-3,21202,-3,1,1,22101,0,-2,2,21102,1,1,3,21102,541,1,0,1106,0,546,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,569,2207,-4,-2,10,1006,10,569,22102,1,-4,-4,1105,1,637,22102,1,-4,1,21201,-3,-1,2,21202,-2,2,3,21102,588,1,0,1105,1,546,22101,0,1,-4,21102,1,1,-1,2207,-4,-2,10,1006,10,607,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,629,21201,-1,0,1,21102,629,1,0,105,1,504,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0];

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
