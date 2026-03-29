//! Auto-generated response models from the LOLZTEAM OpenAPI specification.
//!
//! DO NOT EDIT — regenerate with `cargo run -p lolzteam-codegen`.

#![allow(unused, clippy::all)]

use serde::{Deserialize, Deserializer, Serialize};

// ── Enums ──

/// Auto-generated enum for `RoomId`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoomId {
    #[serde(rename = "1")]
    V1,
    #[serde(rename = "2")]
    V2,
    #[serde(rename = "3")]
    V3,
    #[serde(rename = "4")]
    V4,
    #[serde(rename = "13")]
    V13,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(i64),
}

impl std::fmt::Display for RoomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoomId::V1 => write!(f, "1"),
            RoomId::V2 => write!(f, "2"),
            RoomId::V3 => write!(f, "3"),
            RoomId::V4 => write!(f, "4"),
            RoomId::V13 => write!(f, "13"),
            RoomId::Unknown(n) => write!(f, "{}", n),
        }
    }
}

impl Default for RoomId {
    fn default() -> Self {
        RoomId::V1
    }
}

/// Auto-generated enum for `CategoryId`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CategoryId {
    #[serde(rename = "1")]
    V1,
    #[serde(rename = "3")]
    V3,
    #[serde(rename = "4")]
    V4,
    #[serde(rename = "5")]
    V5,
    #[serde(rename = "6")]
    V6,
    #[serde(rename = "7")]
    V7,
    #[serde(rename = "8")]
    V8,
    #[serde(rename = "9")]
    V9,
    #[serde(rename = "10")]
    V10,
    #[serde(rename = "11")]
    V11,
    #[serde(rename = "12")]
    V12,
    #[serde(rename = "13")]
    V13,
    #[serde(rename = "14")]
    V14,
    #[serde(rename = "15")]
    V15,
    #[serde(rename = "16")]
    V16,
    #[serde(rename = "17")]
    V17,
    #[serde(rename = "18")]
    V18,
    #[serde(rename = "19")]
    V19,
    #[serde(rename = "20")]
    V20,
    #[serde(rename = "22")]
    V22,
    #[serde(rename = "24")]
    V24,
    #[serde(rename = "28")]
    V28,
    #[serde(rename = "30")]
    V30,
    #[serde(rename = "31")]
    V31,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(i64),
}

impl std::fmt::Display for CategoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoryId::V1 => write!(f, "1"),
            CategoryId::V3 => write!(f, "3"),
            CategoryId::V4 => write!(f, "4"),
            CategoryId::V5 => write!(f, "5"),
            CategoryId::V6 => write!(f, "6"),
            CategoryId::V7 => write!(f, "7"),
            CategoryId::V8 => write!(f, "8"),
            CategoryId::V9 => write!(f, "9"),
            CategoryId::V10 => write!(f, "10"),
            CategoryId::V11 => write!(f, "11"),
            CategoryId::V12 => write!(f, "12"),
            CategoryId::V13 => write!(f, "13"),
            CategoryId::V14 => write!(f, "14"),
            CategoryId::V15 => write!(f, "15"),
            CategoryId::V16 => write!(f, "16"),
            CategoryId::V17 => write!(f, "17"),
            CategoryId::V18 => write!(f, "18"),
            CategoryId::V19 => write!(f, "19"),
            CategoryId::V20 => write!(f, "20"),
            CategoryId::V22 => write!(f, "22"),
            CategoryId::V24 => write!(f, "24"),
            CategoryId::V28 => write!(f, "28"),
            CategoryId::V30 => write!(f, "30"),
            CategoryId::V31 => write!(f, "31"),
            CategoryId::Unknown(n) => write!(f, "{}", n),
        }
    }
}

impl Default for CategoryId {
    fn default() -> Self {
        CategoryId::V1
    }
}

/// Auto-generated enum for `Currency`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "rub")]
    Rub,
    #[serde(rename = "uah")]
    Uah,
    #[serde(rename = "kzt")]
    Kzt,
    #[serde(rename = "byn")]
    Byn,
    #[serde(rename = "usd")]
    Usd,
    #[serde(rename = "eur")]
    Eur,
    #[serde(rename = "gbp")]
    Gbp,
    #[serde(rename = "cny")]
    Cny,
    #[serde(rename = "try")]
    Try,
    #[serde(rename = "jpy")]
    Jpy,
    #[serde(rename = "brl")]
    Brl,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::Rub => write!(f, "rub"),
            Currency::Uah => write!(f, "uah"),
            Currency::Kzt => write!(f, "kzt"),
            Currency::Byn => write!(f, "byn"),
            Currency::Usd => write!(f, "usd"),
            Currency::Eur => write!(f, "eur"),
            Currency::Gbp => write!(f, "gbp"),
            Currency::Cny => write!(f, "cny"),
            Currency::Try => write!(f, "try"),
            Currency::Jpy => write!(f, "jpy"),
            Currency::Brl => write!(f, "brl"),
            Currency::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for Currency {
    fn default() -> Self {
        Currency::Rub
    }
}

/// Auto-generated enum for `DatePeriod`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DatePeriod {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for DatePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatePeriod::Day => write!(f, "day"),
            DatePeriod::Month => write!(f, "month"),
            DatePeriod::Year => write!(f, "year"),
            DatePeriod::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for DatePeriod {
    fn default() -> Self {
        DatePeriod::Day
    }
}

/// Auto-generated enum for `YesNoNoMatterScheme`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum YesNoNoMatterScheme {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nomatter")]
    Nomatter,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for YesNoNoMatterScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YesNoNoMatterScheme::Yes => write!(f, "yes"),
            YesNoNoMatterScheme::No => write!(f, "no"),
            YesNoNoMatterScheme::Nomatter => write!(f, "nomatter"),
            YesNoNoMatterScheme::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for YesNoNoMatterScheme {
    fn default() -> Self {
        YesNoNoMatterScheme::Yes
    }
}

/// Auto-generated enum for `AppId`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppId {
    #[serde(rename = "730")]
    V730,
    #[serde(rename = "578080")]
    V578080,
    #[serde(rename = "753")]
    V753,
    #[serde(rename = "570")]
    V570,
    #[serde(rename = "440")]
    V440,
    #[serde(rename = "252490")]
    V252490,
    #[serde(rename = "304930")]
    V304930,
    #[serde(rename = "232090")]
    V232090,
    #[serde(rename = "322330")]
    V322330,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(i64),
}

impl std::fmt::Display for AppId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppId::V730 => write!(f, "730"),
            AppId::V578080 => write!(f, "578080"),
            AppId::V753 => write!(f, "753"),
            AppId::V570 => write!(f, "570"),
            AppId::V440 => write!(f, "440"),
            AppId::V252490 => write!(f, "252490"),
            AppId::V304930 => write!(f, "304930"),
            AppId::V232090 => write!(f, "232090"),
            AppId::V322330 => write!(f, "322330"),
            AppId::Unknown(n) => write!(f, "{}", n),
        }
    }
}

impl Default for AppId {
    fn default() -> Self {
        AppId::V730
    }
}

/// Auto-generated enum for `Autorenewal`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Autorenewal {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nomatter")]
    Nomatter,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for Autorenewal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Autorenewal::Yes => write!(f, "yes"),
            Autorenewal::No => write!(f, "no"),
            Autorenewal::Nomatter => write!(f, "nomatter"),
            Autorenewal::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for Autorenewal {
    fn default() -> Self {
        Autorenewal::Yes
    }
}

