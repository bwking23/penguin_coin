use std::{fmt, ops};
use thiserror::Error;

type Result<T> = std::result::Result<T, PointError>;

#[derive(Error, Debug, PartialEq)]
pub enum PointError {
    #[error("Invalid Point: ({0}, {1}) is not on the curve")]
    InvalidPoint(i64, i64),
    #[error("Both X and Y must both be None of both Some(i64)")]
    SingleInfinity,
    #[error("Points {0} and {1} are on different curves")]
    DifferentCurves(Point, Point),
    #[error("Unknown Addition for {0} and {1}")]
    UnknownAddition(Point, Point),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Point {
    x: Option<i64>,
    y: Option<i64>,
    a: i64,
    b: i64,
}

impl Point {
    pub fn new(x: Option<i64>, y: Option<i64>, a: i64, b: i64) -> Result<Self> {
        let p = Point { x, y, a, b };
        match (x, y) {
            (None, None) => Ok(p),
            (None, Some(_)) => Err(PointError::SingleInfinity),
            (Some(_), None) => Err(PointError::SingleInfinity),
            (Some(x1), Some(y1)) => {
                if y1.pow(2) != x1.pow(3) - a * x1 + b {
                    Err(PointError::InvalidPoint(y1, x1))
                } else {
                    Ok(p)
                }
            }
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x_display = match self.x {
            None => "Infinity".to_string(),
            Some(v) => v.to_string(),
        };
        let y_display = match self.y {
            None => "Infinity".to_string(),
            Some(v) => v.to_string(),
        };
        write!(
            f,
            "Point(x:{}, y:{}, a:{}, b:{})",
            x_display, y_display, self.a, self.b
        )
    }
}

impl ops::Add<Point> for Point {
    type Output = Result<Self>;

    fn add(self, other: Self) -> Result<Self> {
        if self.a != other.a || self.b != other.b {
            return Err(PointError::DifferentCurves(self, other));
        }
        if self.x.is_none() {
            return Ok(other);
        }
        if other.x.is_none() {
            return Ok(self);
        }
        if self.x == other.x && self.y != other.y {
            return Ok(Point {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            });
        }
        if self.x != other.x {
            let s = (other.y.unwrap() - self.y.unwrap()) / (other.x.unwrap() - self.x.unwrap());
            let x = s.pow(2) - self.x.unwrap() - other.x.unwrap();
            let y = s * (self.x.unwrap() - x) - self.y.unwrap();
            return Ok(Point {
                x: Some(x),
                y: Some(y),
                a: self.a,
                b: self.b,
            });
        }

        if self == other && self.y.unwrap() == 0 {
            return Ok(Point {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            });
        }
        if self == other {
            let s = (3 * self.x.unwrap().pow(2) + self.a) / (2 * self.y.unwrap());
            let x = s.pow(2) - 2 * self.x.unwrap();
            let y = s * (self.x.unwrap() - x) - self.y.unwrap();
            return Ok(Point {
                x: Some(x),
                y: Some(y),
                a: self.a,
                b: self.b,
            });
        }
        Err(PointError::UnknownAddition(self, other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point_ne() {
        let a = Point {
            x: Some(3),
            y: Some(-7),
            a: 5,
            b: 7,
        };
        let b = Point {
            x: Some(18),
            y: Some(77),
            a: 5,
            b: 7,
        };
        assert!(a != b);
        assert_eq!(a, a);
    }
    #[test]
    fn test_point_add() {
        let a = Point {
            x: None,
            y: None,
            a: 5,
            b: 7,
        };
        let b = Point {
            x: Some(2),
            y: Some(5),
            a: 5,
            b: 7,
        };
        let c = Point {
            x: Some(2),
            y: Some(-5),
            a: 5,
            b: 7,
        };
        assert_eq!((a + b).unwrap(), b);
        assert_eq!((b + a).unwrap(), b);
        assert_eq!((b + c).unwrap(), a);

        let a = Point {
            x: Some(3),
            y: Some(7),
            a: 5,
            b: 7,
        };
        let b = Point {
            x: Some(-1),
            y: Some(-1),
            a: 5,
            b: 7,
        };
        assert_eq!(
            (a + b).unwrap(),
            Point {
                x: Some(2),
                y: Some(-5),
                a: 5,
                b: 7
            }
        );

        let a = Point {
            x: Some(-1),
            y: Some(-1),
            a: 5,
            b: 7,
        };
        assert_eq!(
            (a + a).unwrap(),
            Point {
                x: Some(18),
                y: Some(77),
                a: 5,
                b: 7
            }
        );
    }
}
