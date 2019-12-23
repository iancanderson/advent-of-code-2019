mod intcode;

use std::collections::HashMap;
use std::fs;
use std::sync::mpsc;
use std::{thread, time};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let program: Vec<i64> = input.trim_end().split(",").map(|n| n.parse().unwrap()).collect();

    run_robot(program);
    // println!("Score: {}", score);
}

#[derive(Clone, Copy, Debug)]
enum MovementCommand {
    North,
    South,
    West,
    East,
}

impl MovementCommand {
    fn to_int(&self) -> i64 {
        match self {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }
}

use MovementCommand::*;

#[derive(Debug)]
enum RobotStatus {
    HitWall,
    HasMoved,
    HasMovedAndFoundOxygenSystem,
}

use RobotStatus::*;

impl RobotStatus {
    fn from_int(int: i64) -> RobotStatus {
        match int {
            0 => HitWall,
            1 => HasMoved,
            2 => HasMovedAndFoundOxygenSystem,
            n => panic!("Unknown RobotStatus {}", n),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn adjacent_points(&self) -> Vec<(MovementCommand, Point)> {
        vec![
            (North, Point { x: self.x, y: self.y + 1 }),
            (South, Point { x: self.x, y: self.y - 1 }),
            (West,  Point { x: self.x - 1, y: self.y }),
            (East,  Point { x: self.x + 1, y: self.y }),
        ]
    }

    fn direction_to(&self, adjacent_point: Point) -> MovementCommand {
        if adjacent_point.x > self.x {
            East
        } else if adjacent_point.x < self.x {
            West
        } else if adjacent_point.y > self.y {
            North
        } else {
            South
        }
    }
}

enum PointType {
    Wall,
    NoWall,
    OxygenSystem,
}

use PointType::*;

fn move_in_direction(point: Point, direction: MovementCommand) -> Point {
    match direction {
        North => Point { x: point.x, y: point.y + 1 },
        South => Point { x: point.x, y: point.y - 1 },
        West => Point { x: point.x - 1, y: point.y },
        East => Point { x: point.x + 1, y: point.y },
    }
}

fn print_map(map: &HashMap<Point, PointType>, robot_position: Point) {
    let min_x = map.keys().min_by_key(|p| p.x).unwrap().x;
    let max_x = map.keys().max_by_key(|p| p.x).unwrap().x;
    let min_y = map.keys().min_by_key(|p| p.y).unwrap().y;
    let max_y = map.keys().max_by_key(|p| p.y).unwrap().y;

    for neg_y in (-max_y)..=(-min_y) {
        let y = -neg_y;
        for x in min_x..=max_x {
            let current_point = Point { x, y };
            if let Point { x: 0, y: 0 } = current_point {
                print!("0");
            } else if robot_position == current_point {
                print!("D");
            } else {
                match map.get(&current_point) {
                    None => print!(" "),
                    Some(Wall) => print!("#"),
                    Some(NoWall) => print!("."),
                    Some(OxygenSystem) => print!("o"),
                }
            }
        }
        println!("");
    }
    println!("");
}

fn run_robot(program: Vec<i64>) -> i64 {
    // Open channel to send input from main thread to game thread
    let (main_output, robot_input) = mpsc::channel();

    // Open channel to receive output from game thread in main thread
    let (robot_output, main_input) = mpsc::channel();

    let robot_thread = thread::spawn(move || {
      intcode::run_intcode_with_channels(program, robot_input, robot_output);
    });

    let mut robot_position = Point { x: 0, y: 0 };
    let mut robot_direction = North;
    let mut robot_status = HasMoved;
    let mut map: HashMap<Point, PointType> = HashMap::new();

    // represents the path from origin
    let mut path = vec![robot_position];
    let mut shortest_path_length_to_oxygen = None;

    loop {
        // thread::sleep(time::Duration::from_millis(40));

        // Look for an adjacent unexplored point
        robot_direction =
            match robot_position.adjacent_points().iter().find(|(_, p)| !map.contains_key(p)) {
                None => {
                    // We've reached a dead end: time to go back
                    if let None = path.pop() {
                        println!("done");
                        break;
                    }
                    if let Some(last_position) = path.pop() {
                        let direction = robot_position.direction_to(last_position);
                        direction
                    } else {
                        println!("done");
                        break;
                    }
                },
                Some((direction, _)) => { *direction }
            };

        // Accept a movement command via an input instruction.
        // Send the movement command to the repair droid.
        println!("Trying to move: {:?}", robot_direction);
        main_output.send(robot_direction.to_int()).unwrap();

        // Wait for the repair droid to finish the movement operation.
        // Report on the status of the repair droid via an output instruction.
        robot_status = RobotStatus::from_int(main_input.recv().unwrap());
        println!("RobotStatus: {:?}", robot_status);

        match robot_status {
            HasMoved => {
                // Update position
                robot_position = move_in_direction(robot_position, robot_direction);
                map.insert(robot_position, NoWall);
                path.push(robot_position);
            },
            HitWall => {
                let wall_position = move_in_direction(robot_position, robot_direction);
                map.insert(wall_position, Wall);
            },
            HasMovedAndFoundOxygenSystem => {
                // Update position
                robot_position = move_in_direction(robot_position, robot_direction);
                map.insert(robot_position, OxygenSystem);
                path.push(robot_position);

                shortest_path_length_to_oxygen = Some(path.len());

                println!("Oxygen system at: {:?}", robot_position);
                println!("Path length: {:?}", path.len());

                // We're done!
                // If we want to explore the whole map, we can take out this break
                // break;
            },
        }

        println!("Robot position: {:?}", robot_position);
        println!("");

        print_map(&map, robot_position);
    }

    println!("Waiting for robot to finish");
    // robot_thread.join().unwrap();

    // Now we have the whole map
    print_map(&map, robot_position);

    println!("Shortest path length: {}", shortest_path_length_to_oxygen.unwrap());

    return shortest_path_length_to_oxygen.unwrap() as i64;
}
