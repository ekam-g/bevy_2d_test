use bevy::prelude::*;


pub fn setup(mut command : Commands) {
    //camera
    command.spawn_bundle(OrthographicCameraBundle::new_2d());
    //sprite
    command.spawn_bundle(SpriteBundle{
        sprite : Sprite {
            color : Color::rgb( 0.25, 0.25, 0.25),
            custom_size : Some(Vec2::new(150.0 , 150.0 )),
            ..Default::default()
        },
        ..Default::default()
    });
}