#![feature(option_expect_none)]

use std::env;
use std::fs;

type Pixel = u64;
type Row = Vec<Pixel>;
type Layer = Vec<Row>;
type Image = Vec<Layer>;

fn process_image(raw: &String, width: usize, height: usize) -> Image {
    let len = raw.len();
    let mut pixels = raw
        .chars()
        .map(|digit| digit.to_digit(10).expect("Couldn't parse pixel!") as Pixel);

    let mut image = Vec::new();
    let num_layers = len / (width * height);
    for _ in 0..num_layers {
        let mut layer = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(pixels.next().expect("Unexpected end-of-image"));
            };
            layer.push(row);
        };
        image.push(layer);
    };

    image
}

fn flatten(image: &Image) -> Layer {
    let mut flat: Layer = image[0].to_vec();

    for layer in image.iter().skip(1) {
        for (r, row) in layer.iter().enumerate() {
            for (c, &pixel) in row.iter().enumerate() {
                if flat[r][c] == 2 {
                    flat[r][c] = pixel;
                };
            };
        };
    };

    flat
}

fn display(image: &Image) -> () {
    let flat = flatten(image);

    for row in flat {
        for pixel in row {
            match pixel {
                0 => print!("⬛"),
                1 => print!("⬜"),
                2 => print!("  "),
                _ => panic!("Unknown pixel {}", pixel)
            };
        };
        println!("");
    };
}


fn part1(image: &Image) -> () {
    let min_layer: &Layer = image
        .iter()
        .min_by(|layer1, layer2| {
            let count = |layer: &Layer| layer
                .iter()
                .flatten()
                .filter(|&&pixel| pixel == 0)
                .count();
            count(layer1).cmp(&count(layer2))
        })
        .expect("No layer with fewest 0 pixels!");

    let num_1_pixels = min_layer
        .iter()
        .flatten()
        .filter(|&&pixel| pixel == 1)
        .count();
    let num_2_pixels = min_layer
        .iter()
        .flatten()
        .filter(|&&pixel| pixel == 2)
        .count();

    println!("Part 1: {}", num_1_pixels * num_2_pixels);
}

fn part2(image: &Image) -> () {
    println!("Part 2:");
    display(image);
}


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut lines: Vec<String> = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            lines.push(line.to_string());
        };
    };

    let mut dimensions = lines[0].split('x');
    let (width, height) = (
        dimensions.next().expect("No width!").parse().expect("Couldn't parse width!"),
        dimensions.next().expect("No height!").parse().expect("Couldn't parse height!")
    );
    dimensions.next().expect_none("Extraneous dimensions!");
    let raw = &lines[1];
    let image = process_image(raw, width, height);

    part1(&image);
    part2(&image);

    Ok(())
}
