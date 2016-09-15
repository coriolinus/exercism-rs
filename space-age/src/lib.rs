// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::seconds_per_year()
    }

    fn seconds_per_year() -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn seconds_per_year() -> f64 {
        // earth years per orbit * seconds per earth year = seconds per orbit
        0.2408467 * 31557600.0
    }
}
impl Planet for Venus {
    fn seconds_per_year() -> f64 {
        // earth years per orbit * seconds per earth year = seconds per orbit
        0.61519726 * 31557600.0
    }
}
impl Planet for Earth {
    fn seconds_per_year() -> f64 {
        // earth years per orbit * seconds per earth year = seconds per orbit
        31557600.0
    }
}
impl Planet for Mars {
    fn seconds_per_year() -> f64 {
        // earth years per orbit * seconds per earth year = seconds per orbit
        1.8808158 * 31557600.0
    }
}
impl Planet for Jupiter {
    fn seconds_per_year() -> f64 {
        // earth years per orbit * seconds per earth year = seconds per orbit
        11.862615 * 31557600.0
    }
}
impl Planet for Saturn {
    fn seconds_per_year() -> f64 {
        // earth years per orbit * seconds per earth year = seconds per orbit
        29.447498 * 31557600.0
    }
}
impl Planet for Uranus {
    fn seconds_per_year() -> f64 {
        // earth years per orbit * seconds per earth year = seconds per orbit
        84.016846 * 31557600.0
    }
}
impl Planet for Neptune {
    fn seconds_per_year() -> f64 {
        // earth years per orbit * seconds per earth year = seconds per orbit
        164.79132 * 31557600.0
    }
}
