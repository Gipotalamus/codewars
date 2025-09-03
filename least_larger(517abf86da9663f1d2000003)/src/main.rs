fn main() {
    println!("{:?}", least_larger(&[4, 1, 3, 5, 6], 0));
}

fn least_larger(xs: &[i32], i: usize) -> Option<usize> {
    xs.iter().enumerate().fold(None, |col, (j, &x)| {
        if x > xs[i] && (col.is_none() || xs[col.unwrap()] > x) {
            Some(j)
        } else {
            col
        }
    })
}
