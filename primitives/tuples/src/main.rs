use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

impl Matrix {
    fn reverse(&self) -> Matrix {
        matrix_reverse(self)
    }
}

fn reverse(tuple: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = tuple;
    (boolean, integer)
}

fn matrix_reverse(matrix: &Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    let long_tuple: (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, char, bool) =
        (1, 2, 3, 4, -1, -2, -3, -4, 0.1, 0.2, 'a', true);

    let tuple_1: (i32, bool) = (42, false);
    println!("Tuple 1: {tuple_1:?}");

    let tuple_1: (bool, i32) = reverse(tuple_1);
    println!("Tuple 1: {tuple_1:?}");

    let matrix_1: Matrix = Matrix(1.0, 2.0, 3.0, 4.0);
    println!("Matrix 1: {matrix_1:?}");

    println!("Matrix 1: {matrix_1}");

    let matrix_2: Matrix = matrix_reverse(&matrix_1);
    println!("Matrix 1 transposed: {matrix_2}");

    println!("Matrix 1: {matrix_1}");

    let matrix_3: Matrix = matrix_2.reverse();
    println!("Matrix 1 transposed transposed: {matrix_3}")
}
