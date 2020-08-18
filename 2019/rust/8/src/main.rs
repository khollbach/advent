use common::read_line;
use sif::{Pixel, SIF};

mod sif;

fn main() {
    let img = read_input();

    println!("{}", part1(&img));
    part2(&img);
}

#[allow(clippy::ptr_arg)]
fn part1(img: &SIF) -> u32 {
    // Find layer with minimum number of zeros.
    let best_layer = img
        .iter()
        .map(|layer| {
            let mut freqs = vec![0; 3];

            for row in layer {
                for pixel in row {
                    freqs[pixel.value() as usize] += 1;
                }
            }

            (freqs[0], freqs[1], freqs[2])
        })
        .min()
        .unwrap();

    let (_, ones, twos) = best_layer;
    ones * twos
}

#[allow(clippy::ptr_arg)]
fn part2(img: &SIF) {
    // Ensure we have at least 1 layer, and 1 row.
    if img.is_empty() || img[0].is_empty() {
        return;
    }

    let height = img[0].len();
    assert!(height != 0);
    let width = img[0][0].len();

    for h in 0..height {
        for w in 0..width {
            let pixel = first_visible_pixel(img, h, w)
                .unwrap_or_else(|| panic!("No visible pixel at position ({}, {})", h, w));
            print!("{}", pixel.to_char());
        }
        println!();
    }
}

/// What is the first visible pixel, top-to-bottom, in this image?
fn first_visible_pixel(img: &SIF, h: usize, w: usize) -> Option<Pixel> {
    for d in 0..img.len() {
        let pixel = img[d][h][w];
        if pixel.is_visible() {
            return Some(pixel);
        }
    }
    None
}

/// Read the image into a 3-D matrix: [layer, row, column].
fn read_input() -> Vec<Vec<Vec<Pixel>>> {
    const HEIGHT: usize = 6;
    const WIDTH: usize = 25;

    let s = read_line().unwrap();

    // Calculate the number of layers.
    let n = s.len();
    let depth = n / HEIGHT / WIDTH;
    assert_eq!(depth * HEIGHT * WIDTH, n, "Image isn't \"square\" (cubic)");

    let mut chars = s.chars();

    let mut layers = Vec::with_capacity(depth);
    for _ in 0..depth {
        let mut layer = Vec::with_capacity(HEIGHT);
        for _ in 0..HEIGHT {
            let mut row = Vec::with_capacity(WIDTH);
            for _ in 0..WIDTH {
                // We checked that the image is "square", so
                // next() will succeed if the input is pure ascii.
                row.push(Pixel::new(chars.next().expect("Non-ascii input")));
            }
            layer.push(row);
        }
        layers.push(layer);
    }

    // Again, this will succeed if the input is pure ascii.
    assert!(chars.next().is_none(), "Non-ascii input");

    layers
}
