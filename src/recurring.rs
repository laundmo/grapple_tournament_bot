use std::{future::Future, time::Duration};
use tokio::{task, time};

use crate::BotError;

pub(crate) fn recurring_async_func<T, R>(delay_s: f32, func: T)
where
    T: Fn() -> R + Send + Sync + 'static,
    R: Future<Output = Result<(), BotError>> + Send,
{
    let mut interval = time::interval(Duration::from_secs_f32(delay_s));
    task::spawn(async move {
        loop {
            interval.tick().await;
            let r = func().await;
            if let Err(e) = r {
                println!("{:?}", e);
            }
        }
    });
}
