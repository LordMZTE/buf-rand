use buf_rand::BufRand;
use rand::RngCore;

fn main() {
    let mut buf_rand = BufRand::new(rand::thread_rng());
    // the inner randomizer (rand) of buf_rand can still be used
    buf_rand.rand.next_u32();
}
