use super::status::*;
use super::pool_data::*;
use rand::{RngExt, IndexedRandom, ThreadRng};

// 算概率的函数
const fn prob90() -> [f64; 91] {
    let mut prob = [0.006f64; 91];

    // 0抽是无意义的，但是为了索引时无需每次减一，在这里填充一个不可能的概率-1.0
    prob[0] = -1.0;

    let mut now = 74usize;
    while now < 91 {
        prob[now] = ((now - 73) as f64) * 0.06 + 0.006;
        now += 1;
    }

    prob
}

const fn prob80() -> [f64; 81] {
    let mut prob = [0.007f64; 81];

    // 0抽是无意义的，但是为了索引时无需每次减一，在这里填充一个不可能的概率-1.0
    prob[0] = -1.0;

    let mut now = 63usize;
    while now < 74 {
        prob[now] = ((now - 62) as f64) * 0.07 + 0.007;
        now += 1;
    }
    while now < 81 {
        prob[now] = ((now - 73) as f64) * 0.035 + 0.777;
        now += 1;
    }

    prob
}

// 概率值
// 值得注意的是，根据前面两个函数给出的公式，第90/80抽时会出现概率>1的数据
// 考虑到rand::rng().random()只能产出[0,1)，这样子的概率在行为上与100%概率无异，故为了方便，不做修改。
// 当前抽数可直接当做索引传入，在index==0处填充了无意义数据。
static PROB_90: [f64; 91] = prob90();
static PROB_80: [f64; 81] = prob80();

// 卡池定义
// trait

pub trait Pool<Status> {
    fn wish(&mut self) -> WishResult<Status>;

    fn wish_multitimes(&mut self, times: u32) -> Vec<WishResult<Status>> {
        let mut result = Vec::with_capacity(times as usize);
        for _ in 0..times {
            result.push(self.wish());
        }
        result
    }
}

// 角色池

pub struct CharacterPool<'a> {
    wanted_item: String,
    up_four_star: Vec<String>,
    guaranteed: bool,
    five_wish_times: u8,
    four_wish_times: u8,
    off_banner_times: u8,
    permanent_data: &'a PermanentData
}

impl CharacterPool {
    pub fn new(wanted_item: String, up_four_star: Vec<String>) -> Self {
        Self {
            wanted_item,
            up_four_star,
            guaranteed: false,
            five_wish_times: 0,
            four_wish_times: 0,
            off_banner_times: 0,
        }
    }

    fn add_one_pity(&mut self) {
        self.five_wish_times += 1;
        self.four_wish_times += 1;
    }

    fn get_prob(&mut self) -> f64 {
        PROB_90[self.five_wish_times as usize]
    }
    
    fn get_five_result(item: String, status: CharacterStatus) -> WishResult<CharacterStatus> {
        WishResult {
            item_name: item,
            item_type: ItemType::Character,
            pool_type: PoolType::Character,
            rarity: Rarity::Five,
            wish_times: self.five_wish_times,
            five_status: Some(status),
            four_status: None,
        }
    }
    
    fn wish_get_five(rng: &mut ThreadRng) -> WishResult<CharacterStatus> {
        if self.guaranteed {
            // 大保底
            self.get_five_result(self.wanted_item.clone(), CharacterStatus::Guaranteed)
        } else if self.off_banner_times >= 3 {
            // 连歪三次触发捕获明光
            self.get_five_result(self.wanted_item.clone(), CharacterStatus::GuaranteedCapturingRadiance)
        } else if rng::random::<f64>() < 0.5f64 {
            // 小保底没歪
            self.get_five_result(self.wanted_item.clone(), CharacterStatus::Featured)
        } else if rng::random::<f64>() < 0.1f64 {
            // 小保底歪了但是触发捕获明光
            self.get_five_result(self.wanted_item.clone(), CharacterStatus::CapturingRadiance)
        } else {
            // 小保底歪了
            if let Some(character) = self.permanent_data.choose(&mut rng) {
                self.get_five_result(character.clone(), CharacterStatus::OffBanner)
            } else {
                eprintln!("Error: 未读取到常驻数据，将默认使用刻晴");
                self.get_five_result(String::from("刻晴"), CharacterStatus::OffBanner)
            }
        }
    }
    
    fn get_four_result(item: String, status: CharacterStatus) -> WishResult<CharacterStatus> {
        WishResult {
            item_name: item,
            item_type: ItemType::Character,
            pool_type: PoolType::Character,
            rarity: Rarity::Four,
            wish_times: self.five_wish_times,
            five_status: None,
            four_status: Some(CharacterStatus),
        }
    }
    
    fn wish_get_four(rng: &mut ThreadRng) -> WishResult<CharacterStatus> {
        if rng::random::<f64> < 0.051f64 {
            // up四星
            self.get_four_result(self.up_four_star.choose(&mut rng).clone(), FourStatus::GuaranteedRateUp)
        } else {
            // 常驻四星
            let character_len = self.permanent_data.four_character.len();
            let weapon_len = self.permanent_data.four_weapon.len();
            let total = character_len + weapon_len;
            let index = rng.rand_range(0..total);
            let // 随机索引
            // self.get_four_result()
        }
    }
}

impl Pool<CharacterStatus> for CharacterPool {
    fn wish(&mut self) -> WishResult<CharacterStatus> {
        self.add_one_pity();

        let mut rng = rand::rng();
        
        let result: WishResult<CharacterStatus> =
            // 出金部分
            if rng.random::<f64>() < self.get_prob() {
                self.wish_get_five(&mut rng)
            }
            // 出紫部分
            // 保底四星
            else if self.four_wish_times == 10u8 {
                self.get_four_result()
            }

        todo!()
    }
}

