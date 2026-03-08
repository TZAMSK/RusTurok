use crate::weapons::components::weapon::Weapon;
use bevy::prelude::*;

#[derive(Component)]
pub struct WeaponSocketsReady;

pub fn spawn_attachment_on_sockets(
    mut commands: Commands,
    weapon_query: Query<(Entity, &Weapon), Without<WeaponSocketsReady>>,
    named_query: Query<(Entity, &Name)>,
    children_query: Query<&Children>,
) {
    for (weapon_entity, weapon) in weapon_query.iter() {
        let descendants = iter_descendants(&children_query, weapon_entity);
        if descendants.is_empty() {
            continue;
        }

        let gun_rig_entity = descendants.iter().find(|&&e| {
            named_query
                .get(e)
                .map(|(_, n)| n.as_str() == "Gun_Rig")
                .unwrap_or(false)
        });

        let Some(&gun_rig) = gun_rig_entity else {
            continue;
        };

        let rig_descendants = iter_descendants(&children_query, gun_rig);
        if rig_descendants.is_empty() {
            continue;
        }

        let mut found_mag = false;
        let mut found_optic = false;
        let mut found_muzzle = false;

        for child in rig_descendants {
            let Ok((entity, name)) = named_query.get(child) else {
                continue;
            };

            if name.as_str() == "mag_socket" && !found_mag {
                let mag_asset = weapon.attachments.mag.asset.clone();
                if mag_asset.id() != Handle::<Scene>::default().id() {
                    commands.entity(entity).with_children(|parent| {
                        parent.spawn(SceneRoot(mag_asset));
                    });
                }
                found_mag = true;
            }

            if name.as_str() == "optic_socket" && !found_optic {
                if let Some(optic) = &weapon.attachments.optic {
                    commands.entity(entity).with_children(|parent| {
                        parent.spawn(SceneRoot(optic.asset.clone()));
                    });
                }
                found_optic = true;
            }

            if name.as_str() == "muzzle_socket" && !found_muzzle {
                if let Some(muzzle) = &weapon.attachments.muzzle {
                    commands.entity(entity).with_children(|parent| {
                        parent.spawn(SceneRoot(muzzle.asset.clone()));
                    });
                }
                found_muzzle = true;
            }
        }

        if found_mag || found_optic || found_muzzle {
            commands.entity(weapon_entity).insert(WeaponSocketsReady);
        }
    }
}

fn iter_descendants(children_query: &Query<&Children>, entity: Entity) -> Vec<Entity> {
    let mut result = vec![];
    let mut stack = vec![entity];

    while let Some(current) = stack.pop() {
        if let Ok(children) = children_query.get(current) {
            for child in children.iter() {
                result.push(child);
                stack.push(child);
            }
        }
    }
    result
}
