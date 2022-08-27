use bevy::{ecs::component, prelude::*};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;
use rand::prelude::random;
use std::collections::HashMap;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Combine".to_string(),
            width: 805.,
            height: 430.,
            position: WindowPosition::At(Vec2 { x: 67., y: 603. }),
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(connect_from_scene)
        .add_system(resize_notificator)
        .add_system(gamepad_system)
        .add_startup_system(setup)
        .add_startup_system(setup_physics)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(kill_player)
        .add_system(follow_cam)
        .add_system(player_collectables)
        .add_system(party)
        .register_inspectable::<Toggles>()
        .run();
}

#[derive(Component, Inspectable)]
struct Toggles {}

fn party(time: Res<Time>, mut party_zone: Query<(&mut Transform, &PartyZone)>) {
    for (mut transform, party_zone) in party_zone.iter_mut() {
        let displacement = (time.seconds_since_startup() * 7.).sin().powf(1.).abs() * 1.;
        print!("Movin!\n {}", displacement);
        transform.translation = party_zone.bob_position + Vec3::Y * displacement as f32;
    }
}

fn follow_cam(
    // toggles: Query<&Toggles>,
    player: Query<&GlobalTransform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    for mut camera_global_transform in camera.iter_mut() {
        let average_player_position = player
            .iter()
            .fold(Vec3::ZERO, |sum, transform| sum + transform.translation())
            / player.iter().count() as f32;
        camera_global_transform.translation =
            average_player_position + camera_global_transform.rotation * Vec3::Z * 50.;
    }
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(1000.0, 0.1, 1000.0))
        .insert(KillWall)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)));

    // add entities to the world
    for y in -2..=2 {
        for x in -5..=5 {
            // sphere
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.5,
                        subdivisions: 32,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::hsl(random::<f32>() * 256., 1., 0.5),
                        metallic: 0.,
                        perceptual_roughness: 0.,
                        ..default()
                    }),
                    transform: Transform::from_xyz(x as f32, y as f32 + 0.5, 0.0),
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(0.5))
                .insert(Restitution::coefficient(0.7))
                .insert(ColliderMassProperties::Density(0.01))
                .insert(GravityScale(10.));
        }
    }
}

const CHARACTER_SPEED: f32 = 12.;

type Animations = HashMap<AnimationID, Handle<AnimationClip>>;

#[derive(Component)]
struct Player {
    spawn_position: Vec3,
}

#[derive(Component)]
struct PartyZone {
    bob_position: Vec3,
}

#[derive(Component)]
struct Collectable {}

#[derive(Component)]
struct KillWall;

#[derive(Clone, Eq, PartialEq, Hash)]
enum AnimationID {
    Idle,
    Walk,
}

