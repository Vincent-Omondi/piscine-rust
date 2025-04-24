use std::ops::{Add, Sub};
use std::fmt;

pub struct Matrix<T>(pub Vec<Vec<T>>);

// Add Debug implementation for Matrix
impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix({:?})", self.0)
    }
}

impl<T: Clone + std::ops::Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;
    
    fn add(self, other: Matrix<T>) -> Self::Output {
        // Check if matrices have the same dimensions
        let self_rows = self.0.len();
        let self_cols = if self_rows > 0 { self.0[0].len() } else { 0 };
        
        let other_rows = other.0.len();
        let other_cols = if other_rows > 0 { other.0[0].len() } else { 0 };
        
        // Return None if dimensions don't match
        if self_rows != other_rows || self_cols != other_cols {
            return None;
        }
        
        // Create a new matrix to store the result
        let mut result = Vec::with_capacity(self_rows);
        
        // Perform addition
        for i in 0..self_rows {
            let mut row = Vec::with_capacity(self_cols);
            for j in 0..self_cols {
                row.push(self.0[i][j].clone() + other.0[i][j].clone());
            }
            result.push(row);
        }
        
        Some(Matrix(result))
    }
}

impl<T: Clone + std::ops::Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;
    
    fn sub(self, other: Matrix<T>) -> Self::Output {
        // Check if matrices have the same dimensions
        let self_rows = self.0.len();
        let self_cols = if self_rows > 0 { self.0[0].len() } else { 0 };
        
        let other_rows = other.0.len();
        let other_cols = if other_rows > 0 { other.0[0].len() } else { 0 };
        
        // Return None if dimensions don't match
        if self_rows != other_rows || self_cols != other_cols {
            return None;
        }
        
        // Create a new matrix to store the result
        let mut result = Vec::with_capacity(self_rows);
        
        // Perform subtraction
        for i in 0..self_rows {
            let mut row = Vec::with_capacity(self_cols);
            for j in 0..self_cols {
                row.push(self.0[i][j].clone() - other.0[i][j].clone());
            }
            result.push(row);
        }
        
        Some(Matrix(result))
    }
}