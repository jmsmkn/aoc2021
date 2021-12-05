pub fn solve(depths: Vec<isize>) -> isize {
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

    return increases;
}
