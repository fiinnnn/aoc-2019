use crate::solver::Solver;
use std::io;

pub struct Problem;

pub struct Image {
    width: u8,
    height: u8,
    layers: Vec<Vec<u8>>,
}

impl Image {
    fn new(width: u8, height: u8, data: Vec<u8>) -> Image {
        let layers = data
            .chunks((width * height) as usize)
            .map(|layer| layer.into())
            .collect();

        Image {
            width,
            height,
            layers,
        }
    }

    fn display(&self) {
        let mut pixels: Vec<u8> = Vec::with_capacity((self.width * self.height) as usize);
        for i in 0..self.width * self.height {
            pixels.push(self.layers.iter()
                .map(|l| l[i as usize])
                .find(|&p| p != 2)
                .unwrap())
        }

        pixels.chunks(self.width as usize)
            .for_each(|row| {
                for &pixel in row {
                    let char = if pixel == 1 { "#" } else { " " };
                    print!("{}", char);
                }
                println!();
            });
    }

    fn counts(layer: &Vec<u8>) -> [i32; 3] {
        let mut counts = [0; 3];
        for &pixel in layer {
            counts[pixel as usize] += 1;
        }
        counts
    }
}

impl Solver for Problem {
    type Input = Image;
    type Output1 = usize;
    type Output2 = String;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let n: Vec<u8> = r.bytes().flatten().map(|b| b - b'0').collect();

        Image::new(25, 6, n)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let counts = Image::counts(input.layers
            .iter()
            .min_by_key(|layer| Image::counts(layer)[0])
            .unwrap());

        (counts[1] * counts[2]) as usize
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.display();

        "GCPHL".into()
    }
}