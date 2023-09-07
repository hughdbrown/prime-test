use prime_test::prime_count;

fn main() {
    let count = prime_count(100 * 1000 * 1000);
    println!("{}", count);
}
