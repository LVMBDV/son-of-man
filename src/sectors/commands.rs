use bevy::ecs::{Command, Commands, Resources, World};
use bevy::math::Vec2;
use extend::ext;

use crate::sectors::geometry::SectorGeometryProperties;

struct SpawnFreeSector {
    vertices: Vec<Vec2>,
    properties: SectorGeometryProperties
}

impl Command for SpawnFreeSector {
    fn write(self: Box<Self>, world: &mut World, resources: &mut Resources) {
    }
}

#[ext(pub)]
impl Commands {
    fn spawn_free_sector(&mut self, vertices: Vec<Vec2>, properties: SectorGeometryProperties) -> &mut Commands {
        self.add_command(SpawnFreeSector {
            vertices, properties
        });
        self
    }
}
