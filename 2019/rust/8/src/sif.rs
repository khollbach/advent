/// An image represented as layers of squares of pixels.
pub type SIF = Vec<Vec<Vec<Pixel>>>;

/// One pixel in one layer of a SIF file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pixel {
    Black,
    White,
    Transparent,
}

impl Pixel {
    pub fn new(c: char) -> Self {
        match c {
            '0' => Pixel::Black,
            '1' => Pixel::White,
            '2' => Pixel::Transparent,
            _ => panic!("Invalid pixel value: {}", c),
        }
    }

    pub fn value(self) -> u8 {
        match self {
            Pixel::Black => 0,
            Pixel::White => 1,
            Pixel::Transparent => 2,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Pixel::Black => ' ',
            Pixel::White => '#',
            Pixel::Transparent => '.',
        }
    }

    pub fn is_visible(self) -> bool {
        match self {
            Pixel::Transparent => false,
            _ => true,
        }
    }
}
