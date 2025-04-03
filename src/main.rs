#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

struct BarChart {
    values: Vec<f64>,
    max_width: usize,
    colors: Vec<&'static str>,
}

impl BarChart {
    fn new(values: Vec<f64>, max_width: usize) -> Self {
        let colors = vec![
            "\x1b[41m",
            "\x1b[42m",
            "\x1b[43m",
            "\x1b[44m",
            "\x1b[45m",
            "\x1b[46m",
            "\x1b[47m",
            "\x1b[101m",
            "\x1b[102m",
            "\x1b[103m",
        ];
        BarChart {
            values,
            max_width,
            colors,
        }
    }

    fn render(&self) {
        let max_value = self.values.iter().fold(0.0, |a: f64, &b: &f64| a.max(b));
        let reset_color = "\x1b[0m";

        for (i, &value) in self.values.iter().enumerate() {
            // Print left side number (1, 2, 3, ...)
            print!("{:2} ", i + 1);

            // Calculate bar width
            let bar_width = if max_value > 0.0 {
                (value / max_value * self.max_width as f64).round() as usize
            } else {
                0
            };

            // Get color (cycle through colors if needed)
            let color = self.colors[i % self.colors.len()];

            // Print the bar
            print!("{color}");
            for _ in 0..bar_width {
                print!(" ");
            }
            print!("{reset_color}");

            // Print the value on the right side
            println!(" {value:.1}");
        }
    }
}

fn main() {
    let values = vec![15.0, 30.0, 10.0, 45.0, 25.0, 60.0, 35.0, 34.];
    let chart = BarChart::new(values, 50); // 50 is the maximum width in characters

    println!("Bar Chart:");
    chart.render();
}
