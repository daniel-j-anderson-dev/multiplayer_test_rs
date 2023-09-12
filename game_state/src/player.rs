use serde::{
    Deserialize, Deserializer,
    ser::SerializeStruct,
    Serialize,
    Serializer,
};

use macroquad::{
    color::Color,
    math::{
        Rect,
        Vec2,
    },
};

pub const PLAYER_WIDTH: f32 = 20.0;
pub const PLAYER_HEIGHT: f32 = 20.0;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]

pub struct Player {
    pub id: usize,

    #[serde(serialize_with = "serialize_rect", deserialize_with = "deserialize_rect")]
    pub boundary: Rect,

    #[serde(serialize_with = "serialize_vec2", deserialize_with = "deserialize_vec2")]
    pub velocity: Vec2,

    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub color: Color,
}

impl Player {
    pub fn new(id:usize, position: Vec2, color: Color) -> Player {
        return Player {
            id,
            boundary: Rect::new(position.x, position.y, PLAYER_WIDTH, PLAYER_HEIGHT),
            velocity: Vec2::ZERO,
            color,
        }
    }
    pub fn update(&mut self) {
        self.boundary.x += self.velocity.x;
        self.boundary.y += self.velocity.y;
    }
}

fn serialize_rect<S>(value: &Rect, serializer: S) -> Result<S::Ok, S::Error> 
where
    S: Serializer
{
    let mut state = serializer.serialize_struct("Rect", 4)?;
    state.serialize_field("x", &value.x)?;
    state.serialize_field("y", &value.y)?;
    state.serialize_field("w", &value.w)?;
    state.serialize_field("h", &value.h)?;
    state.end()
}
fn deserialize_rect<'de, D>(deserializer: D) -> Result<Rect, D::Error>
where
    D: Deserializer<'de>
{
    #[derive(Deserialize)]
    struct Intermediate {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    }

    let intermediate = Intermediate::deserialize(deserializer)?;

    let rect = Rect::new(intermediate.x, intermediate.y, intermediate.w, intermediate.h);

    Ok(rect)
}

fn serialize_vec2<S>(value: &Vec2, serializer: S) -> Result<S::Ok, S::Error> 
where
    S: Serializer
{
    let mut state = serializer.serialize_struct("Vec2", 2)?;
    state.serialize_field("x", &value.x)?;
    state.serialize_field("y", &value.y)?;
    state.end()
}
fn deserialize_vec2<'de, D>(deserializer: D) -> Result<Vec2, D::Error> 
where
    D: Deserializer<'de>
{
    #[derive(Deserialize)]
    struct Intermediate {
        x: f32,
        y: f32,
    }

    let intermediate = Intermediate::deserialize(deserializer)?;

    let vec2 = Vec2::new(intermediate.x, intermediate.y);

    Ok(vec2)
}

fn serialize_color<S>(value: &Color, serializer: S) -> Result<S::Ok, S::Error> 
where
    S: Serializer
{
    let mut state = serializer.serialize_struct("Color", 4)?;
    state.serialize_field("r", &value.r)?;
    state.serialize_field("g", &value.g)?;
    state.serialize_field("b", &value.b)?;
    state.serialize_field("a", &value.a)?;
    state.end()
}
fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error> 
where
    D: Deserializer<'de>
{
    #[derive(Deserialize)]
    struct Intermediate {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    }

    let intermediate = Intermediate::deserialize(deserializer)?;

    let color = Color::new(intermediate.r, intermediate.g, intermediate.b, intermediate.a);

    Ok(color)
}