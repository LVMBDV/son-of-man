use bevy::asset::Assets;
use bevy::ecs::{Command, Commands, Entity, Resources, World};
use bevy::math::Vec2;
use bevy::render::mesh::Mesh;
use bevy::render::pipeline::PrimitiveTopology;
use extend::ext;

use crate::sectors::geometry::{ParentSector, SectorGeometry, SectorGeometryProperties, WallGeometry};

struct SpawnFreeSector {
    vertices: Vec<Vec2>,
    properties: SectorGeometryProperties
}

impl Command for SpawnFreeSector {
    fn write(self: Box<Self>, world: &mut World, resources: &mut Resources) {
        let sector_mesh = resources.get_mut::<Assets<Mesh>>().unwrap().add(Mesh::new(PrimitiveTopology::PointList));
        let sector_entity = world.spawn((SectorGeometry {walls: vec![], properties: self.properties}, sector_mesh));
        let mut walls = Vec::<Entity>::new();

        for (start_index, start_vertex) in self.vertices.iter().enumerate() {
            let end_index = if start_index == (self.vertices.len() - 1) { 0 } else { start_index + 1 };
            let end_vertex = self.vertices[end_index];

            walls.push(world.spawn((WallGeometry {a: *start_vertex, b: end_vertex}, ParentSector {entity: sector_entity})))
        }

        let mut sector_geometry = world.query_one_mut::<&mut SectorGeometry>(sector_entity).unwrap();
        sector_geometry.walls = walls;
    }
}

#[ext(pub)]
impl Commands {
    fn spawn_free_sector(&mut self, vertices: &[Vec2], properties: SectorGeometryProperties) -> &mut Commands {
        self.add_command(SpawnFreeSector {
            vertices: vertices.to_vec(), properties
        });
        self
    }
}
