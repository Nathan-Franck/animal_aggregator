use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .add_state(AppState::MainMenu)
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_game_scene))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_ui))
        .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(setup_ui))
        .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(any_key_to_restart))
        .insert_resource(WindowDescriptor {
            title: "Combine".to_string(),
            width: 800.,
            height: 600.,
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
        .add_system(gameover_checker)
        .add_startup_system(setup_ui)
        .insert_resource(GameResources {
            scene_entity: None,
            party_material: StandardMaterial {
                base_color: Color::Rgba {
                    red: 0.,
                    green: 1.,
                    blue: 0.,
                    alpha: 1.,
                },
                ..default()
            },
            ui_node: None,
            despawned_party_animals_count: 0,
        })
        .run();
}

fn any_key_to_restart(
    mut commands: Commands,
    mut game_resources: ResMut<GameResources>,
    mut app_state: ResMut<State<AppState>>,
    buttons: Res<Input<GamepadButton>>,
    keys: Res<Input<KeyCode>>,
) {
    if buttons.get_just_pressed().count() > 0 || keys.get_just_pressed().count() > 0 {
        match game_resources.scene_entity {
            Some(entity) => {
                commands.entity(entity).despawn_recursive();
            }
            _ => {}
        }
        game_resources.despawned_party_animals_count = 0;
        app_state.set(AppState::MainMenu).unwrap();
    }
}

fn setup_ui(
    app_state: ResMut<State<AppState>>,
    mut game_resources: ResMut<GameResources>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    party_animals: Query<(), With<PartyAnimal>>,
    collectables: Query<(), With<Collectable>>,
) {
    match game_resources.ui_node {
        Some(entity) => {
            commands.entity(entity).despawn_recursive();
        }
        None => {}
    }
    game_resources.ui_node = Some(commands
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
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        align_items: match app_state.current() {
                            &AppState::InGame => AlignItems::FlexStart,
                            _ => AlignItems::Center,
                        },
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: Color::rgba(0., 0., 0., 0.).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(
                        TextBundle::from_section(
                            match app_state.current() {
                                &AppState::InGame => {
                                    "Combine your animal herd and take them to the exit!".to_string()
                                }
                                &AppState::GameOver => {
                                    let party_total = party_animals.iter().count() as i32 + game_resources.despawned_party_animals_count;
                                    let potential_total = collectables.iter().count() as i32 + game_resources.despawned_party_animals_count;
                                    if party_total == potential_total - 1 { // 1 animal missing? Close enough :/
                                        "WOW! You got all the animals! You win the game for sure! Press any key/button to try again :)".to_string()
                                    } else {
                                        format!(
                                            "Congrats! You got {} out of a possible {} animals to the exit! Press any key/button to try again :)",
                                            party_total,
                                            potential_total)
                                    }
                                        
                                },
                                &AppState::MainMenu => "Press any button to start!".to_string(),
                            },
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
                    // });
                });
        })
        .id());
}

fn party(time: Res<Time>, mut party_zone: Query<(&mut Transform, &PartyZone)>) {
    for (mut transform, party_zone) in party_zone.iter_mut() {
        let displacement = (time.seconds_since_startup() * 7.).sin().powf(1.).abs() * 1.;
        transform.translation = party_zone.bob_position + Vec3::Y * displacement as f32;
    }
}

fn follow_cam(
    mut commands: Commands,
    player: Query<(Entity, &GlobalTransform, &Transform), With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    for mut camera_global_transform in camera.iter_mut() {
        let average_player_position = player.iter().fold(Vec3::ZERO, |sum, (_, transform, _)| {
            sum + transform.translation()
        }) / player.iter().count() as f32;
        camera_global_transform.translation =
            average_player_position + camera_global_transform.rotation * Vec3::Z * 50.;
        for (entity, global_transform, _) in player.iter() {
            if global_transform
                .translation()
                .distance(average_player_position)
                > 20.
            {
                commands.entity(entity).remove::<Player>();
            }
        }
    }
}

