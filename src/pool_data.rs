use serde::Desrialize;
use std::fs;

#[derive(Desrialize)]
pub struct PermanentData {
    pub five_character: Vec<String>,
    pub five_weapon: Vec<String>,
    pub four_characrer: Vec<String>,
    pub four_weapon: Vec<String>,
    pub three: Vec<String>,
}

