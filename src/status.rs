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
    Chronicled,
    #[default]
    Permanent,
}

#[derive(Default)]
pub enum ItemType {
    Character,
    #[default]
    Weapon,
}

pub enum CharacterStatus {
    Featured,
    OffBanner,
    Guaranteed,
    CapturingRadiance,
    GuaranteedCapturingRadiance,
}

pub enum WeaponStatus {
    Featured,
    OffBannerRateUp,
    OffBannerPermanent,
    Guaranteed,
}

pub enum FourStarStatus {
    Common,
    RateUp,
    GuaranteedCommon,
    GuaranteedRateUp,
}

pub enum ChronicledStatus {
    Featured,
    OffBanner,
    Guaranteed,
}

#[derive(Default)]
pub struct WishResult<Status> {
    pub item_name: String,
    pub item_type: ItemType,
    pub pool_type: PoolType,
    pub rarity: Rarity,
    pub wish_times: u8,
    pub five_status: Option<Status>,
    pub four_status: Option<FourStarStatus>,
}
