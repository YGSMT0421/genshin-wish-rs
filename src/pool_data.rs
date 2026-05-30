use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct PermanentData {
    pub five_character: Vec<String>,
    pub five_weapon: Vec<String>,
    pub four_character: Vec<String>,
    pub four_weapon: Vec<String>,
    pub three: Vec<String>,
}
