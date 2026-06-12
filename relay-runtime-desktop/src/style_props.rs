//! Parse resolved `props.style` strings into layout/paint values.

use tiny_skia::Color;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct StyleProps {
    pub display: Display,
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub padding: Edges,
    pub margin: Edges,
    pub background_color: Option<Color>,
    pub color: Option<Color>,
    pub font_size: f32,
    pub font_weight: f32,
    pub border_radius: f32,
    pub gap: f32,
    pub cursor_pointer: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Display {
    #[default]
    Flex,
    Block,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexDirection {
    #[default]
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JustifyContent {
    #[default]
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlignItems {
    #[default]
    Stretch,
    FlexStart,
    Center,
    FlexEnd,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Edges {
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

pub fn parse_style(map: &HashMap<String, String>) -> StyleProps {
    let mut s = StyleProps::default();
    for (k, v) in map {
        let key = k.as_str();
        let val = v.trim();
        match key {
            "display" => {
                s.display = match val {
                    "none" => Display::None,
                    "block" => Display::Block,
                    _ => Display::Flex,
                };
            }
            "flexDirection" => {
                s.flex_direction = if val.eq_ignore_ascii_case("column") {
                    FlexDirection::Column
                } else {
                    FlexDirection::Row
                };
            }
            "justifyContent" => {
                s.justify_content = match val {
                    "center" => JustifyContent::Center,
                    "flex-end" | "end" => JustifyContent::FlexEnd,
                    "space-between" => JustifyContent::SpaceBetween,
                    _ => JustifyContent::FlexStart,
                };
            }
            "alignItems" => {
                s.align_items = match val {
                    "flex-start" | "start" => AlignItems::FlexStart,
                    "center" => AlignItems::Center,
                    "flex-end" | "end" => AlignItems::FlexEnd,
                    _ => AlignItems::Stretch,
                };
            }
            "flex" | "flexGrow" => {
                if let Ok(n) = val.parse::<f32>() {
                    s.flex_grow = n;
                }
            }
            "flexShrink" => {
                if let Ok(n) = val.parse::<f32>() {
                    s.flex_shrink = n;
                }
            }
            "width" => s.width = parse_length(val),
            "height" => s.height = parse_length(val),
            "minWidth" => s.min_width = parse_length(val),
            "minHeight" => s.min_height = parse_length(val),
            "padding" => s.padding = parse_edges(val),
            "paddingTop" => s.padding.top = parse_px(val).unwrap_or(0.0),
            "paddingRight" => s.padding.right = parse_px(val).unwrap_or(0.0),
            "paddingBottom" => s.padding.bottom = parse_px(val).unwrap_or(0.0),
            "paddingLeft" => s.padding.left = parse_px(val).unwrap_or(0.0),
            "margin" => s.margin = parse_edges(val),
            "marginTop" => s.margin.top = parse_px(val).unwrap_or(0.0),
            "marginRight" => s.margin.right = parse_px(val).unwrap_or(0.0),
            "marginBottom" => s.margin.bottom = parse_px(val).unwrap_or(0.0),
            "marginLeft" => s.margin.left = parse_px(val).unwrap_or(0.0),
            "backgroundColor" => s.background_color = parse_color(val),
            "color" => s.color = parse_color(val),
            "fontSize" => s.font_size = parse_px(val).unwrap_or(16.0),
            "fontWeight" => s.font_weight = parse_px(val).unwrap_or(400.0),
            "borderRadius" => s.border_radius = parse_px(val).unwrap_or(0.0),
            "gap" => s.gap = parse_px(val).unwrap_or(0.0),
            "cursor" if val == "pointer" => s.cursor_pointer = true,
            _ => {}
        }
    }
    if s.font_size <= 0.0 {
        s.font_size = 16.0;
    }
    s
}

fn parse_length(s: &str) -> Option<f32> {
    if s == "100%" {
        return Some(f32::NAN);
    }
    parse_px(s)
}

fn parse_px(s: &str) -> Option<f32> {
    let s = s.trim();
    if s.ends_with("px") {
        s.trim_end_matches("px").trim().parse().ok()
    } else if s.ends_with('%') {
        None
    } else {
        s.parse().ok()
    }
}

fn parse_edges(s: &str) -> Edges {
    let parts: Vec<&str> = s.split_whitespace().collect();
    match parts.len() {
        1 => {
            let v = parse_px(parts[0]).unwrap_or(0.0);
            Edges {
                top: v,
                right: v,
                bottom: v,
                left: v,
            }
        }
        2 => {
            let v = parse_px(parts[0]).unwrap_or(0.0);
            let h = parse_px(parts[1]).unwrap_or(0.0);
            Edges {
                top: v,
                right: h,
                bottom: v,
                left: h,
            }
        }
        4 => Edges {
            top: parse_px(parts[0]).unwrap_or(0.0),
            right: parse_px(parts[1]).unwrap_or(0.0),
            bottom: parse_px(parts[2]).unwrap_or(0.0),
            left: parse_px(parts[3]).unwrap_or(0.0),
        },
        _ => Edges::default(),
    }
}

pub fn parse_color(s: &str) -> Option<Color> {
    let s = s.trim();
    if s.starts_with('#') {
        let hex = &s[1..];
        let (r, g, b, a) = match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                (r, g, b, 255)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                (r, g, b, a)
            }
            _ => return None,
        };
        return Some(Color::from_rgba8(r, g, b, a));
    }
    None
}
