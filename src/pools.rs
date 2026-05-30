use super::pool_data::*;
use super::status::*;
use rand::RngExt;
use rand::seq::IndexedRandom;

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
// 这里的 pub(crate) 是为了测试时修改状态用的

pub struct CharacterPool<'a> {
    wanted_item: String,
    up_four_star: Vec<String>,
    pub(crate) guaranteed: bool,
    pub(crate) five_wish_times: u8,
    pub(crate) four_wish_times: u8,
    pub(crate) off_banner_times: u8,
    permanent_data: &'a PermanentData,
}

impl<'a> CharacterPool<'a> {
    pub fn new(
        wanted_item: String,
        up_four_star: Vec<String>,
        permanent_data: &'a PermanentData,
    ) -> Self {
        Self {
            wanted_item,
            up_four_star,
            guaranteed: false,
            five_wish_times: 0,
            four_wish_times: 0,
            off_banner_times: 0,
            permanent_data,
        }
    }

    fn add_one_pity(&mut self) {
        self.five_wish_times += 1;
        self.four_wish_times += 1;
    }

    fn get_prob(&mut self) -> f64 {
        PROB_90[self.five_wish_times as usize]
    }

    fn get_five_result(
        &self,
        item: String,
        status: CharacterStatus,
    ) -> WishResult<CharacterStatus> {
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

    fn wish_get_five(&self) -> WishResult<CharacterStatus> {
        let mut rng = rand::rng();
        if self.guaranteed {
            // 大保底
            self.get_five_result(self.wanted_item.clone(), CharacterStatus::Guaranteed)
        } else if self.off_banner_times >= 3 {
            // 连歪三次触发捕获明光
            self.get_five_result(
                self.wanted_item.clone(),
                CharacterStatus::GuaranteedCapturingRadiance,
            )
        } else if rng.random::<f64>() < 0.5f64 {
            // 小保底没歪
            self.get_five_result(self.wanted_item.clone(), CharacterStatus::Featured)
        } else if rng.random::<f64>() < 0.1f64 {
            // 小保底歪了但是触发捕获明光
            self.get_five_result(self.wanted_item.clone(), CharacterStatus::CapturingRadiance)
        } else {
            // 小保底歪了
            // 由于 PermanentData 的字段不可能为空，所以这里使用 .unwrap() 方法
            let character = self
                .permanent_data
                .five_character
                .choose(&mut rng)
                .cloned()
                .unwrap();
            self.get_five_result(character, CharacterStatus::OffBanner)
        }
    }

    fn get_four_result(
        &self,
        item: String,
        status: FourStarStatus,
        item_type: ItemType,
    ) -> WishResult<CharacterStatus> {
        WishResult {
            item_name: item,
            item_type: item_type,
            pool_type: PoolType::Character,
            rarity: Rarity::Four,
            wish_times: self.five_wish_times,
            five_status: None,
            four_status: Some(status),
        }
    }

    fn wish_get_four(&self, guaranteed: bool) -> WishResult<CharacterStatus> {
        let mut rng = rand::rng();

        if rng.random::<f64>() < 0.5f64 {
            // up四星
            let status = {
                if guaranteed {
                    FourStarStatus::GuaranteedRateUp
                } else {
                    FourStarStatus::RateUp
                }
            };
            let character = self
                .up_four_star
                .choose(&mut rng)
                .cloned()
                .unwrap_or_else(|| {
                    eprintln!("warning: 卡池up四星为空，将默认使用芭芭拉");
                    String::from("芭芭拉")
                });

            self.get_four_result(character, status, ItemType::Character)
        } else {
            // 常驻四星
            let character_len = self.permanent_data.four_character.len();
            let weapon_len = self.permanent_data.four_weapon.len();
            let total = character_len + weapon_len;
            let index = rng.random_range(0..total);
            let (result, item_type) = {
                if index < character_len {
                    (
                        self.permanent_data.four_character[index].clone(),
                        ItemType::Character,
                    )
                } else {
                    (
                        self.permanent_data.four_weapon[index - character_len].clone(),
                        ItemType::Weapon,
                    )
                }
            };
            let status = {
                if guaranteed {
                    FourStarStatus::GuaranteedCommon
                } else {
                    FourStarStatus::Common
                }
            };

            self.get_four_result(result, status, item_type)
        }
    }

    fn wish_get_three(&self) -> WishResult<CharacterStatus> {
        let mut rng = rand::rng();

        // 由于 PermanentData 的字段不可能为空，所以这里使用 .unwrap() 方法
        let item = self.permanent_data.three.choose(&mut rng).cloned().unwrap();

        WishResult {
            item_name: item,
            item_type: ItemType::Weapon,
            pool_type: PoolType::Character,
            rarity: Rarity::Three,
            wish_times: self.five_wish_times,
            five_status: None,
            four_status: None,
        }
    }

    fn after_wish(&mut self, wish_result: &WishResult<CharacterStatus>) {
        match wish_result.rarity {
            Rarity::Three => (),
            Rarity::Four => {
                self.four_wish_times = 0;
            }
            Rarity::Five => {
                self.five_wish_times = 0;
                self.four_wish_times = 0;

                match wish_result.five_status {
                    Some(CharacterStatus::Featured)
                    | Some(CharacterStatus::CapturingRadiance)
                    | Some(CharacterStatus::GuaranteedCapturingRadiance) => {
                        self.off_banner_times = 0;
                    }
                    Some(CharacterStatus::OffBanner) => {
                        self.guaranteed = true;
                        self.off_banner_times += 1;
                    }
                    Some(CharacterStatus::Guaranteed) => {
                        self.guaranteed = false;
                    }
                    // 在五星的情况下不可能出现None值
                    None => (),
                };
            }
        };
    }
}

impl<'a> Pool<CharacterStatus> for CharacterPool<'a> {
    fn wish(&mut self) -> WishResult<CharacterStatus> {
        self.add_one_pity();

        let mut rng = rand::rng();

        let result: WishResult<CharacterStatus> = {
            // 出金部分
            if rng.random::<f64>() < self.get_prob() {
                self.wish_get_five()
            }
            // 出紫部分
            // 保底四星
            else if self.four_wish_times == 10u8 {
                self.wish_get_four(true)
            }
            // 非保底四星
            else if rng.random::<f64>() < 0.051f64 {
                self.wish_get_four(false)
            }
            // 三星部分
            else {
                self.wish_get_three()
            }
        };

        // 更新状态
        self.after_wish(&result);

        result
    }
}
