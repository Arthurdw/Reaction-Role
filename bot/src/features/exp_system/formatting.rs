use crate::utils::formatting::apply_format;

use super::{
    io::models::UserData,
    levels::{convert_exp_to_level, convert_level_to_exp},
};

pub fn format_user_data(message: &mut String, data: &UserData) {
    let level = convert_exp_to_level(data.exp);
    let exp_next_level = convert_level_to_exp(level + 1);

    let attributes = vec![
        ("id", data.id.to_string()),
        ("exp", int_to_word(data.exp)),
        ("position", data.position.to_string()),
        ("total", data.total_users.to_string()),
        ("level", level.to_string()),
        ("exp_next_level", int_to_word(exp_next_level)),
        ("left", int_to_word(exp_next_level - data.exp)),
    ];

    for (key, value) in attributes {
        apply_format(message, &format!("data.{}", key), value.as_str());
    }
}

pub fn int_to_word(num: i64) -> String {
    let suffixes = ["", "K", "M", "B", "T"];
    let mut n = num;
    let mut idx = 0;

    while n >= 1000 && idx < suffixes.len() - 1 {
        n /= 1000;
        idx += 1;
    }

    if idx == 0 {
        format!("{}", num)
    } else {
        format!("{:.1}{}", n, suffixes[idx])
    }
}
