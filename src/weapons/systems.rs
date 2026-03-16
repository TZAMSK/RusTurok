use super::bullets::DespawnAfter;
use crate::{
    animations::systems::{play_weapon_animation, AnimationPlayerLinked},
    camera::{
        components::{FirstLayerCamera, WeaponLayerCamera},
        renderlayers::{VIEW_MODEL_RENDER_LAYER, WORLD_RENDER_LAYER},
    },
    combat::DamageMessage,
    enemy::components::Enemy,
    player::components::Player,
    weapons::{
        components::{
            ads::ADS,
            animation::{GunAnimation, WeaponAnimationStance, WeaponAnimationState},
            attachments::{
                grip::Grip, mag::Mag, muzzle::Muzzle, optic::Optic, Attachment, AttachmentStats,
                Rarity,
            },
            bullet::{Bullet, BulletTracer},
            weapon::{weapon_type_from_str, Weapon},
        },
        data::database::WeaponDatabase,
        ressources::input::WeaponInput,
        util::apply_render_layers_to_children,
    },
};
use bevy::{camera::visibility::RenderLayers, color::palettes::tailwind, prelude::*};

struct RaycastHit {
    point: Vec3,
    entity: Option<Entity>,

    #[allow(dead_code)]
    distance: f32,
}

#[derive(Message)]
pub struct WeaponSpawnEvent {
    pub weapon_id: String,
}

pub fn spawn_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<Entity, With<Player>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut weapon_db: ResMut<WeaponDatabase>,
    mut spawn_event: MessageReader<WeaponSpawnEvent>,
    camera_query: Query<Entity, With<FirstLayerCamera>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    let Ok(cam_entity) = camera_query.single() else {
        return;
    };

    for event in spawn_event.read() {
        let id = &event.weapon_id;

        let mag = weapon_db.get_mag("stock_mag").unwrap().clone();
        let optic = weapon_db.get_optic("red_dot").unwrap().clone();
        weapon_db.get_weapon_mut(id).unwrap().attach_mag(mag);
        weapon_db.get_weapon_mut(id).unwrap().attach_optic(optic);

        let weapon_data = weapon_db.get_weapon(id).unwrap();

        let initial_weapon_state =
            WeaponAnimationState::define_state_by_stance(WeaponAnimationStance::Grounded);

        let ads_position = weapon_data.ads_position;

        let shooting_clip: Handle<AnimationClip> = asset_server.load(
            GltfAssetLabel::Animation(0)
                .from_asset(weapon_db.get_weapon(id).unwrap().assets.shooting.clone()),
        );

        let reloading_clip: Handle<AnimationClip> = asset_server.load(
            GltfAssetLabel::Animation(0)
                .from_asset(weapon_db.get_weapon(id).unwrap().assets.reload.clone()),
        );

        let mut graph = AnimationGraph::new();
        let shooting_node = graph.add_clip(shooting_clip, 1.0, graph.root);
        let reloading_node = graph.add_clip(reloading_clip, 1.0, graph.root);
        let graph_handle = graphs.add(graph);

        let optic = weapon_data.attachments.optic.as_ref().map(|o| Optic {
            stats: AttachmentStats {
                name: o.name.clone(),
                rarity: Rarity::Standard,
            },
            asset: asset_server.load(GltfAssetLabel::Scene(0).from_asset(o.asset.clone())),
            zoom: o.bonus_zoom,
        });

        let mag = weapon_data.attachments.mag.as_ref().map(|m| Mag {
            stats: AttachmentStats {
                name: m.name.clone(),
                rarity: Rarity::Standard,
            },
            asset: asset_server.load(GltfAssetLabel::Scene(0).from_asset(m.asset.clone())),
            bullets: m.bullets,
        });

        let muzzle = weapon_data.attachments.muzzle.as_ref().map(|m| Muzzle {
            stats: AttachmentStats {
                name: m.name.clone(),
                rarity: Rarity::Standard,
            },
            asset: asset_server.load(GltfAssetLabel::Scene(0).from_asset(m.asset.clone())),
            stability: m.bonus_stability,
        });

        let grip = weapon_data.attachments.grip.as_ref().map(|g| Grip {
            stats: AttachmentStats {
                name: g.name.clone(),
                rarity: Rarity::Standard,
            },
            handling: g.bonus_handling,
        });

        let attachments = Attachment::new(optic, mag, grip, muzzle);
        let weapon_type = weapon_type_from_str(&weapon_data.weapon_type);

        let weapon = Weapon::new(
            id.clone(),
            weapon_data.name.clone(),
            weapon_type,
            graph_handle,
            shooting_node,
            reloading_node,
            attachments,
            weapon_data,
        );

        let weapon_entity =
            commands
                .spawn((
                    SceneRoot(asset_server.load(
                        GltfAssetLabel::Scene(0).from_asset(weapon_data.assets.model.clone()),
                    )),
                    Transform::from_xyz(
                        initial_weapon_state.translation.x,
                        initial_weapon_state.translation.y,
                        initial_weapon_state.translation.z,
                    ),
                    RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                    weapon,
                    GunAnimation::default(),
                    initial_weapon_state,
                    ADS::new(initial_weapon_state.translation, ads_position),
                ))
                .observe(apply_render_layers_to_children)
                .with_children(|parent| {
                    parent.spawn((
                        Transform {
                            translation: Vec3::new(0.0, 0.04, 0.3),
                            ..default()
                        },
                        BulletTracer,
                        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                    ));
                })
                .id();

        commands.entity(cam_entity).add_child(weapon_entity);
    }
}

