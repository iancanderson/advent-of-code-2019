mod intcode;

use std::collections::HashMap;
use std::fs;
use std::sync::mpsc;
use std::thread;

fn main() {
    let mut input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let program: Vec<i64> = input.trim_end().split(",").map(|n| n.parse().unwrap()).collect();

    let tiles = run_game(program);
    println!("Number of tiles: {}", tiles.len());

    let block_tiles: Vec<&TileType> = tiles.values().filter(|&tile_type| tile_type == &TileType::Block).collect();
    println!("Number of block tiles: {}", block_tiles.len());
}

#[derive(PartialEq)]
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
}

fn run_game(program: Vec<i64>) -> HashMap<Position, TileType> {
    let mut tiles: HashMap<Position, TileType> = HashMap::new();

    // Open channel to receive output from game thread in main thread
    let (main_output, game_input) = mpsc::channel();

    // Open channel to send input from main thread to robot thread
    let (game_output, main_input) = mpsc::channel();

    let game_thread = thread::spawn(move || {
      intcode::run_intcode_with_channels(program, game_input, game_output);
    });

    loop {
        let x = main_input.recv().unwrap();
        if x == -99 {
            // End of program
            break;
        }
        let y = main_input.recv().unwrap();
        let tile_id = main_input.recv().unwrap();

        tiles.insert(Position { x, y }, TileType::from_tile_id(tile_id));
    }

    game_thread.join();

    return tiles;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i64,
    y: i64,
}
