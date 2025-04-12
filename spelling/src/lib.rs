/// Converts a number to its word representation
pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    // Handle numbers up to one million
    if n > 1_000_000 {
        return "one million".to_string();
    }

    let ones = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let teens = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

    let mut result = String::new();

    // Handle thousands (1,000 - 999,999)
    if n >= 1000 {
        let thousands = n / 1000;
        if thousands > 0 {
            result.push_str(&spell_hundreds(thousands));
            result.push_str(" thousand");
            
            // Check if there's a remainder
            if n % 1000 > 0 {
                result.push_str(" ");
                result.push_str(&spell_hundreds(n % 1000));
            }
            return result;
        }
    }

    // Handle hundreds and below
    spell_hundreds(n)
}

/// Helper function to spell numbers from 1-999
fn spell_hundreds(n: u64) -> String {
    let ones = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let teens = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

    let mut result = String::new();

    // Handle hundreds (100-999)
    if n >= 100 {
        result.push_str(ones[(n / 100) as usize]);
        result.push_str(" hundred");
        
        // Check if there's a remainder
        if n % 100 > 0 {
            result.push_str(" ");
        } else {
            return result;
        }
    }

    // Handle tens and ones (1-99)
    let remainder = n % 100;
    
    if remainder >= 20 {
        // Handle 20-99
        result.push_str(tens[(remainder / 10) as usize]);
        
        if remainder % 10 > 0 {
            result.push_str("-");
            result.push_str(ones[(remainder % 10) as usize]);
        }
    } else if remainder >= 10 {
        // Handle 10-19
        result.push_str(teens[(remainder - 10) as usize]);
    } else if remainder > 0 {
        // Handle 1-9
        result.push_str(ones[remainder as usize]);
    }

    result
}
