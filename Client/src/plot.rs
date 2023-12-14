use plotters::backend::BitMapBackend;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::full_palette::BLACK;
use plotters::prelude::full_palette::RED;
use plotters::prelude::full_palette::WHITE;
use plotters::prelude::ChartBuilder;
use plotters::prelude::LineSeries;
use plotters::prelude::PathElement;
use plotters::style::BLUE;
use plotters::style::Color;
use plotters::style::IntoFont;

use crate::readers::UsageReport;

pub fn plot_graph(rel_data: UsageReport) {
    let root = BitMapBackend::new("relatorioConsumo.png", (900, 600)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let xstart: chrono::prelude::NaiveDateTime = rel_data.start_time;
    let xend: chrono::prelude::NaiveDateTime = rel_data.end_time;
    let duration = xend.signed_duration_since(xstart);
    let runtime_label = format!(
        "Relatório de consumo - Tempo de execução: {:?} secs",
        duration.num_seconds()
    );
    let mut chart = ChartBuilder::on(&root)
        .caption(runtime_label, ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f32..rel_data.cpu_usage.len() as f32, 0f32..100f32)
        .unwrap();

    let _ = chart.configure_mesh().x_desc("Ticks").draw();

    chart
        .draw_series(LineSeries::new(
            rel_data
                .cpu_usage
                .iter()
                .enumerate()
                .map(|(i, &x)| (i as f32, 100.00 * x)),
            &RED,
        ))
        .unwrap()
        .label("Uso da CPU %")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            rel_data
                .ram_usage
                .iter()
                .enumerate()
                .map(|(i, &x)| (i as f32, 100.00 * x)),
            &BLUE,
        ))
        .unwrap()
        .label("Uso da RAM %")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();

    let _ = root.present();
}
