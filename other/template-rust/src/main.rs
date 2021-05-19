#![allow(dead_code)]
#[allow(unused_imports)]
use std::cmp::{min,max};
use std::io::{BufWriter, stdin, stdout, Write, BufRead, BufReader};
use std::iter;
use std::ops;
use std::time::{SystemTime, UNIX_EPOCH};

const FILENAME: &str = "filename";
const IS_FILES: bool = false;

struct Scanner {
    buffer: Vec<String>,
    reader: Box<dyn BufRead>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    fn new(reader: Box<dyn BufRead>) -> Scanner {
        Scanner {
            buffer: vec![],
            reader
        }
    }
}

//================================ CODE BEGIN ===============================================


fn gcdext(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
	return (b, 0, 1);
    }
    let (d, x, y) = gcdext(b % a, a);
    (d, y - (b / a) * x, x)
}

fn solve_eq(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
    let (mut d, x, y) = gcdext(a, b);
    if c % d != 0 {
	return None;
    }
    return Some((x * (c / d), y * (c / d)));
}
fn modmul(a: i64, b: i64, m:i64) -> i64 {
    (((a as i128) * (b as i128)) % (m as i128)) as i64
}

fn pow(n: i64, k: i64, m: i64) -> i64 {
    if m == 0 {
	return 1;
    }
    let mut b: i64 = 1;
    let mut k = k;
    let mut n = n % m;
    while k > 1 {
	if k % 2 == 0 {
	    n = modmul(n, n, m);
	    k /= 2;
	} else {
	    b = modmul(b, n, m);
	    k -= 1;
	}
    }
    return modmul(b, n, m);
}

fn sol(scan: &mut Scanner, out: &mut dyn Write ) {
    let mut n = scan.next::<i64>();
    let mut e = scan.next::<i64>();
    let mut c = scan.next::<i64>();
    let mut p = 2;
    let m = (n as f64).sqrt().floor() as i64;
    while p <= m {
	if n % p == 0 {
	    break;
	}
	p += 1;
    }
    let q = n / p;
    let mut d = 1;
    while (e * d) % ((p - 1)*(q - 1)) != 1 {
	d += 1;
    }
    let m = pow(c, d, n);
    writeln!(out, "{}", m).ok();
}

//================================ CODE END =================================================

fn main() {

    let mut local = false;
    for arg in std::env::args() {
        if arg == "LOCAL" {
            local = true;
        }
    }
    if local {
        let mut scan = Scanner::new(Box::new(BufReader::new(std::fs::File::open("local.in").expect("Cannot open local.in"))));
        let mut out = &mut BufWriter::new(std::fs::File::create("local.out").expect("Cannot open local.out"));
        let now = std::time::Instant::now();
        sol(&mut scan, &mut out);
        writeln!(out, "{}ms", now.elapsed().as_millis()).ok();
    } else {
        if IS_FILES {
            let mut scan = Scanner::new(Box::new(BufReader::new(std::fs::File::open(FILENAME.to_owned() + ".in").expect("Cannot open remote input"))));
            let mut out = &mut BufWriter::new(std::fs::File::create(FILENAME.to_owned() + ".out").expect("Cannot open remote output"));
            sol(&mut scan, &mut out);
        } else {
            let mut scan = Scanner::new(Box::new(BufReader::new(stdin())));
            let mut out = &mut BufWriter::new(stdout());
            sol(&mut scan, &mut out);
        }
    }
}
