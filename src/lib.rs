//use num::complex::Complex;
#[derive(Debug, PartialEq, Clone, Copy)]
struct Complex {
    pub real: f64,
    pub img: f64,
}

impl Complex {
    pub fn magnitude(&self) -> f64 {
        (self.real.powi(2) + self.img.powi(2)).sqrt()
    }
}

fn complex_add(z1: &Complex, z2: &Complex) -> Complex {
    Complex {
        real: z1.real + z2.real,
        img: z1.img + z2.img,
    }
}

fn complex_mult(z1: &Complex, z2: &Complex) -> Complex {
    Complex {
        real: z1.real * z2.real - z1.img * z2.img,
        img: z1.real * z2.img + z1.img * z2.real,
    }
}

fn mandelbrot(z: Complex, n: u32) -> u32 {
    let mut diverge_count: u32 = 0;

    let mut z1 = z;
    while diverge_count <= n {
        println!("{}", z1.magnitude());

        if z1.magnitude() > 2. {
            return diverge_count;
        }

        z1 = complex_add(&complex_mult(&z1, &z1), &z);
        diverge_count += 1;
    }

    // if z hasn't deiverged by the end
    return n;
}

use plotters::prelude::*;
use std::ops::Range;

const OUT_FILE_NAME: &'static str = "mandelbrot.png";
fn draw_mandelbrot() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (800, 600)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20 as i32)
        .x_label_area_size(10 as i32)
        .y_label_area_size(10 as i32)
        .build_cartesian_2d(-2.1f64..0.6f64, -1.2f64..1.2f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let range = plotting_area.get_pixel_range();

    let samples = (range.0.end - range.0.start, range.1.end - range.1.start);
    let (real, complex) = (chart.x_range(), chart.y_range());

    let step = (
        (real.end - real.start) / samples.0 as f64,
        (complex.end - complex.start) / samples.1 as f64,
    );

    const NUM_CONVERGE: u32 = 100;

    for k in 0..(samples.0 * samples.1) {
        let z = Complex {
            real: real.start + step.0 * (k % samples.0) as f64,
            img: complex.start + step.1 * (k / samples.0) as f64,
        };
        let count = mandelbrot(z, NUM_CONVERGE);

        if count != NUM_CONVERGE {
            plotting_area.draw_pixel((z.real, z.img), &HSLColor(count as f64 / 100.0, 1.0, 0.5))?;
        } else {
            plotting_area.draw_pixel((z.real, z.img), &BLACK)?;
        }
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let z1 = Complex {
            real: -2.5,
            img: 4.,
        };
        let z2 = Complex {
            real: 5.5,
            img: 1.5,
        };
        assert_eq!(complex_add(&z1, &z2), Complex { real: 3., img: 5.5 });
    }

    fn mult_test() {
        let z1 = Complex { real: -2., img: 4. };
        let z2 = Complex {
            real: 2.5,
            img: 0.5,
        };
        assert_eq!(complex_mult(&z1, &z2), Complex { real: -7., img: 9. });
    }

    fn magnitude_test() {
        let z = Complex { real: -3., img: 4. };
        assert_eq!(z.magnitude(), 5.);
    }

    #[test]
    fn mandelbrot_test() {
        let z = Complex {
            real: 0.25,
            img: 0.75,
        };
        let count = mandelbrot(z, 20);
        println!("{}", count);
        assert!(true);
    }

    #[test]
    fn draw_mandelbrot_test() {
        draw_mandelbrot().unwrap()
    }
}
