use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    asset_server.watch_for_changes().unwrap();

    let animal_scene = asset_server.load("animals.gltf#Scene0");

    // mesh
    commands.spawn_bundle(SceneBundle {
        scene: animal_scene,
        ..default()
    });

    // add entities to the world
    for y in -2..=2 {
        for x in -5..=5 {
            let x01 = (x + 5) as f32 / 10.0;
            let y01 = (y + 2) as f32 / 4.0;
            let kitty_mesh = asset_server.load("animals.gltf#Mesh0/Primitive0");

            // kitty mesh
            commands.spawn_bundle(PbrBundle {
                mesh: kitty_mesh,
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("ffd891").unwrap(),
                    // vary key PBR parameters on a grid of spheres to show the effect
                    metallic: y01,
                    perceptual_roughness: x01,
                    ..default()
                }),
                transform: Transform {
                    translation: Vec3 {
                        x: x as f32,
                        y: y as f32 + 0.5,
                        z: 0.0,
                    },
                    rotation: Quat::from_axis_angle(
                        Vec3 {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        5.0,
                    ),
                    scale: Vec3 {
                        x: 0.25,
                        y: 0.25,
                        z: 0.25,
                    },
                },
                ..default()
            });
        }
    }
    // // unlit sphere
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Icosphere {
    //         radius: 0.45,
    //         subdivisions: 32,
    //     })),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::hex("ffd891").unwrap(),
    //         // vary key PBR parameters on a grid of spheres to show the effect
    //         unlit: true,
    //         ..default()
    //     }),
    //     transform: Transform::from_xyz(-5.0, -2.5, 0.0),
    //     ..default()
    // });
    // // light
    // commands.spawn_bundle(PointLightBundle {
    //     transform: Transform::from_xyz(50.0, 50.0, 50.0),
    //     point_light: PointLight {
    //         intensity: 600000.,
    //         range: 100.,
    //         ..default()
    //     },
    //     ..default()
    // });
    // // camera
    // commands.spawn_bundle(Camera3dBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::default(), Vec3::Y),
    //     projection: OrthographicProjection {
    //         scale: 0.01,
    //         ..default()
    //     }
    //     .into(),
    //     ..default()
    // });
}