fn setup_physics(
    mut commands: Commands,
) {
    commands
        .spawn()
        .insert(Collider::cuboid(1000.0, 0.1, 1000.0))
        .insert(KillWall)
        .insert(Friction {
            coefficient: 0.,
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)));
}

const CHARACTER_SPEED: f32 = 12.;

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
    scene_entity: Option<Entity>,
    ui_node: Option<Entity>,
    despawned_party_animals_count: i32,
}

#[derive(Component)]
struct Collectable {}

#[derive(Component)]
struct KillWall;

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
    mut game_resources: ResMut<GameResources>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collisions: EventReader<CollisionEvent>,
    players: Query<&Children, With<Player>>,
    party_zones: Query<(), With<PartyZone>>,
    mut material_handles: Query<Entity, With<Handle<StandardMaterial>>>,
    party_animals: Query<Entity, With<PartyAnimal>>,
) {
    let mut party_count = party_animals.iter().count();
    let mut party_iter = party_animals.iter();
    for collision in collisions.iter() {
        match collision {
            &CollisionEvent::Started(a, b, _) => {
                if [a, b].iter().any(|&entity| party_zones.contains(entity)) {
                    for &entity in [a, b].iter() {
                        match players.get(entity) {
                            Ok(children) => {
                                if party_count > 10 {
                                    match party_iter.next() {
                                        Some(entity) => {
                                            commands.entity(entity).despawn_recursive();
                                        }
                                        _ => {}
                                    }
                                    party_count -= 1;
                                    print!(
                                        "Despawned count {}",
                                        game_resources.despawned_party_animals_count
                                    );
                                    game_resources.despawned_party_animals_count += 1;
                                }
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

fn gameover_checker(
    mut app_state: ResMut<State<AppState>>,
    game_resources: Res<GameResources>,
    players: Query<(), With<Player>>,
    party_animals: Query<(), With<PartyAnimal>>,
) {
    if app_state.current() == &AppState::InGame {
        if players.iter().count() == 0 {
            let score_total =
                party_animals.iter().count() as i32 + game_resources.despawned_party_animals_count;
            print!("Game Over! Your score is {}", score_total);
            app_state.set(AppState::GameOver).unwrap();
        }
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

fn connect_from_scene(
    mut app_state: ResMut<State<AppState>>,
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
        app_state.set(AppState::InGame).unwrap();
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
    keys: Res<Input<KeyCode>>,
    camera: Query<&GlobalTransform, With<Camera>>,
    mut player: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    let mut raw_input = Vec3 {
        z: if keys.pressed(KeyCode::Up) { -1. } else { 0. }
            + if keys.pressed(KeyCode::Down) { 1. } else { 0. },
        x: if keys.pressed(KeyCode::Left) { -1. } else { 0. }
            + if keys.pressed(KeyCode::Right) { 1. } else { 0. },
        ..default()
    };
    raw_input += Vec3 {
        z: if keys.pressed(KeyCode::W) { -1. } else { 0. }
            + if keys.pressed(KeyCode::S) { 1. } else { 0. },
        x: if keys.pressed(KeyCode::A) { -1. } else { 0. }
            + if keys.pressed(KeyCode::D) { 1. } else { 0. },
        ..default()
    };
    for gamepad in gamepads.iter().cloned() {
        raw_input += Vec3 {
            x: axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap(),
            z: -axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
                .unwrap(),
            ..default()
        };
        raw_input += Vec3 {
            x: axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
                .unwrap(),
            z: -axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
                .unwrap(),
            ..default()
        };
    }

    if raw_input.length() > 0. {
        raw_input = raw_input.normalize() * (raw_input.length() - 0.05).max(0.) / 0.95;
    }

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
        flat_camera_rotation * raw_input
    } else {
       raw_input 
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

fn setup_game_scene(
    mut game_resources: ResMut<GameResources>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    // load scene
    game_resources.scene_entity = Some(
        commands
            .spawn_bundle(SceneBundle {
                scene: asset_server.load("animals.gltf#Scene0"),
                ..default()
            })
            .id(),
    );
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    asset_server.watch_for_changes().unwrap();

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
