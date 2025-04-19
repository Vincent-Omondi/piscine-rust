use std::ops::{Add, Mul};

pub trait Scalar: Clone + Default + Add<Output = Self> + Mul<Output = Self> {}
impl<T: Clone + Default + Add<Output = T> + Mul<Output = T>> Scalar for T {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }
        let sum_vec = self.0
            .into_iter()
            .zip(other.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();
        Some(Vector(sum_vec))
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
        let mut sum = T::default();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            sum = sum + (a.clone() * b.clone());
        }
        Some(sum)
    }
}