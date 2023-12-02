use std::sync::Arc;

use crate::{api, recurring::recurring_async_func};
use chrono::Utc;
use color_eyre::eyre::Result;
use poise::serenity_prelude::*;
use sqlx::PgPool;
use tabled::{
    grid::config::Border,
    settings::{Padding, Settings, Style},
    Table,
};

pub(crate) async fn create_db_writer(pool: PgPool) -> Result<()> {
    recurring_async_func(60.0 * 2.0, move || {
        let pool = pool.clone();
        async move {
            let users = api::get_users().await?;

            // skip writing db while testing
            #[cfg(any(not(debug_assertions), feature = "prepare"))]
            for user in users.iter() {
                sqlx::query!(
                    r#"INSERT INTO public.playercounts ("time", country, amount) VALUES (Now(), (SELECT id FROM regions WHERE short = $1), $2);"#,
                    user.region.as_ref(),
                    user.amount as i32
                )
                .execute(&pool)
                .await?;
            }
            Ok(())
        }
    });
    Ok(())
}

pub(crate) async fn create_leaderboard_updater(ctx: Arc<Context>, me: UserId) {
    let table_config = Settings::default()
        .with(Padding::new(0, 0, 0, 0))
        .with(Style::modern());
    recurring_async_func(60.0 * 5.0, move || {
        let ctx = ctx.clone();
        let table_config = table_config.clone();
        async move {
            let mut leaderboard = api::get_leaderboard().await?;
            leaderboard.sort_unstable_by_key(|l| l.position);
            let channel = ChannelId(str::parse(&std::env::var("LEADERBOARD_CHANNEL")?)?);
            let mut messages = channel.messages(ctx.clone(), |f| f.limit(100)).await?;
            let mut mine: Vec<_> = messages.iter_mut().filter(|m| m.author.id == me).collect();
            for msg in mine.iter_mut().skip(1) {
                msg.delete(ctx.clone()).await.ok(); // ignore errors
            }
            let content = format!(
                "## Weekly Leaderboard\n```{}```\nUpdated: <t:{}:R>",
                Table::new(leaderboard).with(table_config),
                Utc::now().timestamp()
            );
            if mine.is_empty() {
                channel.send_message(ctx, |f| f.content(content)).await?;
            } else {
                mine[0].edit(ctx, |f| f.content(content)).await?;
            }
            Ok(())
        }
    })
}

pub(crate) async fn create_online_updater(ctx: Arc<Context>, me: UserId) {
    let table_config = Settings::default()
        .with(Padding::new(0, 0, 0, 0))
        .with(Style::modern());
    recurring_async_func(60.0, move || {
        let ctx = ctx.clone();
        let table_config = table_config.clone();
        async move {
            let mut online = api::get_users().await?;
            online.sort_unstable_by_key(|l| i64::MAX - l.amount);
            let channel = ChannelId(str::parse(&std::env::var("ONLINE_CHANNEL")?)?);
            let mut messages = channel.messages(ctx.clone(), |f| f.limit(100)).await?;
            let mut mine: Vec<_> = messages.iter_mut().filter(|m| m.author.id == me).collect();
            for msg in mine.iter_mut().skip(1) {
                msg.delete(ctx.clone()).await.ok(); // ignore errors
            }
            let content = if online.is_empty() {
                format!(
                    "## Online Players\nNobody online :(\nUpdated: <t:{}:R>",
                    Utc::now().timestamp()
                )
            } else {
                format!(
                    "## Online Players\nTotal Players: {}\n```{}```\nUpdated: <t:{}:R>",
                    online.iter().map(|f| f.amount).sum::<i64>(),
                    Table::new(online).with(table_config),
                    Utc::now().timestamp()
                )
            };
            if mine.is_empty() {
                channel.send_message(ctx, |f| f.content(content)).await?;
            } else {
                mine[0].edit(ctx, |f| f.content(content)).await?;
            }
            Ok(())
        }
    })
}
