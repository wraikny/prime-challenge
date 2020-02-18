use lazy_static::lazy_static;

// // https://ja.wikipedia.org/wiki/ミラー–ラビン素数判定法
#[inline]
fn mod_pow(mut b : u64, mut p : u64, m : u64) -> u64{
    let mut result = 1;
    while p > 0 {
        if p & 1 == 1 {
            result = (result * b) % m;
        }
        b = (b * b) % m;
        p >>= 1;
    }
    result
}

#[inline]
fn miller_rabin(n : &u32) -> bool {
    let d = {
        let mut d = n - 1;
        while d & 1 == 0 { d >>= 1; }
        d as u64
    };
    let n = *n as u64;

    lazy_static! {
        static ref NUMS : Vec<u64> = vec![2, 7, 61];
    }

    for a in NUMS.iter() {
        let mut t = d;
        let mut y = mod_pow(*a, t, n);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = (y * y) % n;
            t <<= 1;
        }
        if y != n - 1 && t & 1 == 0 { return false; }
    }

    true
}

use std::io::{stdout, Write, BufWriter};
use std::thread;
use rayon::prelude::*;

fn main() {
    const MAXNUM : u32 = 4_294_967_295;

    let ps = (1..=MAXNUM/6).into_par_iter()
        .map(|n| 6 * n - 1)
        .filter(miller_rabin)
        .collect::<Vec<_>>();

    let handle = thread::spawn(|| {
        (1..=MAXNUM/6).into_par_iter()
            .map(|n| 6 * n + 1)
            .filter(miller_rabin)
            .collect::<Vec<_>>()
    });

    // Rustで高速な標準出力
    // https://keens.github.io/blog/2017/10/05/rustdekousokunahyoujunshutsuryoku/
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    out.write(b"2\n3\n").unwrap();

    for n in ps.into_iter() {
        writeln!(out, "{}", n).unwrap();
    }

    for n in handle.join().unwrap().into_iter() {
        writeln!(out, "{}", n).unwrap();
    }
}