/// Auto-generated enum for `CategoryName`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CategoryName {
    #[serde(rename = "steam")]
    Steam,
    #[serde(rename = "fortnite")]
    Fortnite,
    #[serde(rename = "mihoyo")]
    Mihoyo,
    #[serde(rename = "riot")]
    Riot,
    #[serde(rename = "telegram")]
    Telegram,
    #[serde(rename = "supercell")]
    Supercell,
    #[serde(rename = "ea")]
    Ea,
    #[serde(rename = "world-of-tanks")]
    WorldOfTanks,
    #[serde(rename = "wot-blitz")]
    WotBlitz,
    #[serde(rename = "epicgames")]
    Epicgames,
    #[serde(rename = "gifts")]
    Gifts,
    #[serde(rename = "minecraft")]
    Minecraft,
    #[serde(rename = "escape-from-tarkov")]
    EscapeFromTarkov,
    #[serde(rename = "socialclub")]
    Socialclub,
    #[serde(rename = "uplay")]
    Uplay,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "tiktok")]
    Tiktok,
    #[serde(rename = "instagram")]
    Instagram,
    #[serde(rename = "chatgpt")]
    Chatgpt,
    #[serde(rename = "battlenet")]
    Battlenet,
    #[serde(rename = "vpn")]
    Vpn,
    #[serde(rename = "roblox")]
    Roblox,
    #[serde(rename = "warface")]
    Warface,
    #[serde(rename = "hytale")]
    Hytale,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for CategoryName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoryName::Steam => write!(f, "steam"),
            CategoryName::Fortnite => write!(f, "fortnite"),
            CategoryName::Mihoyo => write!(f, "mihoyo"),
            CategoryName::Riot => write!(f, "riot"),
            CategoryName::Telegram => write!(f, "telegram"),
            CategoryName::Supercell => write!(f, "supercell"),
            CategoryName::Ea => write!(f, "ea"),
            CategoryName::WorldOfTanks => write!(f, "world-of-tanks"),
            CategoryName::WotBlitz => write!(f, "wot-blitz"),
            CategoryName::Epicgames => write!(f, "epicgames"),
            CategoryName::Gifts => write!(f, "gifts"),
            CategoryName::Minecraft => write!(f, "minecraft"),
            CategoryName::EscapeFromTarkov => write!(f, "escape-from-tarkov"),
            CategoryName::Socialclub => write!(f, "socialclub"),
            CategoryName::Uplay => write!(f, "uplay"),
            CategoryName::Discord => write!(f, "discord"),
            CategoryName::Tiktok => write!(f, "tiktok"),
            CategoryName::Instagram => write!(f, "instagram"),
            CategoryName::Chatgpt => write!(f, "chatgpt"),
            CategoryName::Battlenet => write!(f, "battlenet"),
            CategoryName::Vpn => write!(f, "vpn"),
            CategoryName::Roblox => write!(f, "roblox"),
            CategoryName::Warface => write!(f, "warface"),
            CategoryName::Hytale => write!(f, "hytale"),
            CategoryName::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for CategoryName {
    fn default() -> Self {
        CategoryName::Steam
    }
}

/// Auto-generated enum for `ChangeEmail`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeEmail {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nomatter")]
    Nomatter,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for ChangeEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeEmail::Yes => write!(f, "yes"),
            ChangeEmail::No => write!(f, "no"),
            ChangeEmail::Nomatter => write!(f, "nomatter"),
            ChangeEmail::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for ChangeEmail {
    fn default() -> Self {
        ChangeEmail::Yes
    }
}

/// Auto-generated enum for `Cookies`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cookies {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nomatter")]
    Nomatter,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for Cookies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cookies::Yes => write!(f, "yes"),
            Cookies::No => write!(f, "no"),
            Cookies::Nomatter => write!(f, "nomatter"),
            Cookies::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for Cookies {
    fn default() -> Self {
        Cookies::Yes
    }
}

/// Auto-generated enum for `Email`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Email {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nomatter")]
    Nomatter,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Email::Yes => write!(f, "yes"),
            Email::No => write!(f, "no"),
            Email::Nomatter => write!(f, "nomatter"),
            Email::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for Email {
    fn default() -> Self {
        Email::Yes
    }
}

/// Auto-generated enum for `LastTransDatePeriod`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LastTransDatePeriod {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for LastTransDatePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LastTransDatePeriod::Day => write!(f, "day"),
            LastTransDatePeriod::Month => write!(f, "month"),
            LastTransDatePeriod::Year => write!(f, "year"),
            LastTransDatePeriod::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for LastTransDatePeriod {
    fn default() -> Self {
        LastTransDatePeriod::Day
    }
}

/// Auto-generated enum for `NotEmailProvider`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotEmailProvider {
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "rambler")]
    Rambler,
    #[serde(rename = "outlook")]
    Outlook,
    #[serde(rename = "firstmail")]
    Firstmail,
    #[serde(rename = "notletters")]
    Notletters,
    #[serde(rename = "mail_ru")]
    MailRu,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for NotEmailProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotEmailProvider::Other => write!(f, "other"),
            NotEmailProvider::Rambler => write!(f, "rambler"),
            NotEmailProvider::Outlook => write!(f, "outlook"),
            NotEmailProvider::Firstmail => write!(f, "firstmail"),
            NotEmailProvider::Notletters => write!(f, "notletters"),
            NotEmailProvider::MailRu => write!(f, "mail_ru"),
            NotEmailProvider::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for NotEmailProvider {
    fn default() -> Self {
        NotEmailProvider::Other
    }
}

/// Auto-generated enum for `OrderBy`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderBy {
    #[serde(rename = "price_to_up")]
    PriceToUp,
    #[serde(rename = "price_to_down")]
    PriceToDown,
    #[serde(rename = "pdate_to_down")]
    PdateToDown,
    #[serde(rename = "pdate_to_up")]
    PdateToUp,
    #[serde(rename = "pdate_to_down_upload")]
    PdateToDownUpload,
    #[serde(rename = "pdate_to_up_upload")]
    PdateToUpUpload,
    #[serde(rename = "edate_to_up")]
    EdateToUp,
    #[serde(rename = "edate_to_down")]
    EdateToDown,
    #[serde(rename = "ddate_to_up")]
    DdateToUp,
    #[serde(rename = "ddate_to_down")]
    DdateToDown,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderBy::PriceToUp => write!(f, "price_to_up"),
            OrderBy::PriceToDown => write!(f, "price_to_down"),
            OrderBy::PdateToDown => write!(f, "pdate_to_down"),
            OrderBy::PdateToUp => write!(f, "pdate_to_up"),
            OrderBy::PdateToDownUpload => write!(f, "pdate_to_down_upload"),
            OrderBy::PdateToUpUpload => write!(f, "pdate_to_up_upload"),
            OrderBy::EdateToUp => write!(f, "edate_to_up"),
            OrderBy::EdateToDown => write!(f, "edate_to_down"),
            OrderBy::DdateToUp => write!(f, "ddate_to_up"),
            OrderBy::DdateToDown => write!(f, "ddate_to_down"),
            OrderBy::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for OrderBy {
    fn default() -> Self {
        OrderBy::PriceToUp
    }
}

