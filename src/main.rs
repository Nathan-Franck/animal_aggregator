use bevy_inspector_egui::WorldInspectorPlugin;
use std::collections::HashMap;

use bevy::{ecs::*, prelude::*};

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
        .add_system(connect_from_scene)
        .add_system(resize_notificator)
        .add_system(play_on_load)
        .add_system(gamepad_system)
        .add_startup_system(setup)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

// struct Animations {
//     idle: Handle<AnimationClip>,
//     walk: Handle<AnimationClip>,
// }

type Animations = HashMap<AnimationID, Handle<AnimationClip>>;

#[derive(Component)]
struct Player;

#[derive(Clone, Eq, PartialEq, Hash)]
enum AnimationID {
    Idle,
    Walk,
}

fn connect_from_scene(
    mut commands: Commands,
    element: Query<(Entity, &Name), (With<Transform>, Added<Name>)>,
    // names: Query<(Entity, &Name)>,
) {
    for (entity, name) in element.iter() {
        println!("{}", name);
    }
    let bunnies = element
        .iter()
        .filter(|&(_, name)| name.eq(&Name::new("Puppy")));
    for (entity, name) in bunnies {
        println!("{}", name);
        commands.entity(entity).insert(Player);
    }
}

fn gamepad_system(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut player: Query<&mut Transform, With<Player>>,
    camera: Query<&Parent, With<Camera3d>>,
    global_transforms: Query<&GlobalTransform>,
) {
    for gamepad in gamepads.iter().cloned() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            info!("{:?} just pressed South", gamepad);
        } else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South))
        {
            info!("{:?} just released South", gamepad);
        }

        let right_trigger = button_axes
            .get(GamepadButton::new(
                gamepad,
                GamepadButtonType::RightTrigger2,
            ))
            .unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
        let left_stick = Vec3 {
            x: axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap(),
            z: axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
                .unwrap(),
            ..default()
        };

        let camera_relative_input = if let Ok(camera_root) = camera.get_single() {
            let camera_transform = global_transforms.get(camera_root.get()).unwrap();
            let (_, camera_rotation, _) = camera_transform.to_scale_rotation_translation();
            camera_rotation * left_stick
        } else {
            left_stick
        };

        for mut transform in player.iter_mut() {
            let mut translation = transform.translation;
            translation += camera_relative_input;

            // TEMP limit on position to keep player in-frame
            if translation.length() > 10. {
                translation *= 10. / translation.length();
            }
            transform.translation = translation;
        }
    }
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
    commands.insert_resource(Animations::from_iter([
        (
            AnimationID::Idle,
            asset_server.load("animals.gltf#Animation0") as Handle<AnimationClip>,
        ),
        (
            AnimationID::Walk,
            asset_server.load("animals.gltf#Animation1") as Handle<AnimationClip>,
        ),
    ]));

    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform {
            rotation: Quat::from_euler(EulerRot::XYZ, -45., 0., 0.),
            ..Default::default()
        },
        directional_light: DirectionalLight {
            // shadows_enabled: true,
            // shadow_projection: OrthographicProjection {
            //     bottom: -8.,
            //     top: 8.,
            //     left: -8.,
            //     right: 8.,
            //     near: -80.,
            //     far: 80.,
            //     ..Default::default()
            // },
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: -10.,
                z: 0.,
            },
            rotation: Quat::from_euler(EulerRot::XYZ, 45., 0., 0.),
            ..Default::default()
        },
        point_light: PointLight {
            color: Color::Rgba {
                red: 0.,
                green: 1.,
                blue: 0.,
                alpha: 1.,
            },
            range: 50.,
            radius: 10.,
            intensity: 3000.,
            ..Default::default()
        },
        ..Default::default()
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
    mut animations: ResMut<Animations>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    // HACK - offset animations to start at 0, requires animations to have a keyframe at the 0th frame of their "action" (Blender term)
    for (animation_id, animation_handle) in animations.clone().iter() {
        if let Some(animation_clip) = animation_clips.get_mut(&animation_handle) {
            let existing_animation_clip = animation_clip.clone();
            let curves_map = existing_animation_clip.curves();
            let mut new_animation_clip = AnimationClip::default();
            for (path, curves) in curves_map.iter() {
                for curve in curves.iter() {
                    if let Some(first_time) = curve.keyframe_timestamps.first() {
                        new_animation_clip.add_curve_to_path(
                            path.clone(),
                            VariableCurve {
                                keyframe_timestamps: curve
                                    .keyframe_timestamps
                                    .iter()
                                    .map(|timestamp| timestamp - first_time)
                                    .collect(),
                                keyframes: curve.keyframes.clone(),
                            },
                        );
                    }
                }
            }
            animations.insert(
                animation_id.clone(),
                animation_clips.set(animation_handle, new_animation_clip),
            );
        }
    }

    // start playing animation
    for mut player in players.iter_mut() {
        player.play(animations[&AnimationID::Walk].clone()).repeat();
    }
}