fn kill_player(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    kill_wall: Query<&KillWall>,
    mut players: Query<(&Player, &mut Transform)>,
) {
    let mut player_count = players.iter().count();
    for collision in collisions.iter() {
        match collision {
            &CollisionEvent::Started(a, b, _) => {
                if [a, b].iter().any(|&entity| kill_wall.contains(entity)) {
                    for &entity in [a, b].iter() {
                        match players.get_mut(entity) {
                            Ok((player, mut transform)) => {
                                if player_count > 1 {
                                    commands.entity(entity).remove::<Player>();
                                    player_count -= 1;
                                }

                                transform.translation = player.spawn_position;
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn player_collectables(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    players: Query<&Player>,
    collectables: Query<&GlobalTransform, (With<Collectable>, Without<Player>)>,
) {
    for collision in collisions.iter() {
        match collision {
            &CollisionEvent::Started(a, b, _) => {
                if [a, b].iter().any(|&entity| players.contains(entity)) {
                    for &entity in [a, b].iter() {
                        match collectables.get(entity) {
                            Ok(&transform) => {
                                commands.entity(entity).insert(Player {
                                    spawn_position: transform.translation(),
                                });
                            }
                            Err(_) => {}
                        };
                    }
                }
            }
            _ => {}
        }
    }
}

// fn follow_cam

fn connect_from_scene(
    named_entities: Query<(Entity, &Name, &Transform), Added<Name>>,
    named_entities_with_children: Query<(Entity, &Name, &Children, &Transform), Added<Name>>,
    meshes: Query<&Handle<Mesh>>,
    mesh_assets: Res<Assets<Mesh>>,
    mut commands: Commands,
) {
    let bunnies = named_entities
        .iter()
        .filter(|&(_, name, _)| name.contains("Player"))
        .collect::<Vec<_>>();
    let collectables = named_entities
        .iter()
        .filter(|&(_, name, _)| name.contains("Collectable"));
    for (entity, _, transform) in bunnies.clone() {
        commands.entity(entity).insert(Player {
            spawn_position: transform.translation,
        });
    }

    for (entity, name, _) in collectables.chain(bunnies) {
        println!("{}", name);
        commands
            .entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(Velocity { ..default() })
            .insert(Collider::cylinder(1.0, 2.0))
            .insert(Restitution::coefficient(0.2))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(GravityScale(10.))
            .insert(Collectable {});
    }
    let level = named_entities_with_children
        .iter()
        .filter(|&(_, name, _, _)| name.eq(&Name::new("Level")));
    let party_zone = named_entities_with_children
        .iter()
        .filter(|&(_, name, _, _)| name.eq(&Name::new("PartyZone")));
    let goal = named_entities_with_children
        .iter()
        .filter(|&(_, name, _, _)| name.eq(&Name::new("Goal")));
    for (entity, name, children, _) in level {
        let (child_mesh_entities, _): (Vec<_>, Vec<_>) = children
            .iter()
            .map(|&child| meshes.get(child))
            .partition(Result::is_ok);
        let child_mesh_entities: Vec<_> = child_mesh_entities
            .into_iter()
            .map(Result::unwrap)
            .collect();
        for mesh in child_mesh_entities {
            commands.entity(entity).insert(
                Collider::from_bevy_mesh(
                    mesh_assets.get(mesh).unwrap(),
                    &ComputedColliderShape::TriMesh,
                )
                .unwrap(),
            );
        }
        println!("Level Geometry Found: {}", name);
    }
    for (entity, name, children, _) in goal {
        let (child_mesh_entities, _): (Vec<_>, Vec<_>) = children
            .iter()
            .map(|&child| meshes.get(child))
            .partition(Result::is_ok);
        let child_mesh_entities: Vec<_> = child_mesh_entities
            .into_iter()
            .map(Result::unwrap)
            .collect();
        for mesh in child_mesh_entities {
            commands.entity(entity).insert(
                Collider::from_bevy_mesh(
                    mesh_assets.get(mesh).unwrap(),
                    &ComputedColliderShape::TriMesh,
                )
                .unwrap(),
            );
        }
        println!("Goal Geometry Found: {}", name);
    }
    for (entity, name, _, transform) in party_zone {
        commands
            .entity(entity)
            .insert(Collider::cuboid(1., 1., 1.))
            .insert(PartyZone {
                bob_position: transform.translation,
            })
            .insert(RigidBody::KinematicPositionBased);
            // .insert(Ccd::enabled());
        println!("Party Zone Geometry Found: {}", name);
    }
}

fn gamepad_system(
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    camera: Query<&GlobalTransform, With<Camera>>,
    mut player: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    for gamepad in gamepads.iter().cloned() {
        let left_stick = Vec3 {
            x: axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap(),
            z: -axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
                .unwrap(),
            ..default()
        };

        let camera_relative_input = if let Ok(camera_transform) = camera.get_single() {
            let (_, camera_rotation, _) = camera_transform.to_scale_rotation_translation();
            let flat_camera_rotation = Quat::from_axis_angle(
                Vec3::Y,
                ((camera_rotation * Vec3::Z)
                    * Vec3 {
                        x: 1.,
                        y: 0.,
                        z: 1.,
                    })
                .angle_between(Vec3::Z),
            );
            flat_camera_rotation * left_stick
        } else {
            left_stick
        };

        for (mut velocity, mut transform) in player.iter_mut() {
            velocity.linvel = Vec3 {
                x: camera_relative_input.x * CHARACTER_SPEED,
                z: camera_relative_input.z * CHARACTER_SPEED,
                ..velocity.linvel
            };
            if camera_relative_input.length() > 0.25 {
                transform.rotation = Quat::from_axis_angle(
                    Vec3::Y,
                    3.14 / 2.
                        + Vec2 {
                            x: camera_relative_input.x,
                            y: camera_relative_input.z,
                        }
                        .angle_between(Vec2::X),
                )
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(Toggles {})
        .insert(Name::new("Toggles"));
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
