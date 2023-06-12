use regex::Regex;
use info_types::InfoType;
use crate::info_types;
use std::collections::HashMap;


pub struct RegexMatcher {
    pub expr: Regex,
    pub info_types: Vec<InfoType>,
    pub info_type_map: HashMap<String, InfoType>,
    pub info_type_names: Vec<String>,
    pub name: String,
    pub word_boundary: bool,
}

pub struct Position {
    pub start: usize,
    pub end: usize
}

impl RegexMatcher {
    pub fn get_positions_for_expr(expr: &Regex, text: &str) -> Vec<Position> {
        let mut positions = Vec::new();
        for mat in expr.find_iter(text) {
            positions.push(Position {
                start: mat.start(),
                end: mat.end(),
            });
        }
        positions
    }

    pub fn create_sub_expression(info_type: &InfoType, i: usize) -> String {
        let mut builder = String::new();
        if i > 0 {
            builder.push_str("|");
        }
        builder.push_str(&format!("(?P<{}>", info_type.name));
        if info_type.word_boundary {
            builder.push_str("\\b");
        }
        builder.push_str(&info_type.expr);
        builder.push_str(")");
        builder
    }

    pub fn get_unique_matches(&self, text: &str) -> Vec<String> {
        let mut unique_matches: HashMap<_, _> = self.expr.find_iter(text).map(|m| (m.as_str().to_owned(), ())).collect();
        unique_matches.keys().cloned().collect()
    }

    pub fn get_group_name(&self, text: &str) -> (bool, String) {
        let caps = self.expr.captures(text);
        match caps {
            Some(caps) => {
                for (idx, name) in self.info_type_names.iter().enumerate() {
                    if let Some(mat) = caps.name(name) {
                        return (true, mat.as_str().to_string());
                    }
                }
                (false, String::new())
            }
            None => (false, String::new()),
        }
    }

    pub fn match_fn(&self, text: &str) -> Vec<Match> {
        let mut matches = Vec::new();
        let matches_as_string = self.get_unique_matches(text);

        for match_ in &matches_as_string {
            let expr = Regex::new(match_).expect("Failed to compile regex");
            let (ok, name) = self.get_group_name(match_);
            if ok {
                let positions = RegexMatcher::get_positions_for_expr(&expr, text);
                matches.push(Match {
                    text: match_.clone(),
                    positions,
                    info_type: self.info_type_map[&name].clone(),
                });
            }
        }
        matches
    }
}

pub fn new(info_types: Vec<InfoType>) -> RegexMatcher {
    let mut builder = String::new();
    for (i, inf) in info_types.iter().enumerate() {
        let sub_expr = RegexMatcher::create_sub_expression(inf, i);
        builder.push_str(&sub_expr);
    }

    let r = Regex::new(&builder).expect("Unable to add create regex expression from string");

    let mut info_type_map = HashMap::new();
    for inf in &info_types {
        info_type_map.insert(inf.name.clone(), inf.clone());
    }
    let names = r.capture_names().map(|n| n.unwrap().to_owned()).collect();
    RegexMatcher {
        expr: r,
        info_types: info_types,
        info_type_map: info_type_map,
        info_type_names: names,
        name: String::new(),
        word_boundary: false,
    }
}
