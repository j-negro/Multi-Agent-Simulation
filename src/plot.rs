use super::Result;
use plotters::prelude::*;

const MARGIN_SIZE: u32 = 10;
const LABEL_AREA_SIZE: u32 = 40;
const FONT_SIZE: u32 = 50;
const HEIGHT: u32 = 1000;
const WIDTH: u32 = 1000;

pub fn order_time_graph(
    graph_path: &str,
    orders_list: Vec<(u32, f64)>,
    iterations: u32,
) -> Result<()> {
    let drawing_area = BitMapBackend::new(graph_path, (WIDTH, HEIGHT)).into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&drawing_area)
        .caption("Order parameter vs time", ("serif", FONT_SIZE).into_font())
        .margin(MARGIN_SIZE)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE)
        .build_cartesian_2d(0u32..iterations, 0f64..1f64)?;

    ctx.configure_mesh().x_desc("t").y_desc("va").draw()?;

    ctx.configure_mesh().x_desc("va").y_desc("t").draw()?;
    ctx.draw_series(LineSeries::new(orders_list.into_iter(), &BLUE))?;
    Ok(())
}
