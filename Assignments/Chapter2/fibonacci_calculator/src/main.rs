fn main() {
    let output = fibonacci(2);
    println!("The result was {output}");
}

fn fibonacci(n: u32) -> u32 {
    let mut num_0 = 1;
    let mut num_1 = 0;

    for _ in 0..n {
        let num_n = num_0 + num_1;
        num_0 = num_1;
        num_1 = num_n;
    }

    num_1
}
