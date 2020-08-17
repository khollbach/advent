use common::read_line;

fn main() {
    let img = read_input();

    println!("{}", day1(&img));
}

#[allow(clippy::ptr_arg)]
fn day1(img: &SIF) -> u32 {
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

/// An image represented as layers of squares of pixels.
type SIF = Vec<Vec<Vec<Pixel>>>;

/// One pixel in one layer of a SIF file.
enum Pixel {
    Zero,
    One,
    Two,
}

impl Pixel {
    fn new(c: char) -> Self {
        match c {
            '0' => Pixel::Zero,
            '1' => Pixel::One,
            '2' => Pixel::Two,
            _ => panic!("Invalid pixel value: {}", c),
        }
    }

    fn value(&self) -> u8 {
        match self {
            Pixel::Zero => 0,
            Pixel::One => 1,
            Pixel::Two => 2,
        }
    }
}

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

/// Read the image into a 3-D matrix: [layer, row, column].
fn read_input() -> Vec<Vec<Vec<Pixel>>> {
    let s = read_line().unwrap();

    // Calculate the number of layers.
    let n = s.len();
    let depth = n / HEIGHT / WIDTH;
    if depth * HEIGHT * WIDTH != n {
        panic!(
            "Image isn't \"square\" (cubic). (d, w, h, n): ({}, {}, {}, {})",
            depth, WIDTH, HEIGHT, n
        );
    }

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

    // Again, this won't happen if the input is pure ascii.
    if chars.next().is_some() {
        panic!("Non-ascii input");
    }

    layers
}
