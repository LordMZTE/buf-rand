use buf_rand::BufRand;

fn main() {
    let mut rand = BufRand::new(rand::thread_rng());
    println!("{}", rand.rand_string_case("Hello World!"));
}