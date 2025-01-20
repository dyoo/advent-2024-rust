use std::cmp::Ordering;
use std::cmp::PartialOrd;

#[derive(Debug, PartialEq)]
struct Point(u32, u32);

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other.0 > self.0 || other.1 > self.1 {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {}

fn main() {
    println!("Hello world");
}
