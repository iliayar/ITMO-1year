#![allow(dead_code)]
#[allow(unused_imports)]
use std::cmp::{min,max};
use std::io::{BufWriter, stdin, stdout, Write, BufRead, BufReader};
use std::iter;
use std::ops;
use std::collections::BinaryHeap;
use std::collections::BTreeSet;
use std::cmp::Reverse;

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

const INF: i64 = 1e+18 as i64;

#[derive(Clone, Copy, Debug)]
struct Edge {
    c: i64,
    from: usize,
    to: usize,
    f: i64,
    rev: usize,
    w: i64,
    idx: usize
}

#[derive(Debug)]
struct State {
    g: Vec<Vec<usize>>,
    edges: Vec<Edge>,
    t: usize,
    s: usize
}

fn add_edge(c: i64, u: usize, v: usize, w: i64, idx: usize, state: &mut State) {
    add_edge_impl(c, c, u, v, w, idx, state);
}

fn add_or_edge(c: i64, u: usize, v: usize, w: i64, idx: usize, state: &mut State) {
    add_edge_impl(c, 0, u, v, w, idx, state);
}

fn add_edge_impl(c1: i64, c2:i64, u: usize, v: usize, w: i64, idx: usize, state: &mut State) {
    state.g[u].push(state.edges.len());
    state.edges.push(Edge { c: c1, from: u, to: v, f: 0, rev: state.edges.len() + 1, w, idx});
    state.g[v].push(state.edges.len());
    state.edges.push(Edge { c: c2, from: v, to: u, f: 0, rev: state.edges.len() - 1, w: -w, idx});
}

fn ford(state: &State) -> (Vec<usize>, i64) {
    let mut a: Vec<Vec<i64>> = vec![vec![INF; state.g.len()]; state.g.len()];
    let mut p: Vec<Vec<usize>> = vec![vec![0; state.g.len()]; state.g.len()];
    a[state.s][0] = 0;
    for i in 1..state.g.len() {
	for j in 0..state.edges.len() {
	    let e: Edge = state.edges[j];
	    if a[e.to][i] > a[e.from][i - 1] + e.w && e.f < e.c {
		a[e.to][i] = a[e.from][i - 1] + e.w;
		p[e.to][i] = j;
	    }
	}
    }
    let mut j = 0;
    let mut m = INF;
    for i in 0..state.g.len() {
	if a[state.t][i] < m {
	    j = i;
	    m = a[state.t][i];
	}
    }
    let mut res: Vec<usize> = vec![0; j];
    let mut i = p[state.t][j];
    let n = j;
    while j > 0 {
	res[n - j] = i;
	j -= 1;
	i = p[state.edges[i].from][j];
    }
    return (res, m);
}

fn dijkstra(state: &State) -> (Vec<usize>, i64) {
    let mut d: Vec<i64> = vec![INF; state.g.len()];
    let mut set = BTreeSet::new();
    let mut p: Vec<usize> = vec![0; state.g.len()];
    d[state.s] = 0;
    set.insert((0, state.s));
    while let Some((dv, v))= set.iter().cloned().next() {
	set.remove(&(dv, v));
	for j in state.g[v].iter().cloned() {
	    let e = state.edges[j];
	    if d[v] + e.w < d[e.to] && e.f < e.c {
		set.remove(&(d[e.to], e.to));
		d[e.to] = d[v] + e.w;
		p[e.to] = j;
		set.insert((d[e.to], e.to));
	    }
	}
    }
    let mut res: Vec<usize> = Vec::new();
    if d[state.t] == INF {
	return (res, INF);
    }

    let mut i = p[state.t];
    loop {
	res.push(i);
	if state.edges[i].from == state.s {
	    break;
	}
	i = p[state.edges[i].from];
    }
    // res.reverse();
    return (res, d[state.t]);
}

fn with_delta((path, m): (Vec<usize>, i64), state: &State) -> (Vec<usize>, i64, bool) {
    let mut delta: i64 = INF;
    for e in path.iter().cloned() {
	delta = min(delta, state.edges[e].c - state.edges[e].f);
    }
    return (path, delta, m != INF);
}

fn min_cost(state: &mut State) -> i64 {
    while let (path, delta, true) = with_delta(dijkstra(state), state) {
	for e in path {
	    let edge = state.edges[e];
	    state.edges[e].f += delta;
	    state.edges[edge.rev].f -= delta;
	}
    }
    let mut res: i64 = 0;
    for e in state.edges.iter().step_by(2) {
	res += e.f * e.w;
    }
    return res;
}

fn dfs(u: usize, t: usize, was: &mut [bool], path: &mut Vec<usize>, state: &State) {
    if u == t {
	return;
    }
    for i in state.g[u].iter().cloned() {
	if was[i] {
	    continue;
	}
	let e = state.edges[i];
	if e.f == 1 && e.c != 0 {
	    path.push(i);
	    was[i] = true;
	    dfs(e.to, t, was, path, state);
	    break;
	}
    }
}

fn sol(scan: &mut Scanner, out: &mut dyn Write ) {
    let (n, m, k): (usize, usize, i64) = (scan.next(), scan.next(), scan.next());
    let mut state = State {
        g: vec![Vec::new(); n + 2],
        edges: Vec::new(),
        t: n + 1,
        s: n,
    };
    for i in 0..m {
	let (u, v, w) = (scan.next::<usize>() - 1, scan.next::<usize>() - 1, scan.next::<i64>());
	add_or_edge(1, u, v, w, i, &mut state);
	add_or_edge(1, v, u, w, i, &mut state);
    }
    add_or_edge(k, state.s, 0, 0, n, &mut state);
    add_or_edge(k, n - 1, state.t, 0, n + 1, &mut state);
    let c = min_cost(&mut state);
    let flow = state.edges[state.g[state.s][0]].f;
    if  flow < k {
	writeln!(out, "-1").ok();
	return;
    }
    writeln!(out, "{}", (c as f64) / (k as f64)).ok();
    let mut was: Vec<bool> = vec![false; state.edges.len()];
    for _ in 0..k {
	let mut path: Vec<usize> = Vec::new();
	dfs(0, n - 1, &mut was, &mut path, &state);
	write!(out, "{}", path.len()).ok();
	for i in path.iter().cloned() {
	    write!(out, " {}", state.edges[i].idx + 1).ok();
	}
	writeln!(out, "").ok();
    }
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
