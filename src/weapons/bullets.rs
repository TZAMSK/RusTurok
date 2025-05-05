use bevy::prelude::*;

#[derive(Component)]
pub struct DespawnAfter(pub f32);

pub fn despawn_timed_entities(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(Entity, &DespawnAfter)>,
) {
    for (entity, DespawnAfter(despawn_time)) in query.iter() {
        if time.elapsed_secs() > *despawn_time {
            commands.entity(entity).despawn();
        }
    }
}
