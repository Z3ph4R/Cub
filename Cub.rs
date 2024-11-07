use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("output.png", (600, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Rectangles", ("sans-serif", 20).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-5..15, -5..15)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(std::iter::once(Rectangle::new(
        [(1, 2), (5, 9)],
        ShapeStyle {
            color: RED.mix(0.5),
            filled: true,
            stroke_width: 2,
        },
    )))?;

    chart.draw_series(std::iter::once(Rectangle::new(
        [(3, 5), (12, 7)],
        ShapeStyle {
            color: GREEN.mix(0.5),
            filled: true,
            stroke_width: 2,
        },
    )))?;

    chart.draw_series(std::iter::once(Rectangle::new(
        [(8, 1), (13, 10)],
        ShapeStyle {
            color: BLUE.mix(0.5),
            filled: true,
            stroke_width: 2,
        },
    )))?;

    root.present()?;
    println!("Рисунок сохранён в файл output.png");

    Ok(())
}
