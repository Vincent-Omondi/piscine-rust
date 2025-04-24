pub trait Scalar {
    type Item;
    fn zero() -> Self;
    fn one() -> Self;
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
}

// Implement Scalar for common numeric types
impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
    fn add(&self, other: &Self) -> Self { self + other }
    fn mul(&self, other: &Self) -> Self { self * other }
}

impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn add(&self, other: &Self) -> Self { self + other }
    fn mul(&self, other: &Self) -> Self { self * other }
}

// Add implementation for u32 since it's used in tests
impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
    fn add(&self, other: &Self) -> Self { self + other }
    fn mul(&self, other: &Self) -> Self { self * other }
}

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = vec![vec![T::zero(); n]; n];
        
        for i in 0..n {
            matrix[i][i] = T::one();
        }
        
        Matrix(matrix)
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }
}

// Add PartialEq implementation for Matrix
impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// Debug implementation for Matrix
impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix({:?})", self.0)
    }
}