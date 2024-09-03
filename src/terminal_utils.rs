use bevy::{ecs::event, prelude::*};
use bevy_ascii_terminal::prelude::*;

pub struct TerminalUtilsPlugin;

impl Plugin for TerminalUtilsPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Event)]
pub struct TerminalScanEvent(String);

#[derive(Event)]
pub struct TerminalScanResultEvent(String);

pub fn terminal_scan_event_system(
    mut terminal: Query<&mut Terminal>,
    mut events: EventReader<TerminalScanEvent>
) {
    let mut terminal = terminal.single_mut();

    for (event, _id) in events.read_with_id() {
        terminal.clear();
        terminal.put_string(
            [0, 1].pivot(Pivot::BottomLeft),
            event.0.clone().fg(Color::WHITE).bg(Color::BLUE)
        );
        terminal.put_char([0, 0].pivot(Pivot::BottomLeft), '>'.fg(Color::WHITE).bg(Color::BLUE));
    }
}
