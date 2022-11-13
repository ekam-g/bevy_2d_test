#![allow(unused)]
use bevy::prelude::*;

fn main () {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
        .insert_resource(
            WindowDescriptor{
                width: 720.0,
                height: 1080.0,
                title : "BETA".to_owned(),
                ..Default::default()
            }
        ).add_plugins(DefaultPlugins)
        .run()
}