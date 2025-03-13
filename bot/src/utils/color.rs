use poise::serenity_prelude::Color;

pub fn random_color() -> Color {
    Color::new(rand::random::<u32>() % 16777215)
}
