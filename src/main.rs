use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

// mod block;
mod terminal_utils;
mod world;

use terminal_utils::TerminalUtilsPlugin;
use world::setup_world;

pub const SCREEN_WIDTH: u32 = 80;
pub const SCREEN_HEIGHT: u32 = 40;

fn setup(mut commands: Commands) {
    // spawn terminal
    let mut terminal = Terminal::new([SCREEN_WIDTH, SCREEN_HEIGHT]).with_clear_tile(Tile {
        glyph: ' ',
        fg_color: Color::WHITE,
        bg_color: Color::BLUE
    });
    terminal.put_string([0, 0], "Hello, world!".fg(Color::WHITE).bg(Color::BLUE));

    commands.spawn((
        TerminalBundle::from(terminal),
        AutoCamera
    ));
}

fn update() {
    // do nothing
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    name: Some("Grimplan".to_string()),
                    ..default()
                }),
                ..default()
            }),
            TerminalPlugin
        ))
        .add_systems(PreStartup, setup)
        .add_systems(Startup, setup_world)
        .add_systems(Update, update)
        .run();
}
