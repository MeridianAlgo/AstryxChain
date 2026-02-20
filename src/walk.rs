use std::f64::consts::PI;

pub const NODES: usize = 256;

#[derive(Clone, Copy, Debug, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    #[inline]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[inline]
    pub fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.re * rhs, self.im * rhs)
    }
}

impl std::ops::Mul<Complex> for f64 {
    type Output = Complex;

    #[inline]
    fn mul(self, rhs: Complex) -> Self::Output {
        rhs * self
    }
}

pub fn evolve(message: &[u8]) -> [Complex; NODES] {
    let mut state = [Complex::default(); NODES];
    let mut prior = [Complex::default(); NODES];
    state[0] = Complex::new(1.0, 0.0);
    prior[0] = Complex::new(1.0, 0.0);

    for (step, &byte) in message.iter().enumerate() {
        let h = (byte & 0x03) as usize;
        let (c, s) = if step % 2 == 0 {
            let v = std::f64::consts::FRAC_1_SQRT_2;
            (v, v)
        } else {
            let theta = (h as f64) * (PI / 4.0);
            (theta.cos(), theta.sin())
        };

        let mut next = [Complex::default(); NODES];

        for i in 0..NODES {
            let left = state[(i + NODES - 1) % NODES];
            let right = state[(i + 1) % NODES];
            let hop_p = state[(i + h) % NODES];
            let hop_m = state[(i + NODES - h) % NODES];

            let up = (c * left) + (s * right);
            let down = (s * left) - (c * right);
            let lively = (hop_p - hop_m) * 0.25;
            next[i] = (up + down) * 0.5 + lively;
        }

        if step % 2 == 0 {
            let current = next;
            for i in 0..NODES {
                next[i] = current[i] * 0.7 + prior[i] * 0.3;
            }
            prior = current;
        }

        normalize(&mut next);
        state = next;
    }

    state
}

fn normalize(state: &mut [Complex; NODES]) {
    let mut norm_sq = 0.0f64;
    for amp in state.iter() {
        norm_sq += amp.norm_sqr();
    }

    if norm_sq <= f64::EPSILON || !norm_sq.is_finite() {
        state.fill(Complex::default());
        state[0] = Complex::new(1.0, 0.0);
        return;
    }

    let inv = norm_sq.sqrt().recip();
    for amp in state.iter_mut() {
        *amp = *amp * inv;
    }
}

pub fn quantize(state: &[Complex; NODES]) -> [u8; NODES] {
    let mut out = [0u8; NODES];
    for (i, amp) in state.iter().enumerate() {
        let p = amp.norm_sqr();
        let q = (p * 256.0).floor();
        out[i] = q.clamp(0.0, 255.0) as u8;
    }
    out
}
