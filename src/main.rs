mod renderer;
mod simulation;

fn main() {
    use bevy::prelude::*;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((simulation::SimulationPlugin, renderer::RendererPlugin))
        .run();
}
