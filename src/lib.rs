mod anonymiser;
mod regex_matcher;
mod info_types;

use regex::Regex;

trait InfoType {
    fn compile(&self) -> Regex;
}

pub struct AusPassport{
    name: String,
    expr: String
}

impl InfoType for AusPassport {
    fn compile(&self) -> Regex {
        Regex::new(self.expr.as_str()).unwrap()
    }
}

impl Default for AusPassport {
    fn default() -> Self {
        AusPassport{
            name: "AusPassport".to_string(),
            expr: "[A-Z][0-9]{7}".to_string()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aus_passport() {
        let passport = AusPassport::default();
        let compiled = passport.compile();
        let matched = compiled.is_match("A4365475");
        assert_eq!(matched, true);
    }
}
