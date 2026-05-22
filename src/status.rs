#[derive(Default)]
pub enum Rarity {
    #[default]
    Three,
    Four,
    Five,
}

#[derive(Default)]
pub enum PoolType {
    Character,
    Weapon,
    // 这里还该有个混池的
    #[default]
    Permanent,
}

#[derive(Default)]
pub enum ItemType {
    #[default]
    Character,
    Weapon,
}

#[derive(Default)]
pub enum WeaponStatus {
    Expected,
    UnexpectedUp,
    #[default]
    UnexpectedPermanent,
}

#[derive(Default)]
pub struct WishResult {
    pub item_name: String,
    pub item_type: ItemType,
    pub pool: PoolType,
    pub rarity: Rarity,
    pub wish_times: u8,

    // 下面的都是不同卡池可选的参数
    pub guaranteed: bool,
    pub fate_point: u8,
    pub pool_identifier: String, // 这行是混池的
}
