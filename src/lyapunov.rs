use image::ImageBuffer;

pub struct Lyapunov {
    sequence: String,
    n: usize,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    width: u32,
    height: u32,
    output: String,
    x0: f32,
}

impl Lyapunov {
    pub fn new(
        sequence: String,
        n: usize,
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
        width: u32,
        height: u32,
        output: String,
    ) -> Self {
        let info = format!(
            "Generating Lyapunov fractal with following parameters.\n\
             domain:           {}, {}\n\
             range:            {}, {}\n\
             sequence:         {}\n\
             n (iterations):   {}\n\n\
             Output image properties.\n\
             name:             {}\n\
             resolution:       {}x{}\n",
            x_min, x_max, y_min, y_max, sequence, n, output, width, height
        );
        println!("{}", info,);

        Lyapunov {
            sequence,
            n,
            x_min,
            x_max,
            y_min,
            y_max,
            width,
            height,
            output,
            x0: 0.5,
        }
    }

    fn map(&self, r: f32, xn: f32) -> f32 {
        r * xn * (1.0 - xn)
    }

    fn get_r(&self, i: usize, a: f32, b: f32) -> f32 {
        if self.sequence.chars().nth(i % self.sequence.len()) == Some('a') {
            return a;
        }
        b
    }

    fn exponent(&self, a: f32, b: f32) -> f32 {
        let mut x: f32 = self.x0;
        let mut avg: f32 = 0.0;

        for i in 0..self.n {
            let r: f32 = self.get_r(i, a, b);
            x = self.map(r, x);
            let val = (r * (1.0 - 2.0 * x)).abs();

            if val != 0.0 {
                avg += val.ln();
            }
        }

        avg / self.n as f32
    }

    pub fn generate_image(&self) {
        let mut imgbuf = ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let x_length: f32 = self.x_max - self.x_min;
            let y_length: f32 = self.y_max - self.y_min;
            let a = (y_length / self.height as f32) * (self.height - y) as f32 - self.y_min.abs();
            let b = (x_length / self.width as f32) * x as f32 - self.x_min.abs();

            let mut red: u8 = 0;
            let mut blue: u8 = 0;

            let lambda = self.exponent(a, b);
            // to scale the r,g,b values
            let scale = (x_length).max(y_length);
            if lambda > 0.0 {
                blue = (255.0 - lambda * 256.0 / scale) as u8;
            } else if lambda < 0.0 {
                red = (255.0 + lambda * 256.0 / scale) as u8;
            } else {
                red = 255;
            }

            // red + green makes yellow
            *pixel = image::Rgb([red, red, blue]);
        }

        match imgbuf.save(&self.output) {
            Ok(_t) => {
                println!("Fractal generated successfully.")
            }
            Err(e) => {
                println!("Error occured {}", e)
            }
        }
    }
}
