use crate::{api, recurring::recurring_async_func};
use color_eyre::eyre::Result;
use sqlx::PgPool;

pub(crate) async fn create_db_writer(pool: PgPool) -> Result<()> {
    recurring_async_func(60.0 * 2.0, move || {
        let pool = pool.clone();
        async move {
            let users = api::get_users().await?;
            // skip writing db while testing
            #[cfg(not(debug_assertions))]
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
