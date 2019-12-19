mod intcode;

use std::collections::BTreeMap;
use std::fs;
use std::sync::mpsc;
use std::{thread, time};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut program: Vec<i64> = input.trim_end().split(",").map(|n| n.parse().unwrap()).collect();

    //Memory address 0 represents the number of quarters that have been inserted; set it to 2
    //to play for free.
    program[0] = 2;

    let score = run_game(program);
    println!("Score: {}", score);
}

#[derive(Debug, PartialEq)]
enum TileType {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

use TileType::*;

impl TileType {
    fn from_tile_id(tile_id: i64) -> TileType {
        match tile_id {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => HorizontalPaddle,
            4 => Ball,
            n => panic!("Unknown tile id {}", n),
        }
    }

    fn print(&self) -> &str {
        match self {
            Empty => ".",
            Wall => "▏",
            Block => "▀",
            HorizontalPaddle => "▂",
            Ball => "o",
        }
    }
}

enum Direction {
    Left,
    Right,
}

use Direction::*;

fn run_game(program: Vec<i64>) -> i64 {
    let mut tiles: BTreeMap<Position, TileType> = BTreeMap::new();

    // Open channel to send input from main thread to game thread
    let (main_output, game_input) = mpsc::channel();

    // Open channel to receive output from game thread in main thread
    let (game_output, main_input) = mpsc::channel();

    let game_thread = thread::spawn(move || {
      intcode::run_intcode_with_channels(program, game_input, game_output);
    });

    let mut score = 0;
    let mut ball_position: Option<Position> = None;
    let mut paddle_position: Option<Position> = None;
    let mut need_to_send_initial_message = true;

    loop {
        let x = main_input.recv().unwrap();
        if x == -99 {
            // End of program
            break;
        }
        let y = main_input.recv().unwrap();

        if x == -1 && y == 0 {
            // When three output instructions specify X=-1, Y=0, the third output instruction is
            // not a tile; the value instead specifies the new score to show in the segment
            // display. For example, a sequence of output values like -1,0,12345 would show 12345
            // as the player's current score.
            score = main_input.recv().unwrap();
            println!("New score: {}", score);
        } else {
            let tile_id = main_input.recv().unwrap();
            let tile_type = TileType::from_tile_id(tile_id);
            println!("tile type: {:?}", tile_type);
            let position = Position { x, y };
            println!("tile position: {:?}", position);

            if let Ball = tile_type {
                if let Some(paddle_position) = paddle_position {
                    if tiles.len() == 968 {
                        // Move joystick toward ball's position
                        //If the joystick is in the neutral position, provide 0.
                        //If the joystick is tilted to the left, provide -1.
                        //If the joystick is tilted to the right, provide 1.
                        if position.x < paddle_position.x {
                            main_output.send(-1).unwrap();
                        } else if position.x > paddle_position.x {
                            main_output.send(1).unwrap();
                        } else {
                            main_output.send(0).unwrap();
                        }
                    }
                }

                ball_position = Some(position);
            } else if let HorizontalPaddle = tile_type {
                paddle_position = Some(position);
            }

            tiles.insert(position, tile_type);

            // 44 x 22 tile screen
            if tiles.len() == 968 {
                print_tiles(&tiles, score);
                if need_to_send_initial_message {
                    need_to_send_initial_message = false;
                    main_output.send(1).unwrap();
                }
            }
        }
    }

    game_thread.join().unwrap();

    return score;
}

fn print_tiles(tiles: &BTreeMap<Position, TileType>, score: i64) {
    // slow down the animation
    thread::sleep(time::Duration::from_millis(15));

    for (position, tile_type) in tiles {
        // println!("printing position {:?}", position);
        if position.x == 0 {
            println!("");
        }
        print!("{}", tile_type.print());
    }
    println!("");
    println!("SCORE: {}", score);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct Position {
    y: i64, // order of fields matters here since we're deriving Ord (so (0, 1) comes before (1, 0))
    x: i64,
}
