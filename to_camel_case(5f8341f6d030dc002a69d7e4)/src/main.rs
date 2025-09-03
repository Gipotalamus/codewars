fn main() {
    println!("{:?}", to_camel_case("the_stealth_warrior"));
}

fn to_camel_case(text: &str) -> String {
    unsafe {
        text.to_owned().as_mut_vec().split_mut(|&c| c == b'-' || c == b'_').enumerate().map(|(i, w)| {
            if i == 0 {
                return w;
            }
            if let Some(r) = w.get_mut(0..1) {
                r.make_ascii_uppercase();
            }
            w
        }).fold(String::new(), |mut acc, w| {
            acc.push_str(&String::from_utf8(w.to_vec()).unwrap());
            acc
        })
    }
}
    
