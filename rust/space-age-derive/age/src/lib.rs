use planet::Planet;
use planet_macro_derive::Planet;
pub use time::Duration;

#[derive(Planet)]
pub struct Mercury;

#[derive(Planet)]
pub struct Venus;

#[derive(Planet)]
pub struct Earth;

#[derive(Planet)]
pub struct Mars;

#[derive(Planet)]
pub struct Jupiter;

#[derive(Planet)]
pub struct Saturn;

#[derive(Planet)]
pub struct Uranus;

#[derive(Planet)]
pub struct Neptune;
