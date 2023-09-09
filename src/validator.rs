pub fn is_cep_valid(s: &str) -> bool {
    let s = s.replace("-", "");
    s.len() == 8 && s.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_cep_with_hyphen() {
        let cep = "12345-678";
        assert!(is_cep_valid(cep));
    }

    #[test]
    fn valid_cep_without_hyphen() {
        let cep = "54321876";
        assert!(is_cep_valid(cep));
    }

    #[test]
    fn invalid_short_cep() {
        let cep = "12345";
        assert!(!is_cep_valid(cep));
    }

    #[test]
    fn invalid_long_cep() {
        let cep = "123456789";
        assert!(!is_cep_valid(cep));
    }

    #[test]
    fn invalid_non_numeric_cep() {
        let cep = "12A45-678";
        assert!(!is_cep_valid(cep));
    }

    #[test]
    fn empty_string() {
        let cep = "";
        assert!(!is_cep_valid(cep));
    }

    #[test]
    fn minimum_length_cep() {
        let cep = "00000000";
        assert!(is_cep_valid(cep));
    }

    #[test]
    fn maximum_length_cep() {
        let cep = "99999999";
        assert!(is_cep_valid(cep));
    }
}
