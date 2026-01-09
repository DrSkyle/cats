use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::population::{PopulationConfig, Creature};

#[derive(Event)]
pub struct MeteorStrikeEvent;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MeteorStrikeEvent>()
           .add_systems(Update, render_dashboard);
    }
}

fn render_dashboard(
    mut contexts: EguiContexts,
    mut pop_config: ResMut<PopulationConfig>,
    mut meteor_writer: EventWriter<MeteorStrikeEvent>,
    time: Res<Time>,
    // TODO: Query for graph data
) {
    egui::Window::new("God Mode Dashboard").show(contexts.ctx_mut(), |ui| {
        ui.collapsing("ℹ️ How to Play", |ui| {
            ui.label("• Right Click + Drag to Rotate Camera");
            ui.label("• Scroll to Zoom");
            ui.label("• Watch them evolve!");
            ui.label("• Mobile: Touch controls coming soon.");
        });
        ui.separator();

        ui.heading("Global Parameters");
        
        ui.add(egui::Slider::new(&mut pop_config.mutation_rate, 0.0..=1.0).text("Radiation Level"));
        
        if ui.button("☄️ Meteor Strike").clicked() {
            meteor_writer.send(MeteorStrikeEvent);
        }
        
        ui.separator();
        ui.heading("Statistics");
        ui.label(format!("Time: {:.2}", time.elapsed_seconds()));
        
        // Placeholder for Graph
        use egui_plot::{Line, Plot, PlotPoints};
        let sin: PlotPoints = (0..1000).map(|i| {
            let x = i as f64 * 0.01;
            [x, x.sin()]
        }).collect();
        let line = Line::new(sin);
        Plot::new("IQ Graph")
            .view_aspect(2.0)
            .show(ui, |plot_ui| plot_ui.line(line));
    });
}
