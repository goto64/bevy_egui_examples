use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{emath, Frame, Pos2, TextureId, Ui};
use crate::notification_example::notification::Notifications;

/// Demonstration of how to create an expandable selection list with icons and text
pub struct ExpansionListPlugin;

impl Plugin for ExpansionListPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ExpansionList>()
            .add_systems(Startup, load_food_images)
            .add_systems(Update, expand_list);
    }
}

#[derive(Resource)]
pub struct ExpansionList {
    expanded: bool,
    selected: usize,
}

impl Default for ExpansionList {
    fn default() -> Self {
        Self {
            expanded: true,
            selected: 0,
        }
    }
}

fn expand_list(
    mut contexts: EguiContexts,
    mut expansion_list: ResMut<ExpansionList>,
    food_images: Res<FoodImages>,
    mut notifications: ResMut<Notifications>,
) {
    egui::Window::new("Expansion list")
        .resizable(false)
        .collapsible(false)
        .default_pos(Pos2{x: 50.0, y: 50.0})
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width(225.0);

            egui::TopBottomPanel::show_animated_between_inside(
                ui, expansion_list.expanded,
                egui::TopBottomPanel::top("BuildCollapsed").show_separator_line(false).frame(Frame::none()),
                egui::TopBottomPanel::top("BuildExpand").show_separator_line(false).frame(Frame::none()),
                |ui, _|
            {
                if !expansion_list.expanded { // Collapsed
                    ui.label(bigger_text(String::from("Click to change food:")));
                    if food_images.clickable_food_button(
                        ui, food_images.images[expansion_list.selected], food_images.names[expansion_list.selected].clone(), None) {
                        expansion_list.expanded = true;
                    }
                } else { // Expanded
                   ui.label(bigger_text(String::from("Select food:")));
                   let selected_food = machine_selection_ui(ui, &food_images, expansion_list.selected);
                   if selected_food.is_some() {
                       expansion_list.selected = selected_food.unwrap();
                       expansion_list.expanded  = false;
                   }
                }

                ui.add_space(10.0);
                if ui.button(bigger_text(format!("Eat {}", food_images.names[expansion_list.selected]))).clicked() {
                    notifications.add_notification(format!("Eating {}", food_images.names[expansion_list.selected]));
                }
            });
        });
}

fn bigger_text(text: String) -> egui::RichText {
    return egui::RichText::new(text).size(14.0);
}

const LIST_HIGHLIGHT_COLOR: egui::Color32 = egui::Color32::from_rgb(52, 66, 73);

fn machine_selection_ui(
    ui: &mut Ui,
    food_images: &FoodImages,
    prev_food: usize
) -> Option<usize> {
    let mut selected_food = None;

    for i in 0..food_images.images.len() {
        if food_images.clickable_food_button(
            ui, food_images.images[i], food_images.names[i].clone(),
            if prev_food == i { Some(LIST_HIGHLIGHT_COLOR) } else { None }) {
            selected_food = Some(i);
        }
    }

    return selected_food;
}

#[derive(Resource)]
struct FoodImages {
    images: Vec<TextureId>,
    names: Vec<String>,
}

impl FoodImages {
    fn clickable_food_button(&self, ui: &mut Ui, image: TextureId, text: impl Into<String>, col: Option<egui::Color32>) -> bool {
        let mut btn = egui::Button::image_and_text(
            image, emath::Vec2 { x: 32.0, y: 32.0 }, egui::RichText::new(text).size(16.0).monospace())
            .min_size(emath::Vec2 { x: 220.0, y: 32.0 });
        if col.is_some() {
            btn = btn.fill(col.unwrap());
        }
        let clicked = ui.add(btn).clicked();

        return clicked;
    }
}

fn load_food_images(
    asset_server: Res<AssetServer>,
    mut contexts: EguiContexts,
    mut commands: Commands,
) {
    let mut images: Vec<TextureId> = Vec::new();
    images.push(ui_add_image(&mut contexts, &asset_server, "food/13_bacon.png"));
    images.push(ui_add_image(&mut contexts, &asset_server, "food/15_burger.png"));
    images.push(ui_add_image(&mut contexts, &asset_server, "food/18_burrito.png"));
    images.push(ui_add_image(&mut contexts, &asset_server, "food/20_bagel.png"));
    images.push(ui_add_image(&mut contexts, &asset_server, "food/22_cheesecake.png"));
    images.push(ui_add_image(&mut contexts, &asset_server, "food/24_cheesepuff.png"));
    images.push(ui_add_image(&mut contexts, &asset_server, "food/26_chocolate.png"));
    images.push(ui_add_image(&mut contexts, &asset_server, "food/28_cookies.png"));

    let names: Vec<String> = vec![
        String::from("Bacon"),
        String::from("Burger"),
        String::from("Burrito"),
        String::from("Bagel"),
        String::from("Cheesecake"),
        String::from("Cheese puff"),
        String::from("Chocolate"),
        String::from("Cookie"),
    ];

    assert_eq!(images.len(), names.len());

    commands.insert_resource(FoodImages {
        images,
        names,
    });
}

fn ui_add_image(contexts: &mut EguiContexts, asset_server: &AssetServer, path: &str) -> TextureId {
    let img: Handle<Image> = asset_server.load(path);
    return contexts.add_image(img);
}