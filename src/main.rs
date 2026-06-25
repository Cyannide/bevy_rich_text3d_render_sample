use bevy::{camera::Hdr, post_process::bloom::Bloom, prelude::*};
use bevy_rich_text3d::{LoadFonts, Text3d, Text3dPlugin, Text3dStyling, TextAtlas};

fn main() {
    let mut app = App::new();

    #[cfg(not(target_arch = "wasm32"))]
    {
        app.insert_resource(LoadFonts {
            font_paths: vec!["assets/fonts/FiraCode-Light.ttf".into()],
            ..Default::default()
        });
    }
    #[cfg(target_arch = "wasm32")]
    {
        app.insert_resource(LoadFonts {
            font_embedded: vec![include_bytes!("../../assets/fonts/FiraCode-Light.ttf")],
            ..Default::default()
        });
    }
    app.add_plugins(DefaultPlugins)
        .add_plugins(Text3dPlugin {
            load_system_fonts: false,
            //placeholder_glyph_origin: '\u{E000}',
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)).looking_at(Vec3::default(), Vec3::Y),
        Camera {
            clear_color: Color::BLACK.into(),
            ..default()
        },
        Hdr,
        Bloom::NATURAL,
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 3.2, 0.1))),
        Transform::from_xyz(0.0, -0.5, 0.0),
        Rotates,
    ));

    let mat = materials.add(StandardMaterial {
        base_color_texture: Some(TextAtlas::DEFAULT_IMAGE.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let text = Text3d::new("1234567890 there seems to be something wrong in the 3D matrix...");
    //let text = Text3d::new("1234567890");
    //let text = Text3d::new("0123456789abcdefghijlkmnopqrstuvwxyzöä!ABCDEFGHIJLKMNOPQRSTUVWXYZÖÄ");
    commands.spawn((
        text,
        Text3dStyling {
            size: 32.,
            //stroke: NonZero::new(10),
            color: Srgba::new(2.8, 0.2, 0.1, 1.),
            stroke_color: Srgba::BLACK,
            world_scale: Some(Vec2::splat(0.075)),
            layer_offset: 0.001,
            ..Default::default()
        },
        Mesh3d::default(),
        MeshMaterial3d(mat.clone()),
        Transform {
            translation: Vec3::new(0., 1., 1.),
            rotation: Quat::from_axis_angle(Vec3::Y, 0.),
            scale: Vec3::ONE,
        },
    ));

    // light
    commands.spawn(DirectionalLight {
        illuminance: 1_000.,
        ..default()
    });
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the x and y axis
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_secs());
        transform.rotate_z(0.15 * time.delta_secs());
    }
}
