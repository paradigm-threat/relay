//! tiny-skia paint pass for laid-out VNodes.

use crate::layout::{LayoutNode, LayoutTree};
use ab_glyph::{Font, FontRef, Glyph, PxScale, ScaleFont, point};
use relay_runtime::vdom::{PropValue, VNode};
use std::collections::HashMap;
use taffy::prelude::*;
use tiny_skia::{Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

pub struct Painter {
    font: FontRef<'static>,
}

impl Default for Painter {
    fn default() -> Self {
        let font = FontRef::try_from_slice(include_bytes!("../assets/DejaVuSans.ttf"))
            .expect("embedded font");
        Self { font }
    }
}

impl Painter {
    pub fn paint_layout(&self, pixmap: &mut Pixmap, layout: &LayoutTree) {
        pixmap.fill(tiny_skia::Color::from_rgba8(245, 244, 240, 255));
        self.paint_node_id(pixmap, &layout.tree, layout.root, &layout.nodes, 0.0, 0.0);
    }

    fn paint_node_id(
        &self,
        pixmap: &mut Pixmap,
        taffy: &TaffyTree,
        id: NodeId,
        nodes: &HashMap<NodeId, LayoutNode>,
        offset_x: f32,
        offset_y: f32,
    ) {
        let taffy_layout = match taffy.layout(id) {
            Ok(l) => l,
            Err(_) => return,
        };
        let x = offset_x + taffy_layout.location.x;
        let y = offset_y + taffy_layout.location.y;
        let w = taffy_layout.size.width;
        let h = taffy_layout.size.height;
        let meta = match nodes.get(&id) {
            Some(m) => m,
            None => return,
        };

        if let Some(bg) = meta.style.background_color {
            let mut paint = Paint::default();
            paint.set_color(bg);
            if let Some(rect) = Rect::from_xywh(x, y, w, h) {
                pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            }
        }

        match &meta.vnode {
            VNode::Text(text) => {
                self.draw_text(
                    pixmap,
                    text,
                    x,
                    y + meta.style.font_size,
                    meta.style.color.unwrap_or(tiny_skia::Color::BLACK),
                    meta.style.font_size,
                );
            }
            VNode::Element { tag, props, .. } => {
                if tag == "button" || tag == "li" {
                    let mut paint = Paint::default();
                    paint.set_color(tiny_skia::Color::from_rgba8(180, 180, 180, 255));
                    let stroke = Stroke {
                        width: 1.0,
                        ..Default::default()
                    };
                    if let Some(rect) = Rect::from_xywh(x, y, w, h) {
                        let path = PathBuilder::from_rect(rect);
                        pixmap.stroke_path(
                            &path,
                            &paint,
                            &stroke,
                            Transform::identity(),
                            None,
                        );
                    }
                }
                if let Some(PropValue::String(label)) = props.get("aria-label") {
                    self.draw_text(
                        pixmap,
                        label,
                        x + 8.0,
                        y + h * 0.65,
                        meta.style.color.unwrap_or(tiny_skia::Color::BLACK),
                        meta.style.font_size,
                    );
                }
            }
            _ => {}
        }

        if let Ok(children) = taffy.children(id) {
            for child in children {
                self.paint_node_id(pixmap, taffy, child, nodes, x, y);
            }
        }
    }

    fn draw_text(
        &self,
        pixmap: &mut Pixmap,
        text: &str,
        x: f32,
        y: f32,
        color: tiny_skia::Color,
        size: f32,
    ) {
        let scale = PxScale::from(size);
        let scaled = self.font.as_scaled(scale);
        let (r, g, b, a_base) = (
            (color.red() * 255.0) as u8,
            (color.green() * 255.0) as u8,
            (color.blue() * 255.0) as u8,
            (color.alpha() * 255.0) as u8,
        );
        let mut pen_x = x;
        for ch in text.chars() {
            let id = scaled.glyph_id(ch);
            let glyph = Glyph {
                id,
                scale,
                position: point(pen_x + scaled.h_side_bearing(id), y),
            };
            if let Some(outline) = self.font.outline_glyph(glyph) {
                outline.draw(|gx, gy, v| {
                    if v <= 0.0 {
                        return;
                    }
                    let px = gx as u32;
                    let py = gy as u32;
                    if px >= pixmap.width() || py >= pixmap.height() {
                        return;
                    }
                    let idx = (py * pixmap.width() + px) as usize;
                    let pixels = pixmap.pixels_mut();
                    if idx >= pixels.len() {
                        return;
                    }
                    let a = (v * f32::from(a_base)) as u8;
                    pixels[idx] = tiny_skia::PremultipliedColorU8::from_rgba(r, g, b, a)
                        .unwrap_or(pixels[idx]);
                });
            }
            pen_x += scaled.h_advance(id);
        }
    }
}
