use regex::Regex;
use fake::{Fake, Faker};

pub struct InfoType {
    pub expr: String,
    pub name: String,
    pub word_boundary: bool,
    pub generate: fn() -> String,
}

impl InfoType {
    pub fn aus_drivers_licence() -> Self {
        let name = "AusDriversLicence".to_string();
        let expr = "[A-Z0-9][0-9]{5,7}".to_string();
        InfoType {
            expr: expr.clone(),
            name,
            word_boundary: false,
            generate: move || Faker.fake_with_regex(&expr).unwrap_or_default(),
        }
    }

    pub fn aus_passport() -> Self {
        let name = "AusPassport".to_string();
        let expr = "[A-Z][0-9]{7}".to_string();
        InfoType {
            expr: expr.clone(),
            name,
            word_boundary: false,
            generate: move || Faker.fake_with_regex(&expr).unwrap_or_default(),
        }
    }

    pub fn aus_license_plate() -> Self {
        let name = "AusLicencePlate".to_string();
        let expr = "(([a-zA-Z0-9]{3})([\\s,-.]?)([a-zA-Z0-9]{3}))".to_string();
        InfoType {
            expr: expr.clone(),
            name,
            word_boundary: false,
            generate: move || Faker.fake_with_regex(&expr).unwrap_or_default(),
        }
    }

    pub fn aus_tax_file_number() -> Self {
        let name = "AusTaxFileNumber".to_string();
        let expr = "[0-9]{3}( ?)[0-9]{3}[0-9]{2,3}".to_string();
        InfoType {
            expr: expr.clone(),
            name,
            word_boundary: false,
            generate: move || Faker.fake_with_regex(&expr).unwrap_or_default(),
        }
    }

    pub fn email() -> Self {
        let name = "Email".to_string();
        let expr = "([a-z0-9!#$%&'*+/=?^_{|}~-]+(?:\\.[a-z0-9!#$%&'*+/=?^_{|}~-]+)*|\"(?:[\\x01-\\x08\\x0b\\x0c\\x0e-\\x1f\\x21\\x23-\\x5b\\x5d-\\x7f]|\\[\\x01-\\x09\\x0b\\x0c\\x0e-\\x7f])*\")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\\x01-\\x08\\x0b\\x0c\\x0e-\\x1f\\x21-\\x5a\\x53-\\x7f]|\\[\\x01-\\x09\\x0b\\x0c\\x0e-\\x7f])+))\\]".to_string();
        InfoType {
            expr: expr.clone(),
            name,
            word_boundary: false,
            generate: || Faker.fake::<String>(),
        }
    }

    pub fn aus_post_code() -> Self {
        let name = "AusPostCode".to_string();
        let expr = "(0[289][0-9]{2}\\)|([1345689][0-9]{3})|(2[0-9][0-9]{2})|(290[0-9])|(291[0-9])|(7[0-4][0-9]{2})|(7[8-9][0-9]{2}))".to_string();
        InfoType {
            expr: expr.clone(),
            name,
            word_boundary: false,
            generate: move || Faker.fake_with_regex(&expr).unwrap_or_default(),
        }
    }

    pub fn long_digit(min_length: usize) -> Self {
        let name = "LongDigit".to_string();
        let expr = format!("\\d{{{}}}", min_length);
        InfoType {
            expr: expr.clone(),
            name,
            word_boundary: false,
            generate: || "".to_string(),
        }
    }
}
