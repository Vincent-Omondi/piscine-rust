use std::fmt;
use std::ops::Mul;

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix({:?})", self.0)
    }
}

impl<T: Clone> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Matrix(self.0.clone())
    }
}

impl<T: Clone> Matrix<T> {
    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Clone + Default + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Self::Output {
        let a_rows = self.number_of_rows();
        let a_cols = self.number_of_cols();
        let b_rows = other.number_of_rows();
        let b_cols = other.number_of_cols();

        if a_cols != b_rows {
            return None;
        }

        let mut result = Vec::with_capacity(a_rows);
        for i in 0..a_rows {
            let mut row = Vec::with_capacity(b_cols);
            for j in 0..b_cols {
                let mut sum = T::default();
                for k in 0..a_cols {
                    let prod = self.0[i][k].clone() * other.0[k][j].clone();
                    sum = sum + prod;
                }
                row.push(sum);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}