/// Auto-generated enum for `Premium`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Premium {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nomatter")]
    Nomatter,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for Premium {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Premium::Yes => write!(f, "yes"),
            Premium::No => write!(f, "no"),
            Premium::Nomatter => write!(f, "nomatter"),
            Premium::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for Premium {
    fn default() -> Self {
        Premium::Yes
    }
}

/// Auto-generated enum for `PremiumExpirationPeriod`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PremiumExpirationPeriod {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for PremiumExpirationPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PremiumExpirationPeriod::Day => write!(f, "day"),
            PremiumExpirationPeriod::Month => write!(f, "month"),
            PremiumExpirationPeriod::Year => write!(f, "year"),
            PremiumExpirationPeriod::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for PremiumExpirationPeriod {
    fn default() -> Self {
        PremiumExpirationPeriod::Day
    }
}

/// Auto-generated enum for `RegPeriod`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegPeriod {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for RegPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegPeriod::Day => write!(f, "day"),
            RegPeriod::Month => write!(f, "month"),
            RegPeriod::Year => write!(f, "year"),
            RegPeriod::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for RegPeriod {
    fn default() -> Self {
        RegPeriod::Day
    }
}

/// Auto-generated enum for `Show`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Show {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "awaiting")]
    Awaiting,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "discount_request")]
    DiscountRequest,
    #[serde(rename = "stickied")]
    Stickied,
    #[serde(rename = "pre_active")]
    PreActive,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for Show {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Show::Active => write!(f, "active"),
            Show::Paid => write!(f, "paid"),
            Show::Deleted => write!(f, "deleted"),
            Show::Awaiting => write!(f, "awaiting"),
            Show::Closed => write!(f, "closed"),
            Show::DiscountRequest => write!(f, "discount_request"),
            Show::Stickied => write!(f, "stickied"),
            Show::PreActive => write!(f, "pre_active"),
            Show::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for Show {
    fn default() -> Self {
        Show::Active
    }
}

/// Auto-generated enum for `SubscriptionPeriod`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubscriptionPeriod {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for SubscriptionPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionPeriod::Day => write!(f, "day"),
            SubscriptionPeriod::Month => write!(f, "month"),
            SubscriptionPeriod::Year => write!(f, "year"),
            SubscriptionPeriod::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for SubscriptionPeriod {
    fn default() -> Self {
        SubscriptionPeriod::Day
    }
}

/// Auto-generated enum for `Tel`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tel {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nomatter")]
    Nomatter,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for Tel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tel::Yes => write!(f, "yes"),
            Tel::No => write!(f, "no"),
            Tel::Nomatter => write!(f, "nomatter"),
            Tel::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for Tel {
    fn default() -> Self {
        Tel::Yes
    }
}

/// Auto-generated enum for `TempEmail`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TempEmail {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nomatter")]
    Nomatter,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for TempEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TempEmail::Yes => write!(f, "yes"),
            TempEmail::No => write!(f, "no"),
            TempEmail::Nomatter => write!(f, "nomatter"),
            TempEmail::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for TempEmail {
    fn default() -> Self {
        TempEmail::Yes
    }
}

/// Auto-generated enum for `Verified`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Verified {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nomatter")]
    Nomatter,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for Verified {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verified::Yes => write!(f, "yes"),
            Verified::No => write!(f, "no"),
            Verified::Nomatter => write!(f, "nomatter"),
            Verified::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for Verified {
    fn default() -> Self {
        Verified::Yes
    }
}

/// Auto-generated enum for `WargamingClan`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WargamingClan {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nomatter")]
    Nomatter,
    /// Unknown or new value not yet in the schema.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for WargamingClan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WargamingClan::Yes => write!(f, "yes"),
            WargamingClan::No => write!(f, "no"),
            WargamingClan::Nomatter => write!(f, "nomatter"),
            WargamingClan::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl Default for WargamingClan {
    fn default() -> Self {
        WargamingClan::Yes
    }
}

/// Deserialize a field that may be `null` or have a mismatched type.
/// Falls back to `T::default()` on any type mismatch (e.g. `false` for i64, `null` for String).
fn null_default<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + serde::de::DeserializeOwned,
{
    let v = serde_json::Value::deserialize(deserializer)?;
    Ok(serde_json::from_value(v).unwrap_or_default())
}

/// Deserialize a Vec field that may be null, an array, or an object (takes values).
/// The LOLZTEAM API sometimes returns `{}` or `{"key": val}` instead of `[]`.
fn null_or_vec<'de, D, T>(deserializer: D) -> std::result::Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned + Default,
{
    use serde_json::Value;
    let v = Value::deserialize(deserializer)?;
    match v {
        Value::Array(arr) => {
            let mut out = Vec::with_capacity(arr.len());
            for item in arr {
                out.push(serde_json::from_value(item).unwrap_or_default());
            }
            Ok(out)
        }
        Value::Object(map) => {
            let mut out = Vec::with_capacity(map.len());
            for (_key, item) in map {
                out.push(serde_json::from_value(item).unwrap_or_default());
            }
            Ok(out)
        }
        Value::Null => Ok(Vec::new()),
        _ => Ok(Vec::new()),
    }
}

/// Category model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Category {
    #[serde(deserialize_with = "null_default", default)]
    pub category_description: String,
    #[serde(deserialize_with = "null_default", default)]
    pub category_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub category_title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
}

/// ChatboxMessage model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxMessage {
    #[serde(deserialize_with = "null_default", default)]
    pub can_report: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub is_deleted: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(rename = "messageJson")]
    #[serde(deserialize_with = "null_default", default)]
    pub message_json: String,
    #[serde(rename = "messageRaw")]
    #[serde(deserialize_with = "null_default", default)]
    pub message_raw: String,
    #[serde(deserialize_with = "null_default", default)]
    pub message_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub room: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub user: serde_json::Value,
}

/// ConversationMessage model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationMessage {
    #[serde(deserialize_with = "null_default", default)]
    pub conversation_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_username: String,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_username_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub message_body: String,
    #[serde(deserialize_with = "null_default", default)]
    pub message_body_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub message_body_plain_text: String,
    #[serde(deserialize_with = "null_default", default)]
    pub message_create_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub message_edit_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub message_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub message_is_system: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub message_is_unread: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub message_need_translate: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub user_is_ignored: bool,
}

/// Conversation model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Conversation {
    #[serde(deserialize_with = "null_default", default)]
    pub alerts: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub conversation_create_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub conversation_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub conversation_is_deleted: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub conversation_is_new: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub conversation_is_open: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub conversation_last_read_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub conversation_message_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub conversation_online_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub conversation_title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub conversation_update_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_is_ignored: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_username: String,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_username_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub is_group: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub is_starred: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub is_unread: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub recipient: serde_json::Value,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub recipients: Vec<serde_json::Value>,
}

/// Forum model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Forum {
    #[serde(deserialize_with = "null_default", default)]
    pub forum_description: String,
    #[serde(deserialize_with = "null_default", default)]
    pub forum_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub forum_is_followed: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub forum_post_count: i64,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub forum_prefixes: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub forum_thread_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub forum_title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_default_prefix_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_prefix_is_required: bool,
}

/// Link model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Link {
    #[serde(deserialize_with = "null_default", default)]
    pub link_description: String,
    #[serde(deserialize_with = "null_default", default)]
    pub link_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub link_title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
}

/// Notification model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Notification {
    #[serde(deserialize_with = "null_default", default)]
    pub content_action: String,
    #[serde(deserialize_with = "null_default", default)]
    pub content_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub content_type: String,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_username: String,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_username_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub notification_create_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub notification_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub notification_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub notification_is_unread: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub notification_type: String,
}

/// PostComment model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostComment {
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub post_comment_body: String,
    #[serde(deserialize_with = "null_default", default)]
    pub post_comment_body_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub post_comment_body_plain_text: String,
    #[serde(deserialize_with = "null_default", default)]
    pub post_comment_create_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub post_comment_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub post_comment_is_deleted: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub post_comment_is_published: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub post_comment_like_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub post_comment_update_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub post_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub poster_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub poster_username: String,
    #[serde(deserialize_with = "null_default", default)]
    pub poster_username_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_is_ignored: bool,
}

/// Post model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Post {
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub post_body: String,
    #[serde(deserialize_with = "null_default", default)]
    pub post_body_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub post_body_plain_text: String,
    #[serde(deserialize_with = "null_default", default)]
    pub post_create_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub post_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub post_is_deleted: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub post_is_first_post: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub post_is_published: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub post_like_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub post_update_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub poster_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub poster_username: String,
    #[serde(deserialize_with = "null_default", default)]
    pub poster_username_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub signature: String,
    #[serde(deserialize_with = "null_default", default)]
    pub signature_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub signature_plain_text: String,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_is_deleted: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub user_is_ignored: bool,
}

