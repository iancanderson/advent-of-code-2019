use std::fs;
use itertools::Itertools;

fn main() {
    let mut input = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    // Remove newline at end of string
    input.pop();

    let image_width = 25;
    let image_height = 6;
    let pixels_per_layer = image_width * image_height;

    let layers = sub_strings(input.as_str(), pixels_per_layer);
    let layer_with_fewest_zeros = layers.iter().min_by_key(|&layer| {
        character_count(layer, '0')
    }).expect("There will be a layer with the fewest zeros");

    let ones_in_layer = character_count(layer_with_fewest_zeros, '1');
    let twos_in_layer = character_count(layer_with_fewest_zeros, '2');

    println!("ones: {}, twos: {}", ones_in_layer, twos_in_layer);

    let result = ones_in_layer * twos_in_layer;
    println!("Result: {}", result);
}

fn character_count(input: &Vec<char>, character: char) -> usize {
    return input.iter().filter(|&pixel| *pixel == character).count();
}

// https://users.rust-lang.org/t/solved-how-to-split-string-into-multiple-sub-strings-with-given-length/10542/11
fn sub_strings(source: &str, sub_size: usize) -> Vec<Vec<char>> {
    source.chars()
        .chunks(sub_size).into_iter()
        .map(|chunk| chunk.collect::<Vec<char>>())
        .collect::<Vec<_>>()
}
