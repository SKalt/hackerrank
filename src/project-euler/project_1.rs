// Enter your code here
#[allow(unused_imports)]
use std::io::{self, BufRead, Lines, Error};

fn read_stdin() -> Vec<u64> {
    let stdin = io::stdin(); // creates a reader
    let lines = stdin
        .lock()
        .lines() // gets an iterable of lines
        .filter_map(|l| l.ok()) // checks there's no errors reading input
        .map(|l| l.parse::<u64>().unwrap()) // read to unsigned 64-bit numbers
        .collect(); // as a vector
    return lines;
}

fn cumulative_series(n: u64) -> u64 {
    /*
    return the lower diagonal of n^2 as below

       _1_ _2_ _3_
     1| x
     2| x  x
     3| x  x  x

     */
    return n * (n + 1) / 2;
}

fn solve(n: &u64) -> u64 {
    let thirds = cumulative_series((n - 1) / 3) * 3;
    let fifths = cumulative_series((n - 1) / 5) * 5;
    let fifteenths = cumulative_series((n - 1) / 15) * 15;
    return thirds + fifths - fifteenths;
}

fn main() {
    let input = read_stdin();
    let (_t, rest) = input.split_first().unwrap();
    for n in rest {
        println!("{:?}", solve(n));
    }
}