/// ProfilePostComment model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostComment {
    #[serde(deserialize_with = "null_default", default)]
    pub comment_body: String,
    #[serde(deserialize_with = "null_default", default)]
    pub comment_body_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub comment_body_plain_text: String,
    #[serde(deserialize_with = "null_default", default)]
    pub comment_create_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub comment_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub comment_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub comment_username: String,
    #[serde(deserialize_with = "null_default", default)]
    pub comment_username_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub profile_post_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub timeline_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_is_ignored: bool,
}

/// ProfilePost model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePost {
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub post_body: String,
    #[serde(deserialize_with = "null_default", default)]
    pub post_body_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub post_body_plain_text: String,
    #[serde(deserialize_with = "null_default", default)]
    pub post_comment_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub post_comments_is_disabled: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub post_create_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub post_is_deleted: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub post_is_liked: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub post_is_published: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub post_is_sticked: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub post_like_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub poster_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub poster_username: String,
    #[serde(deserialize_with = "null_default", default)]
    pub poster_username_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub profile_post_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub timeline_user: User,
    #[serde(deserialize_with = "null_default", default)]
    pub timeline_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub timeline_username: String,
    #[serde(deserialize_with = "null_default", default)]
    pub user_is_ignored: bool,
}

/// SystemInfo model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SystemInfo {
    #[serde(deserialize_with = "null_default", default)]
    pub time: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub visitor_id: i64,
}

/// Thread model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Thread {
    #[serde(deserialize_with = "null_default", default)]
    pub contest: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_username: String,
    #[serde(deserialize_with = "null_default", default)]
    pub creator_username_html: String,
    #[serde(deserialize_with = "null_default", default)]
    pub first_post: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub forum_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub last_post: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub node_title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub restrictions: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_create_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_is_closed: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_is_deleted: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_is_followed: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_is_published: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_is_starred: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_is_sticky: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_post_count: i64,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub thread_prefixes: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_tags: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_update_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub thread_view_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_is_ignored: bool,
}

/// User model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct User {
    #[serde(deserialize_with = "null_default", default)]
    pub balance: String,
    #[serde(deserialize_with = "null_default", default)]
    pub banner: String,
    #[serde(deserialize_with = "null_default", default)]
    pub birthday: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub contest_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub conv_welcome_message: String,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub curator_titles: Vec<String>,
    #[serde(deserialize_with = "null_default", default)]
    pub currency: String,
    #[serde(deserialize_with = "null_default", default)]
    pub custom_title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub display_banner_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub display_icon_group_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub edit_permissions: serde_json::Value,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub fields: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub hold: String,
    #[serde(deserialize_with = "null_default", default)]
    pub is_banned: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub secret_answer_first_letter: String,
    #[serde(deserialize_with = "null_default", default)]
    pub secret_answer_rendered: String,
    #[serde(deserialize_with = "null_default", default)]
    pub self_permissions: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub short_link: String,
    #[serde(deserialize_with = "null_default", default)]
    pub trophy_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_deposit: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_email: String,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub user_external_authentications: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub user_followers: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub user_following: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub user_group_id: i64,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub user_groups: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_is_followed: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub user_is_ignored: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub user_is_valid: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub user_is_verified: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub user_is_visitor: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub user_last_seen_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_like2_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_like_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_message_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_register_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_timezone_offset: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub user_unread_conversation_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_unread_notification_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub username: String,
    #[serde(deserialize_with = "null_default", default)]
    pub username_html: String,
}

/// Balance model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Balance {
    #[serde(deserialize_with = "null_default", default)]
    pub balance: String,
    #[serde(deserialize_with = "null_default", default)]
    pub balance_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub custom_title: serde_json::Value,
    #[serde(rename = "fullTitle")]
    #[serde(deserialize_with = "null_default", default)]
    pub full_title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub merchant_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub r#type: String,
    #[serde(deserialize_with = "null_default", default)]
    pub user_id: i64,
}

/// ConfirmationCode model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConfirmationCode {
    #[serde(rename = "codeData")]
    #[serde(deserialize_with = "null_default", default)]
    pub code_data: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub item: Item,
}

/// Discount model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Discount {
    #[serde(deserialize_with = "null_default", default)]
    pub category_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub discount_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub discount_percent: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub discount_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub max_price: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub min_price: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_id: i64,
}

/// Extra model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Extra {
    /// Ark. Optional. Used only if you want to upload Steam account.
    pub ark: Option<bool>,
    /// Ark Ascended. Optional. Used only if you want to upload Steam account.
    pub ark_ascended: Option<bool>,
    /// Check channels. Optional. Used only if you want to upload Telegram account.
    #[serde(rename = "checkChannels")]
    pub check_channels: Option<bool>,
    /// Check ban on Hypixel. Optional. Used only if you want to upload Minecraft account.
    #[serde(rename = "checkHypixelBan")]
    pub check_hypixel_ban: Option<bool>,
    /// Check spam. Optional. Used only if you want to upload Telegram account.
    #[serde(rename = "checkSpam")]
    pub check_spam: Option<bool>,
    /// If set, the item will be closed **item_state = closed**.
    pub close_item: Option<bool>,
    /// Code from email (in case of problems). Optional if you want to upload Supercell account.
    #[serde(rename = "confirmationCode")]
    pub confirmation_code: Option<String>,
    /// Cookie login. Optional. Used only if you want to upload TikTok account.
    pub cookie_login: Option<bool>,
    /// Cookies. Required if you want to upload Fortnite, Epic Games, Social Club, Instagram or TikTok account.
    pub cookies: Option<String>,
    /// Dota 2 MMR. Optional. Used only if you want to upload Steam account.
    pub dota2_mmr: Option<i64>,
    /// EA Games. Optional. Used only if you want to upload Steam account.
    pub ea_games: Option<bool>,
    /// Genshin Impact Primogems count. Optional. Used only if you want to upload miHoYo account.
    pub genshin_currency: Option<i64>,
    /// Honkai Star Rail Stellar Jade count. Optional. Used only if you want to upload miHoYo account.
    pub honkai_currency: Option<i64>,
    /// Login without cookies. Optional if you want to upload Instagram account.
    pub login_without_cookies: Option<bool>,
    /// Steam mafile. Optional. Used only if you want to upload Steam account.
    pub mfa_file: Option<String>,
    /// Telegram 2FA Password. Optional. Used only if you want to upload Telegram account.
    pub password: Option<String>,
    /// Proxy line format ip:port:user:pass (prioritize over proxy_id parameter).
    pub proxy: Option<String>,
    /// Region. Required if you want to upload WoT account. Optional if you want to upload miHoYo or Riot account.
    pub region: Option<String>,
    /// Service. Required if you want to upload VPN, Cinema account or gift.
    pub service: Option<String>,
    /// Supercell system. Required if you want to upload Supercell account.
    pub system: Option<String>,
    /// Telegram client. Optional. Used only if you want to upload Telegram account.
    #[serde(rename = "telegramClient")]
    pub telegram_client: Option<String>,
    /// Contents of session.json file. Optional. Used only if you want to upload Telegram account.
    #[serde(rename = "telegramJson")]
    pub telegram_json: Option<String>,
    /// The quarry. Optional. Used only if you want to upload Steam account.
    pub the_quarry: Option<bool>,
    /// Uplay Games. Optional. Used only if you want to upload Steam account.
    pub uplay_games: Option<bool>,
    /// Warframe. Optional. Used only if you want to upload Steam account.
    pub warframe: Option<bool>,
    /// Zenless Zone Zero Polychrome count. Optional. Used only if you want to upload miHoYo account.
    pub zenless_currency: Option<i64>,
}

