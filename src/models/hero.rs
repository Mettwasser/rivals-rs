use std::collections::HashMap;

use serde::{
    de::Visitor,
    Deserialize,
};

pub type Heroes = Vec<Hero>;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Hero {
    pub id: String,

    pub name: String,

    pub real_name: String,

    #[serde(rename = "imageUrl")]
    pub image_url: String,

    pub role: Role,

    pub attack_type: AttackType,

    pub team: Vec<String>,

    pub difficulty: String,

    pub bio: String,

    pub lore: String,

    pub transformations: Vec<Transformation>,

    pub costumes: Vec<Costume>,

    pub abilities: Vec<Ability>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Ability {
    pub id: i64,

    pub icon: Option<String>,

    pub name: Option<String>,

    #[serde(rename = "type")]
    pub ability_type: Type,

    #[serde(rename = "isCollab")]
    pub is_collab: bool,

    pub description: Option<String>,

    pub additional_fields: Option<HashMap<String, String>>,

    pub transformation_id: String,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Normal,

    Passive,

    Ultimate,

    Weapon,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttackType {
    #[serde(rename = "Hitscan Heroes")]
    HitscanHeroes,

    #[serde(rename = "Melee Heroes")]
    MeleeHeroes,

    #[serde(rename = "Projectile Heroes")]
    ProjectileHeroes,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Costume {
    pub id: String,

    pub name: String,

    pub icon: String,

    pub quality: Quality,

    pub description: String,

    pub appearance: String,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Quality {
    Blue,

    #[serde(rename = "NO_QUALITY")]
    NoQuality,

    Orange,

    Purple,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Duelist,

    Strategist,

    Vanguard,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Transformation {
    pub id: String,

    pub name: String,

    pub icon: String,

    pub health: Option<String>,

    pub movement_speed: Option<MovementSpeed>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MovementSpeed(pub f32);

impl<'de> Deserialize<'de> for MovementSpeed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(MovementSpeedVisitor)
    }
}

struct MovementSpeedVisitor;
impl Visitor<'_> for MovementSpeedVisitor {
    type Value = MovementSpeed;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string containing a number followed by 'm/s'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let number: f32 = value
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect::<String>()
            .parse()
            .map_err(|err| E::custom(err))?;

        Ok(MovementSpeed(number))
    }
}
