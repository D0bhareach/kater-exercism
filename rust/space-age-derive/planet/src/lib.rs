use time::Duration;
#[allow(unused_variables)]
pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        0f64 // not particularly sure I need to make default impl
    }
}
