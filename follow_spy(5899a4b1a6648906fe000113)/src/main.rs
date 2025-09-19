fn main() {
    println!(
        "{:?}",
        find_routes(&[
            ["Chicago", "Winnipeg"],
            ["Halifax", "Montreal"],
            ["Montreal", "Toronto"],
            ["Toronto", "Chicago"],
            ["Winnipeg", "Seattle"]
        ])
    );
}

// &[["Chicago", "Winnipeg"], ["Halifax", "Montreal"], ["Montreal", "Toronto"], ["Toronto", "Chicago"], ["Winnipeg", "Seattle"]]
// "Halifax, Montreal, Toronto, Chicago, Winnipeg, Seattle"

// &[["MNL", "TAG"], ["CEB", "TAC"], ["TAG", "CEB"], ["TAC", "BOR"]]
// "MNL, TAG, CEB, TAC, BOR"

fn find_routes<S>(routes: &[[S; 2]]) -> String
where
    S: AsRef<str> + std::fmt::Debug + std::fmt::Display,
{
    let routes = routes
        .iter()
        .map(|[a, b]| [a.to_string(), b.to_string()])
        .collect::<Vec<[String; 2]>>();
    let mut r: Vec<String> = routes
        .iter()
        .find(|[a, _]| !routes.iter().any(|[_, y]| a == y))
        .unwrap()
        .iter()
        .cloned()
        .collect();

    let mut last = r[1].clone();
    for _ in 0..routes.len() - 1 {
        last = routes.iter().find(|[a, _]| *a == last).unwrap()[1].clone();
        r.push(last.clone());
    }

    r.join(", ")
}
