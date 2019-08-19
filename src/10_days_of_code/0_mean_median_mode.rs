use std::convert::TryInto;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn read_stdin() -> (u64, Vec<u64>) {
    let stdin = io::stdin(); // creates a reader
    let lines: Vec<String> = stdin
        .lock()
        .lines() // gets an iterable of lines
        .filter_map(|l| l.ok()) // checks there's no errors reading input
        .collect(); // as a vector
    let n: u64 = lines.first().unwrap().parse::<u64>().unwrap();
    let args: Vec<u64> = lines[2]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    return (n, args);
}

fn mean(args: &Vec<u64>) -> f64 {
    let summed: u64 = args.iter().sum();
    // let a = f32::from(summed);
    let n: u64 = args.len().try_into().unwrap();
    return  (summed as f64) / (n as f64);
}

fn median(args: &mut Vec<u64>) -> f64 {
    args.sort_unstable();
    let n = args.len();
    let middle = args[n / 2];
    if n % 2 == 0 {
        return middle as f64;
    } else {
        let other_middle = args[n / 2 + 1];
        return ((middle + other_middle) as f64) / 2f64;
    }
}

fn mode(args: &Vec<u64>) -> u64 {
    let mut count = HashMap::new();
    for &i in args {
        let x = count.entry(i).or_insert(1u64);
        *x += 1u64;
    }
    let mut vcounts: Vec<(&u64, &u64)> = count.iter()
        .map(|(num, count)| (num, (count as &u64)))
        .collect();
    vcounts.sort_unstable_by_key(|(_, &k)| k);
    match vcounts.first() {
        Some(k) => return *(*k).0,
        None => return 0
    }
}

fn main() {
    let (_n, mut args) = read_stdin();
    println!("{:.1}", mean(&args));
    println!("{:.1}", median(&mut args));
    println!("{:.1}", mode(&args));
}
