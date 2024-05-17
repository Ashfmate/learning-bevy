use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_entity);
    }
}

fn debug_entity(query: Query<(Entity, &Transform)>) {
    for (entity, pos) in query.iter() {
        info!("Entity {entity:?} is at position {:?}", pos.translation);
    }
}
