fn main() {
    println!("{:?}", reduce_fraction((15, 10)));
}

fn reduce_fraction(fraction: (u32, u32)) -> (u32, u32) {
    if fraction.0 == fraction.1 {
        return (1, 1);
    }
    let min = u32::min(fraction.0, fraction.1);
    for i in (1..=(min / 2)).chain(std::iter::once(min)).rev() {
        if fraction.0 % i == 0 && fraction.1 % i == 0 {
            return (fraction.0 / i, fraction.1 / i);
        }
    }
    (fraction.0, fraction.1)
}
