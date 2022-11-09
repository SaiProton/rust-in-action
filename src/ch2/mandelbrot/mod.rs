use num::complex::Complex;

pub fn execute() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 100, 24);

    render_mandelbrot(mandelbrot);
}

fn calculate_mandelbrot(
    max_iters: usize, // Max iterations to reach to be considered to be within the set.

    // Parameters that specify the space we're searching for to look for members of the set.
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,

    // Size of the output in pixels.
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    // Houses the data from each row.
    let mut rows: Vec<_> = Vec::with_capacity(width);

    // Iterates row by row
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);

        for img_x in 0..width {
            // Calculates the proportion of the space covered in our output and converts that to
            // points within the search space.
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;

            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }

    rows
}

// Called at every pixel.
fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    // Initializes a complex number at the origin.
    let mut z = Complex { re: 0.0, im: 0.0 };
    // Initializes a complex number from the coordinates provided as function arguments.
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        // Checks the escape condition (z's length larger than 2 meaning the value escaped).
        if z.norm() > 2.0 {
            return i;
        }

        // Repeatedly mutates z to check wether c lies within the Mandelbrot set
        z = z * z + c;
    }

    // As i is no longer in scope, we fall back to max_iters.
    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());

        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }

        println!("{}", line);
    }
}