/// Invoice model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Invoice {
    #[serde(deserialize_with = "null_default", default)]
    pub additional_data: String,
    #[serde(deserialize_with = "null_default", default)]
    pub amount: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub comment: String,
    #[serde(deserialize_with = "null_default", default)]
    pub expires_at: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub invoice_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub invoice_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub is_test: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub merchant_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub paid_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub payer_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub payment_id: String,
    #[serde(deserialize_with = "null_default", default)]
    pub resend_attempts: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub url: String,
    #[serde(deserialize_with = "null_default", default)]
    pub url_callback: String,
    #[serde(deserialize_with = "null_default", default)]
    pub url_success: String,
    #[serde(deserialize_with = "null_default", default)]
    pub user_id: i64,
}

/// ItemFromList model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ItemFromList {
    pub allow_ask_discount: Option<i64>,
    #[serde(rename = "bumpSettings")]
    pub bump_settings: Option<serde_json::Value>,
    #[serde(rename = "canBumpItem")]
    pub can_bump_item: Option<bool>,
    #[serde(rename = "canBuyItem")]
    pub can_buy_item: Option<bool>,
    #[serde(rename = "canCloseItem")]
    pub can_close_item: Option<bool>,
    #[serde(rename = "canDeleteItem")]
    pub can_delete_item: Option<bool>,
    #[serde(rename = "canEditItem")]
    pub can_edit_item: Option<bool>,
    #[serde(rename = "canOpenItem")]
    pub can_open_item: Option<bool>,
    #[serde(rename = "canResellItemAfterPurchase")]
    pub can_resell_item_after_purchase: Option<bool>,
    #[serde(rename = "canStickItem")]
    pub can_stick_item: Option<bool>,
    #[serde(rename = "canUnstickItem")]
    pub can_unstick_item: Option<bool>,
    #[serde(rename = "canUpdateItemStats")]
    pub can_update_item_stats: Option<bool>,
    #[serde(rename = "canValidateAccount")]
    pub can_validate_account: Option<bool>,
    #[serde(rename = "canViewAccountLink")]
    pub can_view_account_link: Option<bool>,
    #[serde(rename = "canViewEmailLoginData")]
    pub can_view_email_login_data: Option<bool>,
    #[serde(rename = "canViewLoginData")]
    pub can_view_login_data: Option<bool>,
    pub category_id: Option<i64>,
    pub description: Option<String>,
    pub description_en: Option<String>,
    pub description_html: Option<String>,
    pub description_html_en: Option<String>,
    pub extended_guarantee: Option<i64>,
    pub guarantee: Option<bool>,
    #[serde(rename = "isIgnored")]
    pub is_ignored: Option<i64>,
    pub is_sticky: Option<i64>,
    #[serde(rename = "itemOriginPhrase")]
    pub item_origin_phrase: Option<String>,
    pub item_domain: Option<String>,
    pub item_id: Option<i64>,
    pub item_origin: Option<String>,
    pub item_state: Option<String>,
    pub note_text: Option<String>,
    pub nsb: Option<i64>,
    pub price: Option<i64>,
    pub price_currency: Option<String>,
    pub published_date: Option<i64>,
    pub refreshed_date: Option<i64>,
    pub resale_item_origin: Option<String>,
    pub rub_price: Option<i64>,
    pub seller: Option<serde_json::Value>,
    #[serde(rename = "showGetEmailCodeButton")]
    pub show_get_email_code_button: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub title: Option<String>,
    pub title_en: Option<String>,
    pub update_stat_date: Option<i64>,
    pub view_count: Option<i64>,
}

/// ItemList model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ItemList {
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "deserialize_items", default)]
    pub items: Vec<ItemFromList>,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<ItemFromList>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
}

/// Item model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Item {
    #[serde(rename = "accountLink")]
    #[serde(deserialize_with = "null_default", default)]
    pub account_link: String,
    #[serde(rename = "accountLinks")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub account_links: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub account_last_activity: i64,
    #[serde(rename = "aiPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub ai_price: i64,
    #[serde(rename = "aiPriceCheckDate")]
    #[serde(deserialize_with = "null_default", default)]
    pub ai_price_check_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub allow_ask_discount: i64,
    #[serde(rename = "autoBuyPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub auto_buy_price: i64,
    #[serde(rename = "autoBuyPriceCheckDate")]
    #[serde(deserialize_with = "null_default", default)]
    pub auto_buy_price_check_date: i64,
    #[serde(rename = "bumpSettings")]
    #[serde(deserialize_with = "null_default", default)]
    pub bump_settings: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub buyer: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub buyer_avatar_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub buyer_display_icon_group_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub buyer_uniq_banner: String,
    #[serde(deserialize_with = "null_default", default)]
    pub buyer_user_group_id: i64,
    #[serde(rename = "canAskDiscount")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_ask_discount: bool,
    #[serde(rename = "canChangeEmailPassword")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_change_email_password: bool,
    #[serde(rename = "canChangePassword")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_change_password: bool,
    #[serde(rename = "canCheckAiPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_check_ai_price: bool,
    #[serde(rename = "canCheckAutoBuyPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_check_auto_buy_price: bool,
    #[serde(rename = "canCheckGuarantee")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_check_guarantee: bool,
    #[serde(rename = "canReportItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_report_item: bool,
    #[serde(rename = "canResellItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_resell_item: bool,
    #[serde(rename = "canResellItemAfterPurchase")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_resell_item_after_purchase: bool,
    #[serde(rename = "canShareItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_share_item: bool,
    #[serde(rename = "canUpdateItemStats")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_update_item_stats: bool,
    #[serde(rename = "canValidateAccount")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_validate_account: bool,
    #[serde(rename = "canViewAccountLink")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_view_account_link: bool,
    #[serde(rename = "canViewAccountLoginAndTempEmail")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_view_account_login_and_temp_email: bool,
    #[serde(rename = "canViewEmailLoginData")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_view_email_login_data: bool,
    #[serde(rename = "canViewItemViews")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_view_item_views: bool,
    #[serde(rename = "canViewLoginData")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_view_login_data: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub cart_price: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub category_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub content_id: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub content_type: serde_json::Value,
    #[serde(rename = "copyFormatData")]
    #[serde(deserialize_with = "null_default", default)]
    pub copy_format_data: serde_json::Value,
    #[serde(rename = "customFields")]
    #[serde(deserialize_with = "null_default", default)]
    pub custom_fields: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub delete_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub delete_reason: String,
    #[serde(deserialize_with = "null_default", default)]
    pub delete_user_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub delete_username: String,
    #[serde(deserialize_with = "null_default", default)]
    pub deposit: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub description: String,
    #[serde(rename = "descriptionEnHtml")]
    #[serde(deserialize_with = "null_default", default)]
    pub description_en_html: String,
    #[serde(rename = "descriptionEnPlain")]
    #[serde(deserialize_with = "null_default", default)]
    pub description_en_plain: String,
    #[serde(rename = "descriptionHtml")]
    #[serde(deserialize_with = "null_default", default)]
    pub description_html: String,
    #[serde(rename = "descriptionPlain")]
    #[serde(deserialize_with = "null_default", default)]
    pub description_plain: String,
    #[serde(deserialize_with = "null_default", default)]
    pub description_en: String,
    #[serde(deserialize_with = "null_default", default)]
    pub edit_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub email_provider: String,
    #[serde(deserialize_with = "null_default", default)]
    pub email_type: String,
    #[serde(deserialize_with = "null_default", default)]
    pub extended_guarantee: i64,
    #[serde(rename = "externalAuth")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub external_auth: Vec<serde_json::Value>,
    #[serde(rename = "extraPrices")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub extra_prices: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub feedback_data: String,
    #[serde(rename = "getEmailCodeDisplayLogin")]
    #[serde(deserialize_with = "null_default", default)]
    pub get_email_code_display_login: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub guarantee: serde_json::Value,
    #[serde(rename = "imagePreviewLinks")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub image_preview_links: Vec<String>,
    #[serde(deserialize_with = "null_default", default)]
    pub in_cart: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub information: String,
    #[serde(deserialize_with = "null_default", default)]
    pub information_en: String,
    #[serde(rename = "isBirthdayToday")]
    #[serde(deserialize_with = "null_default", default)]
    pub is_birthday_today: bool,
    #[serde(rename = "isIgnored")]
    #[serde(deserialize_with = "null_default", default)]
    pub is_ignored: bool,
    #[serde(rename = "isPersonalAccount")]
    #[serde(deserialize_with = "null_default", default)]
    pub is_personal_account: bool,
    #[serde(rename = "isSmallExf")]
    #[serde(deserialize_with = "null_default", default)]
    pub is_small_exf: bool,
    #[serde(rename = "isTrusted")]
    #[serde(deserialize_with = "null_default", default)]
    pub is_trusted: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub is_fave: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub is_sticky: i64,
    #[serde(rename = "itemOriginPhrase")]
    #[serde(deserialize_with = "null_default", default)]
    pub item_origin_phrase: String,
    #[serde(deserialize_with = "null_default", default)]
    pub item_domain: String,
    #[serde(deserialize_with = "null_default", default)]
    pub item_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub item_origin: String,
    #[serde(deserialize_with = "null_default", default)]
    pub item_state: String,
    #[serde(deserialize_with = "null_default", default)]
    pub login: String,
    #[serde(rename = "loginData")]
    #[serde(deserialize_with = "null_default", default)]
    pub login_data: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub market_custom_title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub max_discount_percent: i64,
    #[serde(rename = "needToRequireVideoToViewLoginData")]
    #[serde(deserialize_with = "null_default", default)]
    pub need_to_require_video_to_view_login_data: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub note_text: String,
    #[serde(deserialize_with = "null_default", default)]
    pub nsb: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub pending_deletion_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub price: i64,
    #[serde(rename = "priceWithSellerFee")]
    #[serde(deserialize_with = "null_default", default)]
    pub price_with_seller_fee: f64,
    #[serde(rename = "priceWithSellerFeeLabel")]
    #[serde(deserialize_with = "null_default", default)]
    pub price_with_seller_fee_label: String,
    #[serde(deserialize_with = "null_default", default)]
    pub price_currency: String,
    #[serde(deserialize_with = "null_default", default)]
    pub published_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub refreshed_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub resale_item_origin: String,
    #[serde(deserialize_with = "null_default", default)]
    pub rub_price: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub seller: serde_json::Value,
    #[serde(rename = "showGetEmailCodeButton")]
    #[serde(deserialize_with = "null_default", default)]
    pub show_get_email_code_button: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub tags: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub temp_email: String,
    #[serde(deserialize_with = "null_default", default)]
    pub title: String,
    #[serde(deserialize_with = "null_default", default)]
    pub title_en: String,
    #[serde(rename = "uniqueKeyExists")]
    #[serde(deserialize_with = "null_default", default)]
    pub unique_key_exists: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub update_stat_date: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub user_allow_ask_discount: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub view_count: i64,
    #[serde(rename = "visitorIsAuthor")]
    #[serde(deserialize_with = "null_default", default)]
    pub visitor_is_author: bool,
}

