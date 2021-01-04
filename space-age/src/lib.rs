const EARTH_YEAR_SECONDS: f64 = 31557600.0;

pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    const SECONDS_PER_YEAR: f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::SECONDS_PER_YEAR
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

// For each planet:
//
// earth years per orbit * seconds per earth year = seconds per orbit

impl Planet for Mercury {
    const SECONDS_PER_YEAR: f64 = 0.2408467 * EARTH_YEAR_SECONDS;
}
impl Planet for Venus {
    const SECONDS_PER_YEAR: f64 = 0.61519726 * EARTH_YEAR_SECONDS;
}
impl Planet for Earth {
    const SECONDS_PER_YEAR: f64 = 1.0 * EARTH_YEAR_SECONDS;
}
impl Planet for Mars {
    const SECONDS_PER_YEAR: f64 = 1.8808158 * EARTH_YEAR_SECONDS;
}
impl Planet for Jupiter {
    const SECONDS_PER_YEAR: f64 = 11.862615 * EARTH_YEAR_SECONDS;
}
impl Planet for Saturn {
    const SECONDS_PER_YEAR: f64 = 29.447498 * EARTH_YEAR_SECONDS;
}
impl Planet for Uranus {
    const SECONDS_PER_YEAR: f64 = 84.016846 * EARTH_YEAR_SECONDS;
}
impl Planet for Neptune {
    const SECONDS_PER_YEAR: f64 = 164.79132 * EARTH_YEAR_SECONDS;
}
