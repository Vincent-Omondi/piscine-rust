use case::CaseExt;
use edit_distance::edit_distance;

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    let compare_aux = compare.to_lowercase();
    let expected_aux = expected.to_lowercase();

    if (compare_aux.to_ascii_lowercase() == compare_aux
        || compare_aux.to_camel_lowercase() == compare_aux)
        && !compare_aux.contains("-")
        && !compare_aux.contains(" ")
    {
        let distance = edit_distance(&compare_aux, &expected_aux) as i64;

        if distance == 0 {
            return Some("100%".to_string());
        }

        let percentage = 100 - (distance * 100 / expected.len() as i64);
        if percentage >= 50 {
            let mut res = percentage.to_string();
            res.push_str("%");
            return Some(res);
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn no_variable_case() {
        let mut result = expected_variable("It is simply not a variable case", "gonnaFail");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("do-not-use-dashes", "do-not-use-dashes");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("Not a variable case", "needs to fail");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("This should be None", "needs to fail");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("Do not use spaces", "Do not use spaces");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );
    }

    #[test]
    fn incorrect_names() {
        let mut result = expected_variable("it_is_done", "almost_there");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("frankenstein", "Dracula");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );
    }

    #[test]
    fn one_hundred_percent() {
        assert_eq!(
            "100%",
            expected_variable("great_variable", "great_variable").unwrap()
        );
        assert_eq!("100%", expected_variable("SpOtON", "SpotOn").unwrap());
        assert_eq!(
            "100%",
            expected_variable("Another_great_variable", "Another_great_variable").unwrap()
        );
    }

    #[test]
    fn passing_tests() {
        assert_eq!("88%", expected_variable("soClose", "so_close").unwrap());
        assert_eq!("73%", expected_variable("lets_try", "lets_try_it").unwrap());
        assert_eq!("64%", expected_variable("GoodJob", "VeryGoodJob").unwrap());
        assert_eq!("64%", expected_variable("GoodJob", "VeryGoodJob").unwrap());
        assert_eq!(
            "67%",
            expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
        );
    }
}