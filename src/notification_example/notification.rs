use std::collections::VecDeque;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Pos2, Ui};

/// Plugin to display text notifications that slide in from the right
pub struct NotificationsPlugin;

impl Plugin for NotificationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Notifications::new())
            .add_systems(Update, display_notification);
    }
}

#[derive(Resource)]
pub struct Notifications {
    queued: VecDeque<String>,
    displaying: Option<String>,
    timeout: f32,
    offset_x: f32,
}

impl Notifications {
    pub fn new() -> Self {
        Self {
            queued: VecDeque::new(),
            displaying: None,
            timeout: 0.0,
            offset_x: 0.0,
        }
    }

    pub fn add_notification(&mut self, notif_text: String) {
        self.queued.push_back(notif_text);
    }
}

const NOTIFICATION_TIMER_SECS: f32 = 4.0;
const NOTIFICATION_PAUSE_SECS: f32 = 0.5;

fn display_notification(
    mut contexts: EguiContexts,
    mut notifications: ResMut<Notifications>,
    time: Res<Time>,
) {
    const WIN_WIDTH: f32 = 300.0;
    const WIN_HEIGHT: f32 = 100.0;

    if notifications.displaying.is_some() {
        notifications.timeout -= time.delta_seconds();
        // Windows appearance animation
        notifications.offset_x -= (WIN_WIDTH * 3.0 * time.delta_seconds()).min(notifications.offset_x);

        if notifications.timeout <= 0.0 { // Notification timed out, remove
            notifications.displaying = None;
            notifications.timeout = NOTIFICATION_PAUSE_SECS; // Wait before showing next notification
        } else { // Display current notification
            let rect = contexts.ctx_mut().screen_rect();
            let (max_x, max_y) = (rect.max.x, rect.max.y);

            egui::Window::new("Notification")
                .fixed_pos(Pos2 { x: max_x - (WIN_WIDTH + 180.0) + notifications.offset_x, y: max_y - (WIN_HEIGHT + 50.0) })
                .resizable(false)
                .title_bar(false)
                .show(contexts.ctx_mut(), |ui|
            {
                ui.set_width(WIN_WIDTH);
                ui.set_max_height(WIN_HEIGHT);
                ui.horizontal(|ui| {
                    notification_icon(ui);
                    ui.add_space(10.0);
                    let notif_text = egui::Label::new(notifications.displaying.as_ref().unwrap()).wrap(true);
                    let height = ui.add(notif_text).rect.size().y;
                    ui.set_max_height(height);
                });
            });
        }
    } else if notifications.timeout > 0.0 { // Decrease timer
        notifications.timeout -= time.delta_seconds();
    } else if notifications.queued.len() > 0 { // Show next notification in queue
        notifications.timeout = NOTIFICATION_TIMER_SECS;
        notifications.displaying = notifications.queued.pop_front();
        notifications.offset_x = WIN_WIDTH + 180.0;
    }
}

fn notification_icon(ui: &mut Ui) {
    ui.label(egui::RichText::new("！")
        .size(32.0)
        .strong()
        .color(egui::Color32::from_rgb(248, 197, 20)));
}