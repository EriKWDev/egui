use egui::{vec2, Color32, Id, Response, Rgba, Sense, Stroke, TextStyle, Ui, Visuals, WidgetText};

use super::{Node, NodeId, Nodes, ResizeState, SimplificationOptions, UiResponse};

/// Trait defining how the [`Dock`] and its leaf should be shown.
pub trait Behavior<Leaf> {
    /// Show this leaf node in the given [`egui::Ui`].
    ///
    /// If this is an unknown node, return [`NodeAction::Remove`] and the node will be removed.
    fn leaf_ui(&mut self, _ui: &mut Ui, _node_id: NodeId, _leaf: &mut Leaf) -> UiResponse;

    fn tab_text_for_leaf(&mut self, leaf: &Leaf) -> WidgetText;

    fn tab_text_for_node(&mut self, nodes: &Nodes<Leaf>, node_id: NodeId) -> WidgetText {
        match &nodes.nodes[&node_id] {
            Node::Leaf(leaf) => self.tab_text_for_leaf(leaf),
            Node::Branch(branch) => format!("{:?}", branch.get_layout()).into(),
        }
    }

    /// Show the title of a tab as a button.
    fn tab_ui(
        &mut self,
        nodes: &Nodes<Leaf>,
        ui: &mut Ui,
        id: Id,
        node_id: NodeId,
        active: bool,
        is_being_dragged: bool,
    ) -> Response {
        let text = self.tab_text_for_node(nodes, node_id);
        let font_id = TextStyle::Button.resolve(ui.style());
        let galley = text.into_galley(ui, Some(false), f32::INFINITY, font_id);
        let (_, rect) = ui.allocate_space(galley.size());
        let response = ui.interact(rect, id, Sense::click_and_drag());

        // Show a gap when dragged
        if ui.is_rect_visible(rect) && !is_being_dragged {
            {
                let mut bg_rect = rect;
                bg_rect.min.y = ui.max_rect().min.y;
                bg_rect.max.y = ui.max_rect().max.y;
                bg_rect = bg_rect.expand2(vec2(0.5 * ui.spacing().item_spacing.x, 0.0));

                let bg_color = self.tab_bg_color(ui.visuals(), active);
                let stroke = self.tab_outline_stroke(ui.visuals(), active);
                ui.painter().rect(bg_rect, 0.0, bg_color, stroke);

                if active {
                    // Make the tab name area connect with the tab ui area:
                    ui.painter().hline(
                        bg_rect.x_range(),
                        bg_rect.bottom(),
                        Stroke::new(stroke.width + 1.0, bg_color),
                    );
                }
            }

            let text_color = self.tab_text_color(ui.visuals(), active);
            ui.painter()
                .galley_with_color(rect.min, galley.galley, text_color);
        }

        response
    }

    /// Returns `false` if this leaf should be removed from its parent.
    fn retain_leaf(&mut self, _leaf: &Leaf) -> bool {
        true
    }

    // ---
    // Settings:

    /// The height of the bar holding tab names.
    fn tab_bar_height(&self, _style: &egui::Style) -> f32 {
        20.0
    }

    /// Width of the gap between nodes in a horizontal or vertical layout,
    /// and between rows/columns in a grid layout.
    fn gap_width(&self, _style: &egui::Style) -> f32 {
        1.0
    }

    // No child should shrink below this size
    fn min_size(&self) -> f32 {
        32.0
    }

    fn simplification_options(&self) -> SimplificationOptions {
        SimplificationOptions::default()
    }

    fn resize_stroke(&self, style: &egui::Style, resize_state: ResizeState) -> egui::Stroke {
        match resize_state {
            ResizeState::Idle => egui::Stroke::NONE, // Let the gap speak for itself
            ResizeState::Hovering => style.visuals.widgets.hovered.fg_stroke,
            ResizeState::Dragging => style.visuals.widgets.active.fg_stroke,
        }
    }

    /// The background color of the tab bar
    fn tab_bar_color(&self, visuals: &Visuals) -> Color32 {
        (Rgba::from(visuals.window_fill()) * Rgba::from_gray(0.7)).into()
    }

    fn tab_bg_color(&self, visuals: &Visuals, active: bool) -> Color32 {
        if active {
            // blend it with the tab contents:
            visuals.window_fill()
        } else {
            // fade into background:
            self.tab_bar_color(visuals)
        }
    }

    /// Stroke of the outline around a tab title.
    fn tab_outline_stroke(&self, visuals: &Visuals, active: bool) -> Stroke {
        if active {
            Stroke::new(1.0, visuals.widgets.active.bg_fill)
        } else {
            Stroke::NONE
        }
    }

    /// Stroke of the line separating the tab title bar and the content of the active tab.
    fn tab_bar_hline_stroke(&self, visuals: &Visuals) -> Stroke {
        Stroke::new(1.0, visuals.widgets.noninteractive.bg_stroke.color)
    }

    fn tab_text_color(&self, visuals: &Visuals, active: bool) -> Color32 {
        if active {
            visuals.widgets.active.text_color()
        } else {
            visuals.widgets.noninteractive.text_color()
        }
    }
}
