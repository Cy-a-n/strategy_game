use bevy::prelude::{With, World};
use bevy_egui::{
    egui::{ScrollArea, SidePanel},
    EguiContext,
};
use bevy_inspector_egui::bevy_inspector::ui_for_world;
use bevy_window::PrimaryWindow;

pub fn bevy_inspector_panel(world: &mut World) {
    // let mut ctx = world
    //     .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
    //     .get_single_mut(world)
    //     .unwrap()
    //     .clone();
    // SidePanel::right("bevy_inspector")
    //     .resizable(true)
    //     .show(ctx.get_mut(), |ui| {
    //         ScrollArea::both().show(ui, |ui| {
    //             ui_for_world(world, ui);
    //             ui.allocate_space(ui.available_size());
    //         });
    //     });
}
