use ndarray::Array1;
use plotlib::{page::Page, repr::Plot, style::LineStyle, view::ContinuousView};

fn main() {
    // create array variable t from 0 to 10 with 100 elements
    let t = Array1::linspace(0., 10., 100).into_shape((100, 1)).unwrap();
    // create array y equal to the sine of t
    let y = t.map(|t: &f64| t.sin());

    let data: Vec<(f64, f64)> = t.iter().map(|t: &f64| (*t, t.sin())).collect();

    let s1: Plot = Plot::new(data).line_style(LineStyle::new().colour("#DD3355"));

    // show the plot
    let v = ContinuousView::new()
        .add(s1)
        .x_range(0., 10.)
        .y_range(-1., 1.);
    Page::single(&v).save("scatter.svg").unwrap();
}