pub fn spawn_bullets(
    mut weapon_input: ResMut<WeaponInput>,
    mut commands: Commands,
    player_query: Query<(Entity, &Player)>,
    bullet_tracer_query: Query<&GlobalTransform, With<BulletTracer>>,
    mut weapon_query: Query<(&mut Weapon, &Children)>,
    camera_query: Query<&GlobalTransform, (With<Camera>, With<FirstLayerCamera>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    children_query: Query<&Children>,
    mut anim_players: Query<&mut AnimationPlayer, With<AnimationPlayerLinked>>,
    time: Res<Time>,
    enemy_query: Query<(Entity, &GlobalTransform), With<Enemy>>,
    mut damage_events: MessageWriter<DamageMessage>,
) {
    let Ok((player_entity, _player)) = player_query.single() else {
        return;
    };

    let Ok((mut weapon, children)) = weapon_query.single_mut() else {
        return;
    };

    weapon.fire_cooldown = (weapon.fire_cooldown - time.delta_secs()).max(0.0);

    if weapon.fire_cooldown != 0.0 {
        weapon_input.shoot_pressed = false;
    }

    if weapon_input.shoot_pressed
        && weapon.fire_cooldown <= 0.0
        && weapon.unique_trait.current_magazine_bullets > 0
    {
        let Ok(tracer_transform) = bullet_tracer_query.single() else {
            return;
        };
        let Ok(camera_transform) = camera_query.single() else {
            return;
        };

        play_weapon_animation(
            "Shooting",
            &weapon,
            children,
            &children_query,
            &mut anim_players,
        );

        let camera_direction = camera_transform.forward().normalize();
        let camera_start = camera_transform.translation();
        let weapon_start = tracer_transform.translation();

        let max_distance = 1000.0;

        let adjusted_direction = apply_aim_assist(
            camera_start,
            camera_direction,
            &enemy_query,
            weapon.cone_fogiveness(),
        );
        let hit = raycast_from_camera(camera_start, adjusted_direction, max_distance, &enemy_query);

        if let Some(enemy_entity) = hit.entity {
            damage_events.write(DamageMessage {
                target: enemy_entity,
                amount: 10.0,
                shooter: Some(player_entity),
            });
        }

        weapon.unique_trait.current_magazine_bullets -= 1;
        weapon.fire_cooldown = 60.0 / weapon.unique_trait.stats.rounds_per_minute;

        let weapon_to_hit = (hit.point - weapon_start).normalize();

        spawn_visual_tracer(
            &mut commands,
            &mut meshes,
            &mut materials,
            weapon_start,
            weapon_to_hit,
            hit.point,
            &time,
        );

        spawn_muzzle_flash(
            &mut commands,
            &mut meshes,
            &mut materials,
            &asset_server,
            weapon_start,
            camera_direction,
            &time,
        );

        spawn_impact_effect(
            &mut commands,
            &mut meshes,
            &mut materials,
            hit.point,
            hit.entity.is_some(),
            &time,
        );
    }
}

fn apply_aim_assist(
    start: Vec3,
    original_direction: Vec3,
    enemy_query: &Query<(Entity, &GlobalTransform), With<Enemy>>,
    cone_forgiveness: (f32, f32),
) -> Vec3 {
    let mut best_target: Option<(Vec3, f32)> = None;
    let (cone, bend) = cone_forgiveness;

    for (_, transform) in enemy_query.iter() {
        let enemy_pos = transform.translation();
        let to_enemy = (enemy_pos - start).normalize();
        let angle = original_direction.angle_between(to_enemy);

        if angle < cone {
            let distance = start.distance(enemy_pos);
            if best_target.is_none() || distance < best_target.unwrap().1 {
                best_target = Some((to_enemy, distance));
            }
        }
    }

    if let Some((target_dir, _)) = best_target {
        original_direction.lerp(target_dir, bend).normalize()
    } else {
        original_direction
    }
}

fn raycast_from_camera(
    start: Vec3,
    direction: Vec3,
    max_distance: f32,
    enemy_query: &Query<(Entity, &GlobalTransform), With<Enemy>>,
) -> RaycastHit {
    let mut closest_hit: Option<(Entity, Vec3, f32)> = None;

    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation();
        let enemy_radius = 0.5;

        let to_enemy = enemy_pos - start;
        let t = to_enemy.dot(direction);

        if t > 0.0 && t < max_distance {
            let closest_point_on_ray = start + direction * t;
            let distance_to_enemy = closest_point_on_ray.distance(enemy_pos);

            if distance_to_enemy < enemy_radius {
                let hit_distance = t
                    - (enemy_radius * enemy_radius - distance_to_enemy * distance_to_enemy)
                        .sqrt()
                        .max(0.0);
                let hit_point = start + direction * hit_distance;

                if closest_hit.is_none() || hit_distance < closest_hit.unwrap().2 {
                    closest_hit = Some((enemy_entity, hit_point, hit_distance));
                }
            }
        }
    }

    match closest_hit {
        Some((entity, point, distance)) => RaycastHit {
            point,
            entity: Some(entity),
            distance,
        },
        None => RaycastHit {
            point: start + direction * max_distance,
            entity: None,
            distance: max_distance,
        },
    }
}