// API sometimes returns items as array, sometimes as object {id: item}
fn deserialize_items<'de, D>(deserializer: D) -> std::result::Result<Vec<ItemFromList>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;
    use serde_json::Value;

    let v = Value::deserialize(deserializer)?;
    match v {
        Value::Array(arr) => {
            let mut out = Vec::with_capacity(arr.len());
            for item in arr {
                out.push(serde_json::from_value(item).unwrap_or_default());
            }
            Ok(out)
        }
        Value::Object(map) => {
            let mut out = Vec::with_capacity(map.len());
            for (_key, item) in map {
                out.push(serde_json::from_value(item).unwrap_or_default());
            }
            Ok(out)
        }
        Value::Null => Ok(Vec::new()),
        _ => Err(de::Error::custom("expected array or object for items")),
    }
}

// ── Response wrappers ──

/// UsersSaResetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersSaResetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub success: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub waiting_time: String,
}

/// BatchExecuteResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchExecuteResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub jobs: serde_json::Value,
}

/// CategoriesListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesListResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub categories: Vec<Category>,
    #[serde(deserialize_with = "null_default", default)]
    pub categories_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// CategoriesGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub category: Category,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ChatboxIndexResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxIndexResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub ban: serde_json::Value,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub commands: Vec<String>,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub ignore: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub permissions: serde_json::Value,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub rooms: Vec<serde_json::Value>,
    #[serde(rename = "roomsOnline")]
    #[serde(deserialize_with = "null_default", default)]
    pub rooms_online: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ChatboxGetIgnoreResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetIgnoreResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub ignored: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ChatboxGetMessagesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetMessagesResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub messages: Vec<ChatboxMessage>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ChatboxPostMessageResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxPostMessageResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: ChatboxMessage,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ChatboxEditMessageResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxEditMessageResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: ChatboxMessage,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ChatboxGetLeaderboardResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetLeaderboardResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub leaderboard: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ChatboxOnlineResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxOnlineResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<serde_json::Value>,
}

/// ChatboxReportReasonsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxReportReasonsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub reasons: Vec<String>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ThreadsClaimResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsClaimResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub thread: Thread,
}

/// ThreadsCreateContestResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsCreateContestResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub thread: Thread,
}

/// ConversationsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsListResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub can_start: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub conversations: Vec<Conversation>,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub folders: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsCreateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub conversation: Conversation,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsUpdateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsUpdateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub conversation: Conversation,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsMessagesGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: Conversation,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsReadAllResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsReadAllResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsSearchResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsSearchResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub conversations: Vec<Conversation>,
    #[serde(deserialize_with = "null_default", default)]
    pub recipients: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsStartResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsStartResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub conversation: Conversation,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub conversation: Conversation,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsAlertsDisableResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsAlertsDisableResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsAlertsEnableResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsAlertsEnableResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsMessagesListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesListResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub messages: Vec<ConversationMessage>,
    #[serde(deserialize_with = "null_default", default)]
    pub messages_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsMessagesCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesCreateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: ConversationMessage,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsMessagesEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesEditResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: Conversation,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsUnstarResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsUnstarResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ConversationsStarResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsStarResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// AssetsCssResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AssetsCssResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub contents: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// FormsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsListResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub forms: Vec<serde_json::Value>,
    #[serde(rename = "formsPerPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub forms_per_page: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalForms")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_forms: i64,
}

/// FormsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsCreateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub content: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ForumsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsListResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub forums: Vec<Forum>,
    #[serde(deserialize_with = "null_default", default)]
    pub forums_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub tabs: Vec<serde_json::Value>,
}

/// ForumsGetFeedOptionsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetFeedOptionsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub default_excluded_forums_ids: Vec<i64>,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub excluded_forums_ids: Vec<i64>,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub forums: Vec<Forum>,
    #[serde(deserialize_with = "null_default", default)]
    pub keywords: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ForumsFollowedResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowedResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub forums: Vec<Forum>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ForumsGroupedResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGroupedResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub data: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub tabs: Vec<serde_json::Value>,
}

/// ForumsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub forum: Forum,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ForumsFollowersResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowersResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<serde_json::Value>,
}

/// LinksListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LinksListResponse {
    #[serde(rename = "link-forums")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub link_forums: Vec<Link>,
    #[serde(rename = "link-forums_total")]
    #[serde(deserialize_with = "null_default", default)]
    pub link_forums_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// LinksGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LinksGetResponse {
    #[serde(rename = "link-forum")]
    #[serde(deserialize_with = "null_default", default)]
    pub link_forum: Link,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// NavigationListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NavigationListResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub elements: Vec<Category>,
    #[serde(deserialize_with = "null_default", default)]
    pub elements_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// NotificationsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NotificationsListResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub notifications: Vec<Notification>,
    #[serde(deserialize_with = "null_default", default)]
    pub notifications_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// NotificationsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NotificationsGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub notification: Notification,
    #[serde(deserialize_with = "null_default", default)]
    pub notification_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// OAuthTokenResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OAuthTokenResponse {
    /// The access token issued by the authorization server
    #[serde(deserialize_with = "null_default", default)]
    pub access_token: String,
    /// The lifetime in seconds of the access token
    #[serde(deserialize_with = "null_default", default)]
    pub expires_in: i64,
    /// The refresh token, which can be used to obtain new access tokens
    pub refresh_token: Option<String>,
    /// The scope of the access token
    pub scope: Option<String>,
    /// The type of the token
    #[serde(deserialize_with = "null_default", default)]
    pub token_type: String,
}

/// PagesListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesListResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub pages: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub pages_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PagesGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub page: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PostsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsListResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub posts: Vec<Post>,
    #[serde(deserialize_with = "null_default", default)]
    pub posts_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub thread: Thread,
}

/// PostsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCreateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub post: Post,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PostsCommentsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsGetResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub comments: Vec<PostComment>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PostsCommentsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsCreateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub comment: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PostsCommentsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsEditResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub comment: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PostsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub post: Post,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PostsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsEditResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub post: Post,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PostsLikesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsLikesResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<serde_json::Value>,
}

/// PostsReportReasonsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsReportReasonsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub reasons: Vec<String>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ProfilePostsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCreateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub profile_post: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ProfilePostsCommentsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsListResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub comments: Vec<ProfilePostComment>,
    #[serde(deserialize_with = "null_default", default)]
    pub comments_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub profile_post: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub timeline_user: User,
}

/// ProfilePostsCommentsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsCreateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub comment: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ProfilePostsCommentsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsEditResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub comment: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ProfilePostsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub profile_post: ProfilePost,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ProfilePostsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsEditResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub profile_post: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ProfilePostsCommentsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub comment: ProfilePostComment,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ProfilePostsLikesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsLikesResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<serde_json::Value>,
}

/// ProfilePostsReportReasonsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsReportReasonsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub reasons: Vec<String>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// SearchAllResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub data: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub data_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<User>,
}

/// SearchPostsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub data: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub data_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// SearchProfilePostsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchProfilePostsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub data: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub data_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// SearchTaggedResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub data: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub data_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub search_tags: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// SearchThreadsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub data: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub data_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// SearchUsersResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchUsersResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<User>,
}

/// SearchResultsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub data: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub data_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub search_tags: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// TagsPopularResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsPopularResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub tags: serde_json::Value,
}

/// TagsFindResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsFindResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub ids: Vec<i64>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub tags: Vec<String>,
}

/// TagsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsListResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub tags: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub tags_total: i64,
}

/// TagsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub tag: serde_json::Value,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub tagged: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub tagged_total: i64,
}

/// ThreadsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsListResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub forum: Forum,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub threads: Vec<Thread>,
    #[serde(deserialize_with = "null_default", default)]
    pub threads_total: i64,
}

/// ThreadsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsCreateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub thread: Thread,
}

/// ThreadsFollowedResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub threads: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub threads_total: i64,
}

/// ThreadsUnreadResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub data: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub threads: Vec<Thread>,
}

/// ThreadsRecentResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub data: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub threads: Vec<Thread>,
}

/// ThreadsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub thread: Thread,
}

/// ThreadsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsEditResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub thread: Thread,
}

/// ThreadsBumpResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsBumpResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ThreadsFollowersResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowersResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<serde_json::Value>,
}

/// ThreadsHideResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsHideResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ThreadsNavigationResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsNavigationResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub elements: Vec<Category>,
    #[serde(deserialize_with = "null_default", default)]
    pub elements_count: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ThreadsPollGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsPollGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub poll: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// UsersListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersListResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<User>,
    #[serde(deserialize_with = "null_default", default)]
    pub users_total: i64,
}

/// UsersFieldsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFieldsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub fields: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// UsersFindResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFindResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<User>,
}

/// UsersIgnoredResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersIgnoredResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<serde_json::Value>,
}

/// UsersSecretAnswerTypesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersSecretAnswerTypesResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub data: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// UsersGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub user: User,
}

/// UsersAvatarUploadResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersAvatarUploadResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// UsersAvatarCropResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersAvatarCropResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// UsersBackgroundUploadResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersBackgroundUploadResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// UsersBackgroundCropResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersBackgroundCropResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// UsersClaimsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersClaimsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub claims: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub stats: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// UsersFollowersResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowersResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub users_total: i64,
}

/// UsersFollowingsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowingsResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub users: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub users_total: i64,
}

/// UsersLikesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersLikesResponse {
    #[serde(rename = "contentType")]
    #[serde(deserialize_with = "null_default", default)]
    pub content_type: String,
    #[serde(deserialize_with = "null_default", default)]
    pub likes: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalLikes")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_likes: i64,
}

/// ProfilePostsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsListResponse {
    #[serde(rename = "canPostOnProfile")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_post_on_profile: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub profile_posts: Vec<ProfilePost>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalProfilePosts")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_profile_posts: i64,
}

/// UsersContentsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub data: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub data_total: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub links: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub user: User,
}

/// UsersTrophiesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersTrophiesResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub trophies: Vec<serde_json::Value>,
}

/// AutoPaymentsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsCreateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub auto_payment_id: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// AutoPaymentsListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutoPaymentsListResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub payments: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PaymentsPayoutServicesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsPayoutServicesResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub systems: Vec<serde_json::Value>,
}

/// PaymentsFeeResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsFeeResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub calculator: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub commission_percentage: i64,
    #[serde(rename = "spentCurrentMonth")]
    #[serde(deserialize_with = "null_default", default)]
    pub spent_current_month: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// BatchResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub jobs: serde_json::Value,
    pub system_info: Option<SystemInfo>,
}

/// CategoryBattleNetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryBattleNetResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// ManagingBulkGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingBulkGetResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub left_item_id: Vec<i64>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// CartDeleteResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CartDeleteResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub success: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// CartAddResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CartAddResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub success: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// CategoryListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryListResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub category: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// CategoryChatGptResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryChatGptResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// ProfileClaimsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileClaimsResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub claims: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub stats: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ManagingCreateClaimResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCreateClaimResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub thread: serde_json::Value,
}

/// PaymentsCurrencyResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsCurrencyResponse {
    #[serde(rename = "currencyList")]
    #[serde(deserialize_with = "null_default", default)]
    pub currency_list: serde_json::Value,
    #[serde(rename = "lastUpdate")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_update: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "visitorCurrency")]
    #[serde(deserialize_with = "null_default", default)]
    pub visitor_currency: String,
}

/// CustomDiscountsGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsGetResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub discounts: Vec<Discount>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub total: i64,
}

/// CustomDiscountsCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsCreateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub discount: Discount,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub total: i64,
}

/// CustomDiscountsEditResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CustomDiscountsEditResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub discounts: Vec<Discount>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub total: i64,
}

/// CategoryDiscordResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryDiscordResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryEaResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEaResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryEpicGamesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEpicGamesResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryEscapeFromTarkovResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryEscapeFromTarkovResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryFortniteResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryFortniteResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryGiftsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryGiftsResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryHytaleResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryHytaleResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryInstagramResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryInstagramResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// PaymentsInvoiceGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub invoice: Invoice,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PaymentsInvoiceCreateResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceCreateResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub invoice: Invoice,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PaymentsInvoiceListResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsInvoiceListResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub count: i64,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub invoices: Vec<Invoice>,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PublishingAddResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PublishingAddResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub item: Item,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PublishingFastSellResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PublishingFastSellResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub item: Item,
    #[serde(rename = "itemLink")]
    #[serde(deserialize_with = "null_default", default)]
    pub item_link: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ManagingGetLetters2Response model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingGetLetters2Response {
    #[serde(deserialize_with = "null_default", default)]
    pub email: String,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub letters: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ProfileGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfileGetResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(deserialize_with = "null_default", default)]
    pub user: User,
}

/// CategoryMihoyoResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMihoyoResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryMinecraftResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryMinecraftResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// ProxyGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProxyGetResponse {
    #[serde(deserialize_with = "null_or_vec", default)]
    pub proxies: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// CategoryRiotResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRiotResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryRobloxResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryRobloxResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategorySocialClubResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySocialClubResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategorySteamResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySteamResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// ManagingSteamValueResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamValueResponse {
    #[serde(rename = "appId")]
    pub app_id: Option<i64>,
    pub data: Option<serde_json::Value>,
    pub query: Option<String>,
    pub system_info: Option<SystemInfo>,
}

/// CategorySupercellResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategorySupercellResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryTelegramResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTelegramResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryTikTokResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryTikTokResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryUplayResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryUplayResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// ListStatesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListStatesResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "userItemStates")]
    #[serde(deserialize_with = "null_default", default)]
    pub user_item_states: serde_json::Value,
}

/// PaymentsHistoryResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PaymentsHistoryResponse {
    #[serde(rename = "filterDatesDefault")]
    #[serde(deserialize_with = "null_default", default)]
    pub filter_dates_default: bool,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub input: serde_json::Value,
    #[serde(rename = "lastOperationId")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_operation_id: i64,
    #[serde(rename = "nextPageHref")]
    #[serde(deserialize_with = "null_default", default)]
    pub next_page_href: String,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "pageNavLink")]
    #[serde(deserialize_with = "null_default", default)]
    pub page_nav_link: String,
    #[serde(rename = "pageNavParams")]
    #[serde(deserialize_with = "null_default", default)]
    pub page_nav_params: serde_json::Value,
    #[serde(rename = "paymentStats")]
    #[serde(deserialize_with = "null_default", default)]
    pub payment_stats: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub payments: serde_json::Value,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: String,
    #[serde(rename = "periodLabel")]
    #[serde(deserialize_with = "null_default", default)]
    pub period_label: String,
    #[serde(rename = "periodLabelPhrase")]
    #[serde(deserialize_with = "null_default", default)]
    pub period_label_phrase: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// CategoryVpnResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryVpnResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryWarfaceResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWarfaceResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryWotResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryWotBlitzResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryWotBlitzResponse {
    #[serde(rename = "cacheTTL")]
    #[serde(deserialize_with = "null_default", default)]
    pub cache_ttl: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub has_next_page: bool,
    #[serde(deserialize_with = "null_or_vec", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "null_default", default)]
    pub last_modified: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub page: i64,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "null_default", default)]
    pub per_page: i64,
    #[serde(rename = "searchUrl")]
    #[serde(deserialize_with = "null_default", default)]
    pub search_url: String,
    #[serde(rename = "serverTime")]
    #[serde(deserialize_with = "null_default", default)]
    pub server_time: i64,
    #[serde(rename = "stickyItems")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub sticky_items: Vec<serde_json::Value>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
    #[serde(rename = "totalItems")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    #[serde(deserialize_with = "null_default", default)]
    pub total_items_price: serde_json::Value,
    #[serde(rename = "wasCached")]
    #[serde(deserialize_with = "null_default", default)]
    pub was_cached: bool,
}

/// CategoryGamesResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryGamesResponse {
    pub games: Option<Vec<serde_json::Value>>,
    pub system_info: Option<SystemInfo>,
}

/// CategoryParamsResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoryParamsResponse {
    pub base_params: Option<serde_json::Value>,
    pub category: Option<serde_json::Value>,
    pub params: Option<Vec<serde_json::Value>>,
    pub system_info: Option<SystemInfo>,
}

/// ManagingGetResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingGetResponse {
    #[serde(rename = "canBuyItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_buy_item: bool,
    #[serde(rename = "canCancelConfirmedBuy")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_cancel_confirmed_buy: bool,
    #[serde(rename = "canChangeOwner")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_change_owner: bool,
    #[serde(rename = "canCloseItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_close_item: bool,
    #[serde(rename = "canDeleteItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_delete_item: bool,
    #[serde(rename = "canEditItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_edit_item: bool,
    #[serde(rename = "canOpenItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_open_item: bool,
    #[serde(rename = "canReportItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_report_item: bool,
    #[serde(rename = "canStickItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_stick_item: bool,
    #[serde(rename = "canUnstickItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_unstick_item: bool,
    #[serde(rename = "canViewItemHistory")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_view_item_history: bool,
    #[serde(rename = "canViewLoginData")]
    #[serde(deserialize_with = "null_default", default)]
    pub can_view_login_data: bool,
    #[serde(rename = "cannotBuyItemError")]
    #[serde(deserialize_with = "null_default", default)]
    pub cannot_buy_item_error: String,
    #[serde(rename = "faveCount")]
    #[serde(deserialize_with = "null_default", default)]
    pub fave_count: bool,
    #[serde(rename = "isVisibleItem")]
    #[serde(deserialize_with = "null_default", default)]
    pub is_visible_item: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub item: Item,
    #[serde(rename = "itemLink")]
    #[serde(deserialize_with = "null_default", default)]
    pub item_link: String,
    #[serde(rename = "sameItemsCount")]
    #[serde(deserialize_with = "null_default", default)]
    pub same_items_count: i64,
    #[serde(rename = "sameItemsIds")]
    #[serde(deserialize_with = "null_or_vec", default)]
    pub same_items_ids: Vec<i64>,
    #[serde(rename = "showToFavouritesButton")]
    #[serde(deserialize_with = "null_default", default)]
    pub show_to_favourites_button: bool,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ManagingAiPriceResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingAiPriceResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub price: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ManagingAutoBuyPriceResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingAutoBuyPriceResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub price: i64,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ManagingChangePasswordResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingChangePasswordResponse {
    pub message: Option<String>,
    #[serde(deserialize_with = "null_default", default)]
    pub new_password: String,
    pub status: Option<String>,
}

/// ManagingCheckGuaranteeResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingCheckGuaranteeResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub message: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// PurchasingConfirmResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PurchasingConfirmResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub item: serde_json::Value,
    pub status: Option<String>,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ManagingImageResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingImageResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub base64: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ManagingSteamInventoryValueResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamInventoryValueResponse {
    #[serde(rename = "appId")]
    pub app_id: Option<i64>,
    pub data: Option<serde_json::Value>,
    pub query: Option<String>,
    pub system_info: Option<SystemInfo>,
}

/// ManagingSteamGetMafileResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamGetMafileResponse {
    #[serde(rename = "maFile")]
    #[serde(deserialize_with = "null_default", default)]
    pub ma_file: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}

/// ManagingTelegramCodeResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingTelegramCodeResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub codes: serde_json::Value,
    #[serde(deserialize_with = "null_default", default)]
    pub item: Item,
}

/// ManagingTempEmailPasswordResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingTempEmailPasswordResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub item: serde_json::Value,
}

/// ManagingSteamUpdateValueResponse model from the LOLZTEAM API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagingSteamUpdateValueResponse {
    #[serde(deserialize_with = "null_default", default)]
    pub item: Item,
    #[serde(deserialize_with = "null_default", default)]
    pub status: String,
    #[serde(deserialize_with = "null_default", default)]
    pub system_info: SystemInfo,
}
