const DIFFICULTY: f64 = 10f64;

///
/// Converts experience points to the corresponding level.
pub fn convert_exp_to_level(exp: i64) -> i64 {
    ((exp as f64).powf(2f64 / 3f64) / DIFFICULTY.powf(2f64 / 3f64)).round() as i64
}

/// Converts a level to the corresponding experience required.
pub fn convert_level_to_exp(level: i64) -> i64 {
    (DIFFICULTY * (level as f64).sqrt() * level as f64).round() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bidirectional() {
        for i in 0..100 {
            let exp = convert_level_to_exp(i);
            let level = convert_exp_to_level(exp);
            println!("i: {}, Level: {}, Exp: {}", i, level, exp);
            assert_eq!(level, i);
        }
    }
}
