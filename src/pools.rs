use super::status::*;
use rand::Rng;

trait Pool {
    fn wish(&mut self) -> WishResult;

    fn wish_multitimes(&mut self, times: u32) -> Vec<WishResult> {
        let mut result = Vec::with_capacity(times as usize);
        for _ in 0..times {
            result.push(self.wish());
        }
        result
    }
}

pub struct CharacterPool {
    wanted_item: String,
    up_four_star: Vec<String>,
    guaranteed: bool,
    five_wish_times: u8,
    four_wish_times: u8,
}
