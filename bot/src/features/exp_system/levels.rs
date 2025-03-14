const DIFFICULTY: f64 = 10f64;

/// Converts experience points to the corresponding level.
pub fn convert_exp_to_level(exp: i64) -> i64 {
    let expected_level =
        ((exp as f64).powf(2f64 / 3f64) / DIFFICULTY.powf(2f64 / 3f64)).round() as i64;

    if convert_level_to_exp(expected_level) > exp {
        expected_level - 1
    } else {
        expected_level
    }
}

/// Converts a level to the corresponding experience required.
pub fn convert_level_to_exp(level: i64) -> i64 {
    (DIFFICULTY * (level as f64).sqrt() * level as f64).floor() as i64
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

    #[test]
    fn test_exp_not_on_barrier() {
        for i in 1..100 {
            let exp = convert_level_to_exp(i) - i - 1;
            let level = convert_exp_to_level(exp);
            let required = convert_level_to_exp(i);
            println!(
                "i: {}, Level: {}, Exp: {}, required exp: {}",
                i, level, exp, required
            );
            assert_eq!(level, i - 1);
        }
    }
}
