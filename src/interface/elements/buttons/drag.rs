use derive_new::new;

use crate::graphics::{InterfaceRenderer, Renderer};
use crate::interface::{Element, *};

#[derive(new)]
pub struct DragButton {
    window_title: String,
    #[new(default)]
    state: ElementState,
}

impl Element for DragButton {

    fn get_state(&self) -> &ElementState {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut ElementState {
        &mut self.state
    }

    fn resolve(&mut self, placement_resolver: &mut PlacementResolver, _interface_settings: &InterfaceSettings, theme: &Theme) {
        self.state.resolve(placement_resolver, &theme.window.title_size_constraint);
    }

    fn is_focusable(&self) -> bool {
        false
    }

    fn hovered_element(&self, mouse_position: Position) -> HoverInformation {
        self.state.hovered_element(mouse_position)
    }

    fn left_click(&mut self, _force_update: &mut bool) -> Option<ClickAction> {
        Some(ClickAction::MoveInterface)
    }

    fn render(
        &self,
        render_target: &mut <InterfaceRenderer as Renderer>::Target,
        renderer: &InterfaceRenderer,
        _state_provider: &StateProvider,
        interface_settings: &InterfaceSettings,
        theme: &Theme,
        parent_position: Position,
        clip_size: ClipSize,
        hovered_element: Option<&dyn Element>,
        _focused_element: Option<&dyn Element>,
        _second_theme: bool,
    ) {

        let mut renderer = self
            .state
            .element_renderer(render_target, renderer, interface_settings, parent_position, clip_size);

        if self.is_element_self(hovered_element) {
            renderer.render_background(*theme.window.title_border_radius, *theme.window.title_background_color);
        }

        renderer.render_text(
            &self.window_title,
            *theme.window.text_offset,
            *theme.window.foreground_color,
            *theme.window.font_size,
        );
    }
}
