use crate::{
    run::Context,
    utils::{
        color::random_color,
        formatting::{apply_format, format_member},
    },
};
use anyhow::Result;
use poise::{
    CreateReply, command,
    serenity_prelude::{CreateEmbed, User, UserId},
};
use tracing::debug;

use super::{formatting::format_user_data, io::models::UserData};

fn build_formatter(user: &User, data: &UserData) -> impl Fn(&str) -> String {
    move |message| {
        let mut message = message.to_string();
        format_member(&mut message, user);
        format_user_data(&mut message, data);

        message
    }
}

/// Retrieve the rank of a user.
/// Default user is the person whom has invoked the command.
#[command(slash_command, prefix_command, aliases("lvl", "rank"))]
pub async fn level(
    ctx: Context<'_>,
    #[description = "Retrieve the rank of a user."] user: Option<User>,
) -> Result<()> {
    debug!("Received level command from {}", ctx.author().id.get());
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let db = {
        let db = &ctx.data().exp_db;

        if db.is_none() {
            ctx.reply("Leveling system is disabled.").await?;
            return Ok(());
        }

        db.as_ref().unwrap().clone()
    };
    let lang = &ctx.data().lang.exp_system;

    if u.bot {
        let mut msg = lang.bot.clone();
        format_member(&mut msg, u);

        ctx.send(
            CreateReply::default()
                .content(msg)
                .ephemeral(true)
                .reply(true),
        )
        .await?;
        return Ok(());
    }

    let data = db.get_user_data(u.id.get()).await?;
    let fmt = build_formatter(u, &data);

    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title(fmt(&lang.title))
                    .description(fmt(&lang.message))
                    .color(random_color()),
            )
            .ephemeral(true)
            .reply(true),
    )
    .await?;
    Ok(())
}

/// Fetch the top x members with the highest exp.
/// Default amount is 10.
#[command(slash_command, prefix_command, aliases("lvl", "rank"))]
pub async fn top(
    ctx: Context<'_>,
    #[description = "The amount of users to display"] amount: Option<u64>,
) -> Result<()> {
    debug!("Received level command from {}", ctx.author().id.get());
    let cfg = &ctx.data().config.leveling;
    let amount = amount.unwrap_or(10).max(cfg.max_top).min(2);

    let db = {
        let db = &ctx.data().exp_db;
        if db.is_none() {
            ctx.reply("Leveling system is disabled.").await?;
            return Ok(());
        }
        db.as_ref().unwrap().clone()
    };

    let data = db.get_top(amount).await?;
    let lang = &ctx.data().lang.exp_system;

    let mut title = lang.top_title.clone();
    format_member(&mut title, ctx.author());
    apply_format(&mut title, "amount", &data.len().to_string());

    let descriptions: Vec<_> = data
        .iter()
        .enumerate()
        .map(async |(i, data)| {
            let user_id = UserId::new(data.id);
            let user = user_id.to_user(&ctx.http()).await.unwrap();
            let fmt = build_formatter(&user, data);
            let mut msg = fmt(&lang.top_line);
            apply_format(&mut msg, "idx", &(i + 1).to_string());
            msg
        })
        .collect();

    let mut description = futures::future::join_all(descriptions).await.join("\n");

    if description.is_empty() {
        description = lang.top_no_users.clone();
    }

    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title(title)
                    .description(description)
                    .color(random_color()),
            )
            .ephemeral(true)
            .reply(true),
    )
    .await?;
    Ok(())
}
