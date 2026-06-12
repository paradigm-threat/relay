//! Taffy flex layout for VNode trees.

use crate::style_props::{
    parse_style, AlignItems as PropAlign, Display as PropDisplay, FlexDirection as PropFlexDir,
    JustifyContent as PropJustify, StyleProps,
};
use relay_runtime::vdom::{PropValue, VNode};
use std::collections::HashMap;
use taffy::prelude::*;

#[derive(Debug)]
pub struct LayoutNode {
    pub vnode: VNode,
    pub style: StyleProps,
    pub taffy_id: NodeId,
    pub handler_id: Option<u32>,
}

pub struct LayoutTree {
    pub tree: TaffyTree,
    pub root: NodeId,
    pub nodes: HashMap<NodeId, LayoutNode>,
}

pub fn build_layout(vnode: VNode, viewport: Size<f32>) -> LayoutTree {
    let mut tree = TaffyTree::new();
    let mut nodes = HashMap::new();
    let root = build_node(&mut tree, &mut nodes, vnode);
    let _ = tree.compute_layout(
        root,
        taffy::Size {
            width: AvailableSpace::Definite(viewport.width),
            height: AvailableSpace::Definite(viewport.height),
        },
    );
    LayoutTree {
        tree,
        root,
        nodes,
    }
}

fn build_node(
    tree: &mut TaffyTree,
    nodes: &mut HashMap<NodeId, LayoutNode>,
    vnode: VNode,
) -> NodeId {
    match vnode {
        VNode::Text(text) => {
            let style = StyleProps {
                font_size: 16.0,
                ..Default::default()
            };
            let taffy_style = text_style(&style, &text);
            let id = tree.new_leaf(taffy_style).expect("leaf");
            nodes.insert(
                id,
                LayoutNode {
                    vnode: VNode::Text(text),
                    style,
                    taffy_id: id,
                    handler_id: None,
                },
            );
            id
        }
        VNode::Empty => {
            let id = tree.new_leaf(Style::DEFAULT).expect("empty leaf");
            nodes.insert(
                id,
                LayoutNode {
                    vnode: VNode::Empty,
                    style: StyleProps::default(),
                    taffy_id: id,
                    handler_id: None,
                },
            );
            id
        }
        VNode::Element {
            tag,
            props,
            children,
            key,
        } => {
            let style_map = props
                .get("style")
                .and_then(|p| {
                    if let PropValue::Style(m) = p {
                        Some(m.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
            let style = parse_style(&style_map);
            let handler_id = props.get("onClick").and_then(|p| {
                if let PropValue::HandlerId(id) = p {
                    Some(*id)
                } else {
                    None
                }
            });
            let child_ids: Vec<NodeId> = children
                .into_iter()
                .map(|c| build_node(tree, nodes, c))
                .collect();
            let taffy_style = element_style(&style, &tag);
            let id = if child_ids.is_empty() {
                tree.new_leaf(taffy_style).expect("element leaf")
            } else {
                tree.new_with_children(taffy_style, &child_ids)
                    .expect("element")
            };
            nodes.insert(
                id,
                LayoutNode {
                    vnode: VNode::Element {
                        tag,
                        props,
                        children: vec![],
                        key,
                    },
                    style,
                    taffy_id: id,
                    handler_id,
                },
            );
            id
        }
    }
}

fn element_style(style: &StyleProps, tag: &str) -> Style {
    let mut s = Style::DEFAULT;
    if style.display == PropDisplay::None {
        s.display = Display::None;
        return s;
    }
    s.display = Display::Flex;
    s.flex_direction = match style.flex_direction {
        PropFlexDir::Column => FlexDirection::Column,
        PropFlexDir::Row => FlexDirection::Row,
    };
    s.justify_content = Some(match style.justify_content {
        PropJustify::Center => JustifyContent::Center,
        PropJustify::FlexEnd => JustifyContent::FlexEnd,
        PropJustify::SpaceBetween => JustifyContent::SpaceBetween,
        PropJustify::FlexStart => JustifyContent::Start,
    });
    s.align_items = Some(match style.align_items {
        PropAlign::FlexStart => AlignItems::FlexStart,
        PropAlign::Center => AlignItems::Center,
        PropAlign::FlexEnd => AlignItems::FlexEnd,
        PropAlign::Stretch => AlignItems::Stretch,
    });
    if style.flex_grow > 0.0 {
        s.flex_grow = style.flex_grow;
    }
    if style.flex_shrink > 0.0 {
        s.flex_shrink = style.flex_shrink;
    }
    if let Some(w) = style.width {
        if w.is_nan() {
            s.size.width = Dimension::from_percent(1.0);
        } else {
            s.size.width = Dimension::Length(w);
        }
    }
    if let Some(h) = style.height {
        if h.is_nan() {
            s.size.height = Dimension::from_percent(1.0);
        } else {
            s.size.height = Dimension::Length(h);
        }
    }
    if let Some(w) = style.min_width {
        s.min_size.width = Dimension::Length(w).into();
    }
    if let Some(h) = style.min_height {
        s.min_size.height = Dimension::Length(h).into();
    }
    s.padding = Rect {
        left: LengthPercentage::Length(style.padding.left),
        right: LengthPercentage::Length(style.padding.right),
        top: LengthPercentage::Length(style.padding.top),
        bottom: LengthPercentage::Length(style.padding.bottom),
    };
    s.margin = Rect {
        left: LengthPercentageAuto::Length(style.margin.left),
        right: LengthPercentageAuto::Length(style.margin.right),
        top: LengthPercentageAuto::Length(style.margin.top),
        bottom: LengthPercentageAuto::Length(style.margin.bottom),
    };
    if style.gap > 0.0 {
        s.gap = Size {
            width: LengthPercentage::Length(style.gap),
            height: LengthPercentage::Length(style.gap),
        };
    }
    if tag == "button" {
        s.min_size.width = Dimension::Length(48.0).into();
        s.min_size.height = Dimension::Length(32.0).into();
    }
    s
}

fn text_style(style: &StyleProps, text: &str) -> Style {
    let mut s = Style::DEFAULT;
    let est_w = (text.len() as f32 * style.font_size * 0.55).max(8.0);
    let est_h = style.font_size * 1.25;
    s.size.width = Dimension::Length(est_w);
    s.size.height = Dimension::Length(est_h);
    s
}

pub fn hit_test(tree: &LayoutTree, x: f32, y: f32) -> Option<u32> {
    hit_recursive(&tree.tree, tree.root, &tree.nodes, x, y)
}

fn hit_recursive(
    taffy: &TaffyTree,
    id: NodeId,
    nodes: &HashMap<NodeId, LayoutNode>,
    x: f32,
    y: f32,
) -> Option<u32> {
    let layout = taffy.layout(id).ok()?;
    let lx = layout.location.x;
    let ly = layout.location.y;
    let lw = layout.size.width;
    let lh = layout.size.height;
    if x < lx || y < ly || x > lx + lw || y > ly + lh {
        return None;
    }
    if let Some(children) = taffy.children(id).ok() {
        for &child in children.iter().rev() {
            if let Some(h) = hit_recursive(taffy, child, nodes, x - lx, y - ly) {
                return Some(h);
            }
        }
    }
    nodes.get(&id).and_then(|n| n.handler_id)
}
