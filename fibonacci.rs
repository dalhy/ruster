fn main() {
    fibonacci(0, 3);
}

fn fibonacci(num_min: u64, num_max: u64) {
    let valor = num_min + num_max;
    println!("{}", valor);

    if valor < 3000 {
        fibonacci(num_max, valor);
    }
}
