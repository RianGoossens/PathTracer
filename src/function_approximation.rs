use rand::Rng;

pub struct LinearInterpolator {
    xs: Vec<f64>,
    ys: Vec<f64>,
}

impl LinearInterpolator {
    pub fn build(xs: &[f64], ys: &[f64]) -> Self {
        let mut data: Vec<_> = xs.iter().zip(ys).collect();
        data.sort_by(|a, b| a.0.total_cmp(b.0));
        let (xs, ys) = data.into_iter().unzip();
        Self { xs, ys }
    }

    pub fn min_x(&self) -> f64 {
        self.xs[0]
    }

    pub fn max_x(&self) -> f64 {
        self.xs[self.xs.len() - 1]
    }

    pub fn apply(&self, x: f64) -> f64 {
        if x < self.xs[0] {
            self.ys[0]
        } else if x > self.xs[self.xs.len() - 1] {
            self.ys[self.ys.len() - 1]
        } else {
            for i in 0..self.xs.len() {
                if x < self.xs[i + 1] {
                    let weight_a = self.xs[i + 1] - x;
                    let weight_b = x - self.xs[i];
                    let total_weight = weight_a + weight_b;
                    return (self.ys[i] * weight_a + self.ys[i + 1] * weight_b) / total_weight;
                }
            }
            self.ys[self.ys.len() - 1]
        }
    }
}

#[derive(Clone)]
pub struct FunctionApproximation {
    start: f64,
    end: f64,
    step_size: f64,
    pub ys: Vec<f64>,
}

impl FunctionApproximation {
    pub fn build<F: Fn(f64) -> f64>(f: F, start: f64, end: f64, num_steps: usize) -> Self {
        assert!(
            end > start,
            "End {end} needs to be bigger than start {start}."
        );
        assert!(
            num_steps >= 2,
            "Num steps needs to be at least 2 but is {num_steps}"
        );
        let step_size = (end - start) / (num_steps - 1) as f64;

        let mut ys = vec![0.; num_steps];
        let mut x = start;
        for y in &mut ys {
            *y = f(x);
            x += step_size;
        }

        Self {
            start,
            end,
            step_size,
            ys,
        }
    }

    pub fn integrate(&self) -> Self {
        let mut ys = vec![0.; self.ys.len() + 1];
        let mut x = self.start;
        let mut subtotal = 0.;
        let mut previous_y = 0.;
        for y in &mut ys[1..] {
            x += self.step_size;
            let current_y = self.apply(x).unwrap_or_default();
            subtotal += (previous_y + current_y) / 2. * self.step_size;
            *y = subtotal;
            previous_y = current_y;
        }

        Self { ys, ..*self }
    }

    pub fn normalize(&self) -> Self {
        let ys = self
            .ys
            .iter()
            .map(|x| x / self.ys[self.ys.len() - 1])
            .collect();

        Self { ys, ..*self }
    }

    pub fn invert(&self) -> Self {
        let xs = (0..self.ys.len())
            .map(|i| self.start + i as f64 * self.step_size)
            .collect::<Vec<_>>();
        let inversion = LinearInterpolator::build(&self.ys, &xs);
        Self::build(
            |x| inversion.apply(x),
            inversion.min_x(),
            inversion.max_x(),
            self.ys.len(),
        )
    }

    pub fn apply(&self, x: f64) -> Option<f64> {
        if x < self.start {
            None
        } else {
            let center = (x - self.start) / self.step_size;
            let left = center.floor() as usize;

            if left >= self.ys.len() - 1 {
                Some(self.ys[self.ys.len() - 1])
            } else {
                let right_weight = center - left as f64;

                Some(self.ys[left] * (1. - right_weight) + self.ys[left + 1] * right_weight)
            }
        }
    }
}

#[derive(Clone)]
pub struct ProbabilityDensityFunction {
    pub pdf: FunctionApproximation,
    pub inverse_cdf: FunctionApproximation,
    max_density: f64,
}

impl ProbabilityDensityFunction {
    pub fn build<F: Fn(f64) -> f64>(f: F, num_steps: usize) -> Self {
        let unnormalized_pdf = FunctionApproximation::build(&f, 0., 1., num_steps);
        let unnormalized_cdf = unnormalized_pdf.integrate();
        let total_area = unnormalized_cdf.apply(1.).unwrap();

        let pdf = FunctionApproximation::build(|x| f(x) / total_area, 0., 1., num_steps);
        let cdf = pdf.integrate();
        let inverse_cdf = cdf.invert();

        let max_density = *pdf.ys.iter().max_by(|a, b| a.total_cmp(b)).unwrap();

        Self {
            pdf,
            inverse_cdf,
            max_density,
        }
    }

    pub fn likelihood(&self, value: f64) -> f64 {
        if value < self.pdf.start || value > self.pdf.end {
            0.
        } else {
            self.pdf.apply(value).unwrap() / self.max_density
        }
    }

    pub fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        let x = rng.gen();
        self.inverse_cdf.apply(x).unwrap()
    }
}
