use dioxus::core::*;
use std::collections::HashMap;
use tui::style::Style as TuiStyle;

use crate::{
    attributes::{apply_attributes, StyleModifer},
    TuiNode,
};

pub fn collect_layout<'a>(
    layout: &mut stretch2::Stretch,
    nodes: &mut HashMap<ElementId, TuiNode<'a>>,
    vdom: &'a VirtualDom,
    node: &'a VNode<'a>,
) {
    use stretch2::prelude::*;

    match node {
        VNode::Text(t) => {
            //
            let id = t.id.get().unwrap();
            let char_len = t.text.chars().count();

            let style = Style {
                size: Size {
                    // characters are 1 point tall
                    height: Dimension::Points(1.0),

                    // text is as long as it is declared
                    width: Dimension::Points(char_len as f32),
                },

                ..Default::default()
            };

            nodes.insert(
                id,
                TuiNode {
                    node,
                    block_style: tui::style::Style::default(),
                    layout: layout.new_node(style, &[]).unwrap(),
                },
            );
        }
        VNode::Element(el) => {
            // gather up all the styles from the attribute list
            let mut modifier = StyleModifer {
                style: Style::default(),
                tui_style: TuiStyle::default(),
            };

            for &Attribute { name, value, .. } in el.attributes {
                apply_attributes(name, value, &mut modifier);
            }

            // Layout the children
            for child in el.children {
                collect_layout(layout, nodes, vdom, child);
            }

            // Set all direct nodes as our children
            let mut child_layout = vec![];
            for el in el.children {
                let ite = ElementIdIterator::new(vdom, el);
                for node in ite {
                    child_layout.push(nodes[&node.mounted_id()].layout)
                }
            }

            nodes.insert(
                node.mounted_id(),
                TuiNode {
                    node,
                    block_style: modifier.tui_style,
                    layout: layout.new_node(modifier.style, &child_layout).unwrap(),
                },
            );
        }
        VNode::Fragment(el) => {
            //
            for child in el.children {
                collect_layout(layout, nodes, vdom, child);
            }
        }
        VNode::Component(_) => todo!(),
        VNode::Placeholder(_) => todo!(),
    };
}