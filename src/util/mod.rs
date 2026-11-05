use rand::random;

pub fn dice_roll(size: &u8) -> bool {
    (random::<f32>() * *size as f32).floor() == 1.0
}
