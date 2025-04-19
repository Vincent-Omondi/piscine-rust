use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar<Item = T>>(pub Vec<T>);

use std::ops::Add;

impl<T: Scalar<Item = T> + Add<Output = T>> Add<Self> for Vector<T> {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let result: Vector<T> = Vector(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(x, y)| x.clone() + y.clone())
                .collect(),
        );
        Some(result)
    }
}

impl<T: Scalar<Item = T> + std::iter::Sum<<T as std::ops::Mul>::Output>> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let result = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(x, y)| x.clone() * y.clone())
            .sum();

        Some(result)
    }
}