fn spawn_visual_tracer(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    start: Vec3,
    direction: Vec3,
    hit_point: Vec3,
    time: &Res<Time>,
) {
    let distance = start.distance(hit_point);
    let tracer_mesh = meshes.add(Cylinder::new(0.01, distance));
    let tracer_material = materials.add(StandardMaterial {
        base_color: Color::from(tailwind::YELLOW_500),
        emissive: Color::from(tailwind::YELLOW_500).into(),
        ..default()
    });

    let mid_point = (start + hit_point) / 2.0;

    commands.spawn((
        Mesh3d(tracer_mesh),
        MeshMaterial3d(tracer_material),
        Transform {
            translation: mid_point,
            rotation: Quat::from_rotation_arc(Vec3::Y, direction),
            ..default()
        },
        Bullet,
        RenderLayers::layer(WORLD_RENDER_LAYER),
        DespawnAfter(time.elapsed_secs() + 0.05),
    ));
}

fn spawn_muzzle_flash(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    direction: Vec3,
    time: &Res<Time>,
) {
    for i in 1..=2 {
        let rotation_axis: Vec3 = match i {
            1 => Vec3::X,
            2 => Vec3::Z,
            _ => Vec3::Y,
        };

        let flash_mesh = meshes.add(Plane3d::default().mesh().size(0.33, 0.33));
        let flash_material = materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("models/muzzle_flash.png")),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            emissive: Color::from(tailwind::YELLOW_500).into(),
            ..default()
        });

        commands.spawn((
            Mesh3d(flash_mesh.clone()),
            MeshMaterial3d(flash_material),
            Transform {
                translation: position,
                rotation: Quat::from_rotation_arc(rotation_axis, direction),
                ..default()
            },
            RenderLayers::layer(WORLD_RENDER_LAYER),
            DespawnAfter(time.elapsed_secs() + 0.05),
        ));
    }
}

fn spawn_impact_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    hit_enemy: bool,
    time: &Res<Time>,
) {
    let color = if hit_enemy {
        tailwind::RED_500
    } else {
        tailwind::GRAY_500
    };

    let impact_mesh = meshes.add(Sphere::new(0.1).mesh().uv(32, 18));
    let impact_material = materials.add(StandardMaterial {
        base_color: Color::from(color),
        emissive: Color::from(color).into(),
        ..default()
    });

    commands.spawn((
        Mesh3d(impact_mesh),
        MeshMaterial3d(impact_material),
        Transform::from_translation(position),
        RenderLayers::layer(WORLD_RENDER_LAYER),
        DespawnAfter(time.elapsed_secs() + 0.1),
    ));
}
