use std::collections::HashSet;

use bevy::asset::{Assets, Handle};
use bevy::ecs::{Changed, Entity, ResMut, Query};
use bevy::math::Vec2;
use bevy::render::mesh::Mesh;

use crate::sectors::geometry::*;

fn generate_sector_mesh(entity: Entity,
                        walls: &Query<&WallGeometry>,
                        mut sectors: &mut Query<(&SectorGeometry, &mut Handle<Mesh>)>,
                        meshes: &ResMut<Assets<Mesh>>) {
    let (sector_geometry, mut sector_mesh_handle) = sectors.get_mut(entity).unwrap();

    println!("Sector #{0} mesh has been regenerated.", entity.id());
}

pub fn generate_sector_meshes(all_walls: Query<&WallGeometry>,
                          changed_walls: Query<&ParentSector, Changed<WallGeometry>>,
                          changed_sector_geometries: Query<&Entity, Changed<SectorGeometry>>,
                          mut sectors: Query<(&SectorGeometry, &mut Handle<Mesh>)>,
                          meshes: ResMut<Assets<Mesh>>) {
    let mut finished_sectors = HashSet::<Entity>::new();

    for parent_sector in changed_walls.iter() {
        if !finished_sectors.contains(&parent_sector.entity) {
            generate_sector_mesh(parent_sector.entity, &all_walls, &mut sectors, &meshes);
            finished_sectors.insert(parent_sector.entity);
        }
    }

    for changed_sector_entity in changed_sector_geometries.iter() {
        if !finished_sectors.contains(changed_sector_entity) {
            generate_sector_mesh(*changed_sector_entity, &all_walls, &mut sectors, &meshes);
            finished_sectors.insert(*changed_sector_entity);
        }
    }
}
