extern crate rand;
use rand::Rng;

const NSTEPS: usize = 1000;
const TIME_HORIZON: f64 = 1.0;
const DELTA_T: f64 = TIME_HORIZON / NSTEPS as f64;

struct HestonModel {
    mu: f64,    // Expected return of the asset
    kappa: f64, // Rate of mean reversion for volatility
    theta: f64, // Long-term mean variance
    sigma: f64, // Volatility of volatility
    rho: f64,   // Correlation between asset price and volatility (set to 0 for simplicity)
    s0: f64,    // Initial asset price
    v0: f64,    // Initial variance (volatility squared)
}

impl HestonModel {
    fn simulate(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let mut s = self.s0;
        let mut v = self.v0;
        let mut prices = Vec::new();
        prices.push(s);
        for _ in 0..NSTEPS {
            let w_s = rng.gen::<f64>().sqrt() * rng.gen::<f64>();
            let w_v = rng.gen::<f64>().sqrt() * rng.gen::<f64>();
            v += self.kappa * (self.theta - v) * DELTA_T  + self.sigma * v.sqrt() * w_v * DELTA_T.sqrt();
            if v < 0.0 {
                v = 0.0; // vol can't be negative
            }
            s += self.mu * s * DELTA_T + s * v.sqrt() * w_s * DELTA_T.sqrt(); // Euler-Maruyama discretiziation/numerical method update
            prices.push(s);
        }
        prices
    }
}

fn main() {
    let heston = HestonModel {
        mu: 0.05,    // 5% expected return
        kappa: 2.0,  // Mean reversion speed
        theta: 0.02, // Long-term volatility (variance)
        sigma: 0.1, // Vol of vol
        rho: 0.0,  // No correlation for simplicity
        s0: 100.0, // Initial underlying price
        v0: 0.04, // Initial variance
    };
    let prices = heston.simulate();
    for (i, price) in prices.iter().enumerate() {
        println!("Day {}: ${:.2}", i, price);
    }
}
