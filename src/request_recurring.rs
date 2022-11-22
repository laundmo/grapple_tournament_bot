use crate::{api, recurring::recurring_async_func, BotError};
use sqlx::postgres::PgPoolOptions;

pub(crate) async fn create_db_writer() -> Result<(), BotError> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("missing DATABASE_URL"))
        .await?;

    println!("migrating");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("migrations done");

    recurring_async_func(60.0 * 2.0, move || {
        let pool = pool.clone();
        async move {
            let users = api::get_users().await?;
            for user in users.iter() {
                sqlx::query!(
                r#"INSERT INTO public.playercounts ("time", country, amount) VALUES (Now(), (SELECT id FROM regions WHERE short = $1), $2);"#,
            user.region.as_ref(),
            user.online_player_count as i32
            )
            .execute(&pool)
            .await?;
            }
            Ok(())
        }
    });
    Ok(())
}
