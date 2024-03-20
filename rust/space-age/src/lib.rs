// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#[macro_export]
macro_rules! as_planet {
    ($v:vis $exp: ident $t:ident) => {
       $v $exp $t;
       impl Planet for $t {

        fn years_during(d: &Duration) -> f64{
            let name = stringify!( $t);
            let period:f64 = match name{
                "Mercury" =>  0.2408467_f64,
                "Venus" =>    0.61519726_f64,
                "Mars" =>     1.8808158_f64,
                "Jupiter" =>  11.862615_f64,
                "Saturn" =>   29.447498_f64,
                "Uranus" =>   84.016846_f64,
                "Neptune" =>  164.79132_f64,
                _ =>         1.0_f64,
            };
            let y:f64 = 31557600_f64 * period;
            ((d.seconds /  y) * 100.0).round() / 100.0
        }
       }
    };
}

#[derive(Debug)]
pub struct Duration {
    // struct = state.
    seconds: f64,
}

// keep it simple dummy
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64; // can not think of reasonble default implemention
}

// units
as_planet! {pub struct Mercury}
as_planet! {pub struct Venus}
as_planet! {pub struct Earth}
as_planet! {pub struct Mars}
as_planet! {pub struct Jupiter}
as_planet! {pub struct Saturn}
as_planet! {pub struct Uranus}
as_planet! {pub struct Neptune}
