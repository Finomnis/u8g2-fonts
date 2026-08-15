#![no_std]
#![no_main]

use benchmarks::*;

bench_main! {{
    let arg1 = 3;
    let arg2 = 2;

    let result = bench_run!(bench_fn, arg1, arg2);

    assert_eq!(result, 1);
}}

fn bench_fn(a: u32, b: u32) -> u32 {
    a - b
}
