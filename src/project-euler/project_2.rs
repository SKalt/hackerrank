use std::io::{self, BufRead};
use std::vec::Vec;
use std::iter::{Iterator};

fn read_stdin() -> Vec<u64> {
    let stdin = io::stdin(); // creates a reader
    let lines: Vec<u64> = stdin
        .lock()
        .lines() // gets an iterable of lines
        .filter_map(|l| l.ok()) // checks there's no errors reading input
        .map(|l| l.parse::<u64>().unwrap()) // read to unsigned 64-bit numbers
        .collect(); // as a vector
    return lines;
}

fn solve(n: &u64, fibs: &mut Vec<u64>, evens: &mut Vec<u64>) {
    let mut fib = fibs.last().unwrap() + 0; // hack: copy the &u64 -> u64 so I can mutate it
    while fib < *n {
        fib = fibs[fibs.len() - 1] + fibs[fibs.len() - 2];
        fibs.push(fib);
        if fib % 2 == 0 {
            evens.push(fib);
        }
    }
    let r = match evens.binary_search(n) {
        Ok(i) => {i + 1}
        Err(i) => {i}
    };
    let sum_of_evens: u64 = evens[0..r].iter().sum();
    println!("{:?}", sum_of_evens);
}

fn main() {
    let mut fibs = vec![1u64, 1];
    let mut evens: Vec<u64> = vec![0u64];
    let input = read_stdin();
    let (_t, rest) = input.split_first().unwrap();
    for n in rest {
        solve(&n, &mut fibs, &mut evens);
    }
}
