use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Color32, Pos2, Sense};
use crate::notification_example::notification::Notifications;

pub struct TextLayoutPlugin;

impl Plugin for TextLayoutPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InteractiveTextState>()
            .add_systems(Update, text_layout_window);
    }
}

#[derive(Resource)]
pub struct InteractiveTextState {
    hovered: bool,
}

impl Default for InteractiveTextState {
    fn default() -> Self {
        Self {
            hovered: false,
        }
    }
}

pub fn text_layout_window(
    mut contexts: EguiContexts,
    mut notifications: ResMut<Notifications>,
    mut text_state: ResMut<InteractiveTextState>,
) {
    egui::Window::new("Interactive Text")
        .resizable(true)
        .collapsible(false)
        .default_pos(Pos2{x: 400.0, y: 250.0})
        .default_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            let txt1 = egui::RichText::new("This is an example of a paragraph of text. You can").size(14.0);
            let mut txt2 = egui::RichText::new("click this link").size(14.0).underline();
            if text_state.hovered {
                txt2 = txt2.color(Color32::LIGHT_YELLOW);
            } else {
                txt2 = txt2.color(Color32::LIGHT_BLUE);
            }
            let txt3 = egui::RichText::new("to display a notification").size(14.0);

            ui.horizontal_wrapped(|ui| {
                ui.label(txt1);
                let interactive_txt = ui.add(egui::Label::new(txt2).sense(Sense::click()));
                text_state.hovered = interactive_txt.hovered();
                if interactive_txt.clicked() {
                    notifications.add_notification(String::from("This is a notification from the interactive text"));
                }
                ui.label(txt3);
            });
        });
}
