use ndarray::prelude::*;

use sub_wth::foo;

fn foo_dup(n: usize) -> Array2<f64> {
    let mut s = Array::zeros((n, n));

    s[[0, 0]] = 1.0;

    s
}

fn main() {
    let s: Array2<f64> = foo(10);
    let s_dup: Array2<f64> = foo_dup(10);
}
