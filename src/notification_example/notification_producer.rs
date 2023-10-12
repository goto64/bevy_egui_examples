use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Pos2};
use crate::notification_example::notification::Notifications;

#[derive(Resource)]
pub struct NotificationProducer {
    current_text: String,
}

impl Default for NotificationProducer {
    fn default() -> Self {
        Self {
            current_text: String::from("Some notification text"),
        }
    }
}

pub fn ui_notification_producer(
    mut contexts: EguiContexts,
    mut notification_producer: ResMut<NotificationProducer>,
    mut notifications: ResMut<Notifications>,
) {
    const WIN_WIDTH: f32 = 300.0;

    egui::Window::new("Notification producer")
        .resizable(false)
        .collapsible(false)
        .movable(true)
        .default_pos(Pos2{ x: 400.0, y: 50.0 })
        .show(contexts.ctx_mut(), |ui|
        {
            ui.set_width(WIN_WIDTH);

            ui.add_space(5.0);
            ui.label(bigger_text("Notification text:"));
            ui.text_edit_multiline(&mut notification_producer.current_text);
            ui.add_space(5.0);
            if ui.button(bigger_text("Queue notification")).clicked() {
                notifications.add_notification(notification_producer.current_text.clone());
            }
        });
}

fn bigger_text(text: &'static str) -> egui::RichText {
    return egui::RichText::new(text).size(14.0);
}