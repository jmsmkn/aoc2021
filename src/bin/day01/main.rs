use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("inputs/day01/input").unwrap();
    let reader = BufReader::new(f);

    let mut depths: Vec<isize> = vec![];

    for line_ in reader.lines() {
        let line = line_.unwrap();
        let depth = line.trim().parse::<isize>().unwrap();
        depths.push(depth);
    }

    let mut previous: [isize; 3] = [0; 3];
    let mut increases = 0;

    for (i, depth) in depths.iter().enumerate() {
        if let 0..=2 = i {
            previous[i] = *depth;
            continue;
        }

        // TODO: array unpacking?
        let current: [isize; 3] = [previous[1], previous[2], *depth];

        let previous_sum: isize = previous.iter().sum();
        let current_sum: isize = current.iter().sum();

        if current_sum > previous_sum {
            increases += 1;
        }

        previous.copy_from_slice(&current);
    }

    println!("{}", increases);
}
