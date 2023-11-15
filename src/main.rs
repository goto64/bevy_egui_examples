use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_egui::egui::{epaint};
use crate::catppuccin::catppuccin_egui;
use crate::expand_list_example::expand_list::ExpansionListPlugin;
use crate::notification_example::notification::NotificationsPlugin;
use crate::notification_example::notification_producer::{NotificationProducer, ui_notification_producer};

mod notification_example;
mod expand_list_example;
mod catppuccin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { ..Default::default() }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)
        .add_plugins((NotificationsPlugin, ExpansionListPlugin))
        .init_resource::<NotificationProducer>()
        .add_systems(Startup, ui_theme_selection)
        .add_systems(Update, ui_notification_producer)
        .run();
}

fn ui_theme_selection(mut contexts: EguiContexts) {
    catppuccin_egui::set_theme(contexts.ctx_mut(), catppuccin_egui::MOCHA);

    let old = contexts.ctx_mut().style().visuals.clone();
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_shadow: epaint::Shadow {
            color: catppuccin_egui::MOCHA.base,
            extrusion: 10.0,
        },
        ..old
    });
}