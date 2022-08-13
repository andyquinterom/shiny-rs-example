use rand::distributions::Distribution;
use statrs::distribution::Normal;
use plotly::Histogram;
use plotly::Plot;
use plotly::Trace;

pub fn get_dist(n: usize, mean: f64, sd: f64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut r = rand::rngs::OsRng;
    let distribution = Normal::new(mean, sd)?;
    let vals: Vec<f64> = distribution.sample_iter(&mut r).take(n).collect();
    return Ok(vals)
}

pub fn get_plot(dist1: &Vec<f64>, dist2: &Vec<f64>) -> String {
    let plot_string: String;
    {
        let hist1 = Histogram::new(dist1.clone());
        let hist2 = Histogram::new(dist2.clone());
        let mut my_plot = Plot::new();
        my_plot.add_trace(hist1);
        my_plot.add_trace(hist2);
        plot_string = my_plot.to_inline_html("my_plot");
    }
    return plot_string
}

pub fn get_plot_simple(dist1: &Vec<f64>) -> String {
    let plot_string: String;
    {
        let hist1 = Histogram::new(dist1.clone());
        let mut my_plot = Plot::new();
        my_plot.add_trace(hist1);
        plot_string = my_plot.to_inline_html("my_plot");
    }
    return plot_string
}
