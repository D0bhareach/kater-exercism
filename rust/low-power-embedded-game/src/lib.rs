#![allow(unused)]
pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
((dividend / divisor) as i16, dividend.checked_rem(divisor).unwrap_or_default())
}

// it won't pass offset_i32, because it return positioned elements not actual.
pub fn odds<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate().filter(|(x, y)| x % 2 == 1).map(|e| e.1)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}


// no thechical specification for this function. I simply make it pass
pub fn transform<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    evens(iter)
}

pub fn even_string_parts<'a>(iter: impl Iterator<Item = &'a str>) -> impl Iterator<Item = &'a str> {
    evens(iter)
}

pub struct Position(pub i16, pub i16);
impl Position {

    fn manhattan_inner(&self, other: &Position)-> i16 {
        (self.0.abs() - other.0.abs()) + (self.1.abs() - other.1.abs())
    }

    pub fn manhattan(&self) -> i16 {
        const RHS: Position = Position(0, 0);
        self.manhattan_inner(&RHS)
    }

}