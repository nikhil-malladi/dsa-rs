use std::f64::consts;

trait Shape {
    fn area(&self) -> f64;
}

pub struct Rectangle {
    width: f64,
    height: f64
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub struct Circle {
    radius: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        consts::PI * self.radius * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_trait() {
        let rect: Rectangle = Rectangle {width: 3.0, height: 4.0};
        let circle: Circle = Circle {radius: 4.0};

        assert_eq!(12.0,rect.area());
        assert_eq!(50.26548245743669,circle.area());
    }
}
