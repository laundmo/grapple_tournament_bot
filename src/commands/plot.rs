use std::io::Cursor;

use crate::{api::RegionUsers, Context};
use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;
use image::ImageBuffer;
use image::RgbImage;
use itertools::Itertools;
use plotters::prelude::*;
use sqlx::postgres::types::PgInterval;

#[cfg(any(not(debug_assertions), feature = "prepare"))]
#[poise::command(prefix_command, slash_command, aliases("graph"))]
pub(crate) async fn plot(
    ctx: Context<'_>,
    #[min = 1]
    #[max = 18]
    #[description = "how many days in the past to plot"]
    days: Option<u8>,
) -> Result<()> {
    let days = days.unwrap_or(1);

    ctx.defer().await?;
    let data = ctx.data();
    let counts = sqlx::query_as!(
        RegionUsers,
        r#"select
	time_bucket('1 minute', f."time") as "time!",
	sum(amount) as "amount!",
    'None' as "region!"
from
	(
	select
		time_bucket('1 minute', "time") as "time",
		max(amount) as "amount"
	from
		public.playercounts
	where
		"time" > now() - $1::interval
	group by
		country,
		"time"
	order by
		time asc) as f
group by
	f."time""#,
        PgInterval {
            days: days as i32,
            months: 0,
            microseconds: 0
        }
    )
    .fetch_all(
        &data
            .pool
            .clone()
            .expect("Pool should only be missing while testing"),
    )
    .await?;

    let timeiter = counts.iter().map(|c| c.time);
    let valiter = counts.iter().map(|c| c.amount);

    let timerange = timeiter
        .minmax()
        .into_option()
        .map(|t| t.0..t.1)
        .ok_or_else(|| eyre!("Time Range undetermined"))?;
    let valrange = valiter
        .minmax()
        .into_option()
        .map(|t| t.0..t.1)
        .ok_or_else(|| eyre!("Count Range undetermined"))?;
    println!("Ranges calculated");

    let (w, h) = (1920, 1080);
    let mut buf = vec![0u8; w * h * 3];

    {
        let root_area =
            BitMapBackend::with_buffer(&mut buf, (w as u32, h as u32)).into_drawing_area();
        root_area.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root_area)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .caption("Total Players", ("sans-serif", 30).into_font())
            .build_cartesian_2d(timerange, valrange)?;

        chart
            .configure_mesh()
            .light_line_style(WHITE)
            .y_desc("Player Count")
            .x_desc("Time")
            .axis_desc_style(("sans-serif", 20).into_font())
            .label_style(("sans-serif", 18).into_font())
            .x_label_formatter(&|x| x.format("%Y-%m-%d %H:%M UTC").to_string())
            .draw()?;

        chart.draw_series(LineSeries::new(
            counts.iter().map(|x| (x.time, x.amount)),
            RED,
        ))?;
    }
    println!("Plotted");

    let img: RgbImage = ImageBuffer::from_raw(w as u32, h as u32, buf)
        .ok_or_else(|| eyre!("buffer to Image conversion failed"))?;

    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut buffer);
    img.write_to(&mut writer, image::ImageOutputFormat::Png)?;
    println!("Image written");

    ctx.send(|m| m.attachment((buffer.as_slice(), "plot.png").into()))
        .await?;
    println!("plot sent");
    Ok(())
}
