use std::collections::HashSet;

use bevy::asset::{Assets, Handle};
use bevy::ecs::{Changed, Entity, ResMut, Query};
use bevy::math::Vec2;
use bevy::render::mesh::Mesh;

pub struct Portal {
    pub a: Entity,
    pub b: Entity
}

pub struct WallGeometryProperties {
    pub portal_between: Option<Portal>
}

pub struct WallGeometry {
    pub a: Vec2,
    pub b: Vec2,
}

pub struct ParentSector {
    pub entity: Entity
}

pub struct SectorGeometryProperties {
    pub ceiling_height: f32,
    pub floor_depth: f32,
    pub is_sloped: bool
}

pub struct SectorGeometry {
    pub walls: Vec<Entity>,
    pub properties: SectorGeometryProperties
}
