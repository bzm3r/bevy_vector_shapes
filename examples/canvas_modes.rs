// Demonstrates the various canvas modes
// Press Space to request a redraw and M to cycle through the various modes

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin::default())
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_startup_system(setup)
        .add_system(draw_shapes)
        .add_system(update_canvas)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let config = CanvasConfig::new(1024, 1024);
    commands.spawn_canvas(images.as_mut(), config);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 16.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_canvas(keys: Res<Input<KeyCode>>, mut canvas: Query<&mut Canvas>) {
    let mut canvas = canvas.single_mut();

    if keys.just_pressed(KeyCode::Space) {
        canvas.redraw();
    }

    if keys.just_pressed(KeyCode::M) {
        canvas.mode = match canvas.mode {
            CanvasMode::Continuous => CanvasMode::Persistent,
            CanvasMode::Persistent => CanvasMode::OnDemand,
            CanvasMode::OnDemand => CanvasMode::Continuous,
        }
    }
}

fn draw_shapes(time: Res<Time>, mut painter: ShapePainter, canvas: Query<(Entity, &Canvas)>) {
    let (canvas_e, canvas) = canvas.single();
    painter.image(canvas.image.clone(), Vec2::splat(20.));

    painter.set_canvas(canvas_e);
    painter.hollow = true;
    painter.thickness = 6.0;
    painter.color = Color::CRIMSON;
    painter.translate(Vec3::Y * time.elapsed_seconds().sin() * 256.0);
    painter.circle(48.0);

    painter.reset();
}
