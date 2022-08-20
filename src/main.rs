use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Combine".to_string(),
            width: 805.,
            height: 484.,
            position: WindowPosition::At(Vec2 { x: 1106., y: 516. }),
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(resize_notificator)
        .add_system(play_on_load)
        .add_startup_system(setup)
        .run();
}

struct Animations {
    walk: Handle<AnimationClip>,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    asset_server.watch_for_changes().unwrap();

    // load scene
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("animals.gltf#Scene0"),
        ..default()
    });

    // load animations
    commands.insert_resource(Animations {
        walk: asset_server.load("animals.gltf#Animation0"),
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
}

fn resize_notificator(
    resize_event: Res<Events<bevy::window::WindowResized>>,
    move_event: Res<Events<bevy::window::WindowMoved>>,
) {
    for e in resize_event.get_reader().iter(&resize_event) {
        println!("width: {} height: {}", e.width, e.height);
    }
    for e in move_event.get_reader().iter(&move_event) {
        println!("x: {} y: {}", e.position.x, e.position.y);
    }
}

fn play_on_load(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in players.iter_mut() {
        player.play(animations.walk.clone()).repeat();
    }
}
