use bevy::prelude::*;
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
        // .add_plugin(WorldInspectorPlugin::new())
        .add_system(kill_player)
        .add_system(follow_cam)
        .add_system(player_collectables)
        .add_system(start_the_party)
        .add_system(party)
        .add_system(game_over_checker)
        .add_startup_system(setup_ui)
        .insert_resource(GameResources {
            party_material: StandardMaterial {
                base_color: Color::Rgba {
                    red: 0.,
                    green: 1.,
                    blue: 0.,
                    alpha: 1.,
                },
                ..default()
            },
        })
        // .register_inspectable::<Toggles>()
        .run();
}

#[derive(Component, Inspectable)]
struct Toggles {}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn_bundle(
                                TextBundle::from_section(
                                    "Collect All The Animals!",
                                    TextStyle {
                                        font: asset_server.load("FredokaOne-Regular.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                }),
                            );
                        });
                });
            // right vertical fill
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        ..default()
                    },
                    color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn_bundle(
                        TextBundle::from_section(
                            "Scrolling list",
                            TextStyle {
                                font: asset_server.load("FredokaOne-Regular.ttf"),
                                font_size: 25.,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            size: Size::new(Val::Undefined, Val::Px(25.)),
                            margin: UiRect {
                                left: Val::Auto,
                                right: Val::Auto,
                                ..default()
                            },
                            ..default()
                        }),
                    );
                    // List with hidden overflow
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_self: AlignSelf::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                                overflow: Overflow::Hidden,
                                ..default()
                            },
                            color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::ColumnReverse,
                                        flex_grow: 1.0,
                                        max_size: Size::new(Val::Undefined, Val::Undefined),
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent.spawn_bundle(
                                            TextBundle::from_section(
                                                format!("Item {i}"),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("FredokaOne-Regular.ttf"),
                                                    font_size: 20.,
                                                    color: Color::WHITE,
                                                },
                                            )
                                            .with_style(Style {
                                                flex_shrink: 0.,
                                                size: Size::new(Val::Undefined, Val::Px(20.)),
                                                margin: UiRect {
                                                    left: Val::Auto,
                                                    right: Val::Auto,
                                                    ..default()
                                                },
                                                ..default()
                                            }),
                                        );
                                    }
                                });
                        });
                });
            // absolute positioning
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(210.0),
                            bottom: Val::Px(10.0),
                            ..default()
                        },
                        border: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    color: Color::rgb(0.4, 0.4, 1.0).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            ..default()
                        },
                        color: Color::rgb(0.8, 0.8, 1.0).into(),
                        ..default()
                    });
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                ..default()
                            },
                            color: Color::rgb(1.0, 0.0, 0.0).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(20.0),
                                        bottom: Val::Px(20.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                color: Color::rgb(1.0, 0.3, 0.3).into(),
                                ..default()
                            });
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(40.0),
                                        bottom: Val::Px(40.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                color: Color::rgb(1.0, 0.5, 0.5).into(),
                                ..default()
                            });
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(60.0),
                                        bottom: Val::Px(60.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                color: Color::rgb(1.0, 0.7, 0.7).into(),
                                ..default()
                            });
                            // alpha test
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(80.0),
                                        bottom: Val::Px(80.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                color: Color::rgba(1.0, 0.9, 0.9, 0.4).into(),
                                ..default()
                            });
                        });
                });
            // bevy logo (flex center)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // bevy logo (image)
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(500.0), Val::Auto),
                            ..default()
                        },
                        image: asset_server.load("branding/bevy_logo_dark_big.png").into(),
                        ..default()
                    });
                });
        });
}

fn party(time: Res<Time>, mut party_zone: Query<(&mut Transform, &PartyZone)>) {
    for (mut transform, party_zone) in party_zone.iter_mut() {
        let displacement = (time.seconds_since_startup() * 7.).sin().powf(1.).abs() * 1.;
        // print!("Movin!\n {}", displacement);
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
        .insert(Friction {
            coefficient: 0.,
            ..default()
        })
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
                .insert(Friction {
                    coefficient: 0.,
                    ..default()
                })
                .insert(ColliderMassProperties::Density(0.01))
                .insert(GravityScale(4.));
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
struct PartyAnimal {}

struct GameResources {
    party_material: StandardMaterial,
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
    mut players: Query<(&Player, &mut Transform, &mut Velocity)>,
) {
    let mut player_count = players.iter().count();
    for collision in collisions.iter() {
        match collision {
            &CollisionEvent::Started(a, b, _) => {
                if [a, b].iter().any(|&entity| kill_wall.contains(entity)) {
                    for &entity in [a, b].iter() {
                        match players.get_mut(entity) {
                            Ok((player, mut transform, mut velocity)) => {
                                if player_count > 1 {
                                    commands.entity(entity).remove::<Player>();
                                    player_count -= 1;
                                }

                                transform.translation = player.spawn_position;
                                velocity.linvel = Vec3::ZERO;
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

fn start_the_party(
    mut commands: Commands,
    game_resources: Res<GameResources>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collisions: EventReader<CollisionEvent>,
    players: Query<&Children, With<Player>>,
    party_zones: Query<(), With<PartyZone>>,
    mut material_handles: Query<Entity, With<Handle<StandardMaterial>>>,
) {
    for collision in collisions.iter() {
        match collision {
            &CollisionEvent::Started(a, b, _) => {
                if [a, b].iter().any(|&entity| party_zones.contains(entity)) {
                    for &entity in [a, b].iter() {
                        match players.get(entity) {
                            Ok(children) => {
                                commands
                                    .entity(entity)
                                    .remove::<Player>()
                                    .insert(PartyAnimal {});
                                for &child in children.iter() {
                                    match material_handles.get_mut(child) {
                                        Ok(material_entity) => {
                                            let party_material = materials
                                                .add(game_resources.party_material.clone());
                                            commands.entity(material_entity).insert(party_material);
                                        }
                                        Err(_) => {}
                                    }
                                }
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

fn game_over_checker(
    players: Query<(), With<Player>>,
    party_animals: Query<(), With<PartyAnimal>>,
) {
    if players.iter().count() == 0 {
        print!("Game Over! Your score is {}", party_animals.iter().count());
    }
}

fn player_collectables(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    players: Query<&Player>,
    collectables: Query<
        &GlobalTransform,
        (With<Collectable>, Without<Player>, Without<PartyAnimal>),
    >,
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

    for (entity, _, _) in collectables.chain(bunnies) {
        commands
            .entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(Velocity { ..default() })
            .insert(Collider::ball(2.0))
            .insert(Restitution::coefficient(0.2))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(GravityScale(4.))
            .insert(Friction {
                coefficient: 0.,
                ..default()
            })
            // .insert(Ccd::enabled()) // Breaks with TriMesh :(
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
        let right_stick = Vec3 {
            x: axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
                .unwrap(),
            z: -axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
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
            flat_camera_rotation * (left_stick + right_stick)
        } else {
            left_stick + right_stick
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
