use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Structure: {}", self.0)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point2D: x = {} y = {}", self.x, self.y)
    }
}

// Activity

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}
fn main() {
    let structure: Structure = Structure(32);
    println!("{structure} ");

    let point_2d: Point2D = Point2D { x: 2.4, y: 5.6 };

    println!("{point_2d:#?}");
    println!("{point_2d}");

    // Activity

    let complex: Complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {complex}");
    println!("Debug: {complex:?}");
}
