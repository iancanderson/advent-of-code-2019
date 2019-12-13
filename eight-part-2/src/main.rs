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

    // Loop through each pixel, choose the first visible color from front to back
    let rendered_layer =
        (0..pixels_per_layer).map(|pixel_index| {
            for layer in layers.to_vec() {
                // return the frontmost pixel that's not transparent
                if layer[pixel_index] != '2' {
                    return layer[pixel_index];
                }
            }

            return '2';
        }).collect();


    print_layer(rendered_layer);
}

fn print_layer(layer: Vec<char>) {
    for line in layer.chunks(25) {
        for c in line {
            if *c == '0' {
                print!("■■");
            } else {
                print!("  ");
            }
        }
        println!("");
        for c in line {
            if *c == '0' {
                print!("■■");
            } else {
                print!("  ");
            }
        }
        println!("");
    }
}

// https://users.rust-lang.org/t/solved-how-to-split-string-into-multiple-sub-strings-with-given-length/10542/11
fn sub_strings(source: &str, sub_size: usize) -> Vec<Vec<char>> {
    source.chars()
        .chunks(sub_size).into_iter()
        .map(|chunk| chunk.collect::<Vec<char>>())
        .collect::<Vec<_>>()
}
