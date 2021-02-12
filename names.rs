fn main() {
    let nomes = [
        "yhlad",
        "sahniuqul",
        "alognip ordep",
        "euqirneh sacul",
        "nahcor"
    ];

    for nome in nomes.iter() {
        println!("{}", 
            nome.
            chars().rev()
            .collect::<String>()
            .to_ascii_uppercase()
        );
    }
}
