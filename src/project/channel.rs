/*
 Crate:         ore
 File:          /project/category.rs
 Module:        ::project::category
 Visibility:    public
 */

// TODO: documentation

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error as DeserializeError, Visitor};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

// TODO documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    color: Color,
    name: String,
}

impl Channel {

    // TODO: documentation
    pub fn color(&self) -> Color {
        self.color
    }

    // TODO: documentation
    pub fn name(&self) -> String {
        self.name.to_owned()
    }
}

impl Display for Channel {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.name)
    }
}

// TODO documentation
#[derive(Clone, Copy, Debug)]
pub enum Color {
    RGB(u8, u8, u8),
    Transparent,
}

impl Color {

    // TODO: documentation
    pub fn red(&self) -> Option<u8> {
        match *self {
            Color::RGB(r, ..) => Some(r),
            Color::Transparent => None,
        }
    }

    // TODO: documentation
    pub fn green(&self) -> Option<u8> {
        match *self {
            Color::RGB(_, g, ..) => Some(g),
            Color::Transparent => None,
        }
    }

    // TODO: documentation
    pub fn blue(&self) -> Option<u8> {
        match *self {
            Color::RGB(_, _, b) => Some(b),
            Color::Transparent => None,
        }
    }

    // TODO: documentation
    pub fn rbg(&self) -> Option<(u8, u8, u8)> {
        match *self {
            Color::RGB(r, g, b) => Some((r, g, b)),
            Color::Transparent => None,
        }
    }
}

#[doc(hidden)]
impl<'a> Deserialize<'a> for Color {

    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
        where D: Deserializer<'a>
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

impl Display for Color {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Color::Transparent => write!(f, "transparent"),
            Color::RGB(ref r, ref g, ref b) => write!(f, "#{:x}{:x}{:x}", r, g, b),
        }
    }
}

#[doc(hidden)]
impl Serialize for Color {

    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(format!("{}", match *self {
            Color::RGB(ref r, ref g, ref b) => format!("#{:x}{:x}{:x}", r, g, b),
            Color::Transparent => "transparent".to_string(),
        }).as_str())
    }
}

// TODO: documentation
struct ColorVisitor;

impl<'a> Visitor<'a> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "\"transparent\" or a color code like \"#xxxxxx\"")
    }

    fn visit_str<E>(self, c: &str) -> Result<Self::Value, E>
        where E: DeserializeError
    {
        match c {
            "transparent" => Ok(Color::Transparent),
            ref s => {
                if s.len() != 7 {
                    Err(DeserializeError::custom("color code has to be 7 characters long"))
                } else {
                    if !s.starts_with("#") {
                        return Err(
                            DeserializeError::custom("color code needs to start with a \"#\""))
                    }

                    let mut rgb: (u8, u8, u8) = (0, 0, 0);
                    let mut chars = s.chars();
                    let mut buf = String::new();

                    buf.push(chars.nth(1).unwrap());
                    buf.push(chars.nth(2).unwrap());

                    match buf.as_str().parse::<u8>() {
                        Ok(x) => rgb.0 = x,
                        Err(..) => return Err(DeserializeError::custom(
                            r#""red" component of the color code is incorrect"#))
                    }

                    buf.clear();
                    buf.push(chars.nth(3).unwrap());
                    buf.push(chars.nth(4).unwrap());

                    match buf.as_str().parse::<u8>() {
                        Ok(x) => rgb.1 = x,
                        Err(..) => return Err(DeserializeError::custom(
                            r#""green" component of the color code is incorrect"#))
                    }

                    buf.clear();
                    buf.push(chars.nth(5).unwrap());
                    buf.push(chars.nth(6).unwrap());

                    match buf.as_str().parse::<u8>() {
                        Ok(x) => rgb.2 = x,
                        Err(..) => return Err(DeserializeError::custom(
                            r#""blue" component of the color code is incorrect"#))
                    }

                    Ok(Color::RGB(rgb.0, rgb.1, rgb.2))
                }
            }
        }
    }
}
