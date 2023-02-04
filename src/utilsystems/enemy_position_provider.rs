use rand::Rng;

const POSITION_RANGE: (f32, f32) = (80.0, 500.0);

pub fn generate_random_position() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(POSITION_RANGE.0..POSITION_RANGE.1)
}
