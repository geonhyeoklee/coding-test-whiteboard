use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

// arr = [1, 1, 0, -1, -1]
// 0.400000
// 0.400000
// 0.200000

fn plusMinus(arr: &[i32]) {
    let total = arr.len() as f64;
    let mut pos = 0.0;
    let mut neg = 0.0;
    let mut zero = 0.0;

    for i in arr {
        if *i > 0 {
            pos += 1.0;
        } else if *i < 0 {
            neg += 1.0;
        } else {
            zero += 1.0;
        }
    }

    println!("{:.6}", pos / total);
    println!("{:.6}", neg / total);
    println!("{:.6}", zero / total);
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let _n = iter.next().unwrap().parse::<i32>().unwrap();
    let arr = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    plusMinus(&arr);
}
