pub struct CapacityBarPlugin;

impl Plugin for CapacityBarPlugin {
    fn build(self, &mut app: App) {
        app.add_system(update_player_capacity_bar)
    }
}

impl CapacityBarPlugin {
    pub fn attach_player_capacity_bar(
        commands: &mut Commands,
        camera: Entity
    ) {
        commands.entity(camera)
            .insert_bundle(StatBarBundle::new(
                StatBar { 
                    value: 1.0, 
                    length: 80.0, 
                    thickness: 4.0, 
                    style: StatBarStyle {
                        bar_color: BarColor::Fixed(Color::ORANGE),
                        empty_color: Color::BLACK,
                        ..Default::default()
                    },  
                    translation: Vec2::new(0.0, 35.0),                   
                    ..Default::default()
                }
            ));
    }

    fn update_player_capacity_bar(
        player_query: Query<&Player>,
        mut camera_stat_bar_query: Query<&mut StatBar, With<Camera2d>>
    ) {
        let player = player_query.single();
        let mut stat_bar = camera_stat_bar_query.single_mut();

        stat_bar.value = player.health.current / player.health.maximum;
    }
}