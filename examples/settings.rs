use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

fn main() {
    App::new()
        .insert_resource(AtmosphereSettings { resolution: 16 })
        .add_plugins(DefaultPlugins)
        .add_plugin(AtmospherePlugin)
        .add_plugin(bevy_flycam::NoCameraPlayerPlugin)
        .add_startup_system(setup)
        .add_system(change_resolution)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert(AtmosphereCamera(None))
        .insert(bevy_flycam::FlyCam);
}

// Change the resolution when the user presses a number key, 1-8
fn change_resolution(mut settings: ResMut<AtmosphereSettings>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key1) {
        change(settings.as_mut(), 8); // 8x8
    } else if keys.just_pressed(KeyCode::Key2) {
        change(settings.as_mut(), 16); // 16x16
    } else if keys.just_pressed(KeyCode::Key3) {
        change(settings.as_mut(), 32); // 32x32
    } else if keys.just_pressed(KeyCode::Key4) {
        change(settings.as_mut(), 64); // 64x64
    } else if keys.just_pressed(KeyCode::Key5) {
        change(settings.as_mut(), 128); // 128x128
    } else if keys.just_pressed(KeyCode::Key6) {
        change(settings.as_mut(), 256); // 256x256
    } else if keys.just_pressed(KeyCode::Key7) {
        change(settings.as_mut(), 512); // 512x512
    } else if keys.just_pressed(KeyCode::Key8) {
        change(settings.as_mut(), 1024); // 1024x1024
    }
}

// A separate `change` fn makes it easier to debug.
fn change(settings: &mut AtmosphereSettings, resolution: u32) {
    #[cfg(feature = "trace")]
    // bevy_atmosphere offers the "trace" feature for when you debug in tracy
    let _change_resolution_executed_span =
        info_span!("executed", name = "settings::change_resolution").entered();
    settings.resolution = resolution;
    info!("Changed resolution to {resolution}");
}