use super::Result;
use plotters::prelude::*;

const MARGIN_SIZE: u32 = 10;
const NO_LABEL_SIZE: u32 = 40;
const LABEL_AREA_SIZE: u32 = 60;
const HEIGHT: u32 = 720;
const WIDTH: u32 = 1280;
const LABEL_STYLE: (&str, u32) = ("serif", 20);

pub fn order_time_graph(graph_path: &str, orders_list: Vec<f64>) -> Result<()> {
    let drawing_area = BitMapBackend::new(graph_path, (WIDTH, HEIGHT)).into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&drawing_area)
        .margin(MARGIN_SIZE)
        .margin_top(NO_LABEL_SIZE)
        .margin_right(NO_LABEL_SIZE)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE)
        .build_cartesian_2d(0..orders_list.len(), 0f64..1f64)?;

    ctx.configure_mesh()
        .x_desc("Time (iterations)")
        .x_label_style(LABEL_STYLE)
        .y_desc("Order Parameter")
        .y_label_style(LABEL_STYLE)
        .draw()?;

    ctx.draw_series(LineSeries::new(orders_list.into_iter().enumerate(), &BLUE))?;
    Ok(())
}
