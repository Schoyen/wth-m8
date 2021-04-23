use ndarray::prelude::*;


pub fn foo(n: usize) -> Array2<f64> {
    let mut s = Array::zeros((n, n));

    s[[0, 0]] = 1.0;

    s
}
