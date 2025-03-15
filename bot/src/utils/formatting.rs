use poise::serenity_prelude::{ChannelId, Mentionable, PartialGuild, ReactionType, RoleId, User};
use tracing::error;

pub fn apply_format(message: &mut String, to: &str, value: &str) {
    *message = message.replace(&format!("{{{}}}", to), value);
}

pub fn format_user(target: &str, message: &mut String, user: &User) {
    let attributes = vec![
        ("mention", user.mention().to_string()),
        ("name", user.name.clone()),
        (
            "discriminator",
            user.discriminator.map_or("".to_string(), |d| d.to_string()),
        ),
        ("id", user.id.to_string()),
    ];

    for (key, value) in attributes {
        apply_format(message, &format!("{}.{}", target, key), value.as_str());
    }
    apply_format(message, "member", user.display_name());
}

pub fn format_member(message: &mut String, user: &User) {
    format_user("member", message, user)
}

pub fn format_partial_guild(message: &mut String, guild: &PartialGuild) {
    let attributes = vec![
        ("id", guild.id.to_string()),
        ("name", guild.name.clone()),
        ("icon_url", guild.icon_url().unwrap_or("".to_string())),
    ];

    for (key, value) in attributes {
        apply_format(message, &format!("guild.{}", key), value.as_str());
    }
}

pub fn format_channel_id(message: &mut String, channel_id: &ChannelId) {
    let attributes = vec![
        ("id", channel_id.get().to_string()),
        ("mention", channel_id.mention().to_string()),
    ];

    for (key, value) in attributes {
        apply_format(message, &format!("channel.{}", key), value.as_str());
    }
}

pub fn format_reaction(message: &mut String, reaction: &ReactionType) {
    let value = match reaction {
        ReactionType::Custom { id, name, .. } => name.clone().unwrap_or_else(|| id.to_string()),
        ReactionType::Unicode(emoij) => emoij.to_string(),
        _ => {
            error!("Got unsupported reaction type: {:?}", reaction);
            return;
        }
    };

    apply_format(message, "emoji", &value);
}

pub fn format_role(message: &mut String, role: &RoleId) {
    let attributes = vec![
        ("id", role.to_string()),
        ("mention", role.mention().to_string()),
    ];

    for (key, value) in attributes {
        apply_format(message, &format!("role.{}", key), value.as_str());
    }
}
