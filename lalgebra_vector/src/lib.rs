use std::fmt::Debug;
use std::ops::{Add, Mul, AddAssign};

// Define a Scalar trait to restrict what types can be used in our Vector
pub trait Scalar: Debug + Clone + Copy + Add<Output = Self> + Mul<Output = Self> + AddAssign + PartialEq {}

// Implement Scalar for common numeric types
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for f32 {}
impl Scalar for f64 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return Vector(Vec::new());
        }

        let mut result = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            result.push(self.0[i] + other.0[i]);
        }
        Vector(result)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = None;
        for i in 0..self.0.len() {
            let product = self.0[i] * other.0[i];
            if let Some(value) = result {
                result = Some(value + product);
            } else {
                result = Some(product);
            }
        }
        result
    }
}

// Add reference implementation to avoid consuming vectors
impl<'a, 'b, T: Scalar> Add<&'b Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: &'b Vector<T>) -> Self::Output {
        if self.0.len() != other.0.len() {
            return Vector(Vec::new());
        }

        let mut result = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            result.push(self.0[i] + other.0[i]);
        }
        Vector(result)
    }
}