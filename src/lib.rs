use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_egui::EguiPlugin;
use wasm_bindgen::prelude::*;

mod genome;
mod brain;
mod physic_sim;
mod world;
mod audio;
mod visuals;
mod population;
mod ui;
mod camera;

#[wasm_bindgen]
pub fn run() {
    App::new()
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Explicitly target a canvas with ID "bevy" to ensure correct attaching
                canvas: Some("#bevy".into()), 
                // Fill the parent (body)
                fit_canvas_to_parent: true,
                // Handle events
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EguiPlugin)
        .add_plugins(physic_sim::PhysicSimPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(audio::AudioPlugin)
        .add_plugins(visuals::VisualsPlugin)
        .add_plugins(population::PopulationPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, brain::brain_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ambient Light (Global Illumination ensuring nothing is pitch black)
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
    });

    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        Collider::cuboid(25.0, 0.1, 25.0),
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        GlobalTransform::default(),
        Visibility::default(),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        camera::PanOrbitCamera::default(),
    ));
}
