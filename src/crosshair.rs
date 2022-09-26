use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_prototype_lyon::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub struct Crosshair {}

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::spawn_crosshair)
            .add_system(Self::draw_crasshair);
    }
}

impl CrosshairPlugin {
    fn spawn_crosshair(
        mut commands: Commands
    ) {
        let line = shapes::Line(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0)
        );
    
        let _crosshair = commands.spawn()
            .insert(Crosshair {})
            .insert_bundle(GeometryBuilder::build_as(
                &line,
            DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::rgba(1.0, 1.0, 1.0, 0.45)),
                    outline_mode: StrokeMode::new(Color::rgba(1.0, 1.0, 1.0, 0.1), 1.2),
                },
                Transform {
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                }
            )).id();
    }

    fn draw_crasshair(
        wnds: Res<Windows>,
        q_camera: Query<(&Camera, &GlobalTransform)>,
        player_query: Query<(&Player, &Transform), Without<Crosshair>>,
        mut crosshair_query: Query<(&mut Crosshair, &mut Path)>
    ){
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so query::single() is OK
        let (camera, camera_transform) = q_camera.single();
        let (_player, player_trans) = player_query.single();
        let (_crosshair, mut path) = crosshair_query.single_mut();
    
        // get the window that the camera is displaying to (or the primary window)
        let wnd = if let RenderTarget::Window(id) = camera.target {
            wnds.get(id).unwrap()
        } else {
            wnds.get_primary().unwrap()
        };

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    
            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    
            // matrix for undoing the projection and camera transform
            let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    
            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    
            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();
        
            // Draw Crosshair
            {
                let line = shapes::Line (
                    player_trans.translation.truncate(),
                    world_pos,
                );
        
                *path = ShapePath::build_as(&line);
            }
            
            // eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
        }
    }
}