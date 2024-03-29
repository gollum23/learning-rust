use std::fmt::Display;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct USD(i32);

impl Display for USD {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let r = (self.0 as f32) / 100.0;

        if r < 0. {
            return write!(f, "-${:.2}", -r);
        }
        write!(f, "${:.2}", r)
    }
}

impl Clone for USD {
    fn clone(&self) -> USD {
        USD(self.0)
    }
}

impl Copy for USD {}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let u = USD(230);
        let b = u.clone();
        let c = u;

        assert_eq!(u, b);
        assert_eq!(u, c);
        assert_eq!(b, c);

        assert_eq!(u.to_string(), "$2.30");
    }
}
