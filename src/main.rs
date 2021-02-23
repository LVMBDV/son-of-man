mod sectors;

use bevy::prelude::*;
use bevy::math::vec2;

use crate::sectors::rendering::generate_sector_meshes;
use crate::sectors::geometry::SectorGeometryProperties;
use crate::sectors::commands::CommandsExt;

fn setup(commands: &mut Commands) {
    commands
        .spawn_free_sector(
            &[vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.5, 1.0)],
            SectorGeometryProperties {
                ceiling_height: 1.0,
                floor_depth: 0.0,
                is_sloped: false
            }
        );
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(generate_sector_meshes.system())
        .run();
}
