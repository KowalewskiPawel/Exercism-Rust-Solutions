// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}
pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
     iter.enumerate().filter(|&(i, _)| i % 2 == 0 ).map(|(_, e)| e)}
pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        (self.0).abs() + (self.1).abs()
    }
}