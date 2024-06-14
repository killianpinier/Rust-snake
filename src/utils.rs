use rand::Rng;

pub fn generate_random_int(min: u16, max: u16) -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}