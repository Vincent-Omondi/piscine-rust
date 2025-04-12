#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        // If r matches the first value, swap it with the corresponding component
        if self.r == first {
            self.r = second;
        } else if self.r == second {
            self.r = first;
        }
        
        // If g matches the first value, swap it with the corresponding component
        if self.g == first {
            self.g = second;
        } else if self.g == second {
            self.g = first;
        }
        
        // If b matches the first value, swap it with the corresponding component
        if self.b == first {
            self.b = second;
        } else if self.b == second {
            self.b = first;
        }
        
        // If a matches the first value, swap it with the corresponding component
        if self.a == first {
            self.a = second;
        } else if self.a == second {
            self.a = first;
        }
        
        self
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
