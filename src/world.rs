use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

use crate::SCREEN_HEIGHT;

pub trait Country {
    fn name(&self) -> String;
    fn capital(&self) -> String;
    fn points(&self) -> i64;
    fn population(&self) -> u64;
    fn manpower(&self) -> u64;
    fn is_guerrilla(&self) -> bool {
        false
    }
}

#[derive(Component, Debug)]
pub struct Player {
    name: String,
    capital: String,
    points: i64,
    population: u64,
    manpower: u64,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "Colombia".into(),
            capital: "Bogotá".into(),
            points: 0,
            population: 50_000_000,
            manpower: 500_000
        }
    }
}

impl Country for Player {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn capital(&self) -> String {
        self.capital.clone()
    }

    fn points(&self) -> i64 {
        self.points
    }

    fn population(&self) -> u64 {
        self.population
    }

    fn manpower(&self) -> u64 {
        self.manpower
    }
}

#[derive(Component, Debug)]
pub struct EnemyFarc {
    points: i64,
    population: u64,
    manpower: u64,
}

impl Country for EnemyFarc {
    fn name(&self) -> String {
        "FARC (dissidents)".into()
    }

    fn capital(&self) -> String {
        "N/A".into()
    }

    fn points(&self) -> i64 {
        self.points
    }

    fn population(&self) -> u64 {
        self.population
    }

    fn manpower(&self) -> u64 {
        self.manpower
    }

    fn is_guerrilla(&self) -> bool {
        true
    }
}

impl Default for EnemyFarc {
    fn default() -> Self {
        Self {
            points: 0,
            population: 10_000,
            manpower: 10_000
        }
    }
}

/// Sets up the world data
pub fn setup_world(mut commands: Commands, mut terminal_query: Query<&mut Terminal>) {
    let player = Player::default();
    let enemy = EnemyFarc::default();

    terminal_query.single_mut().put_string(
        [0, 0].pivot(Pivot::TopLeft),
        format!("{:#?}\n{:#?}", player, enemy).
        fg(Color::WHITE).bg(Color::BLUE)
    );

    commands.spawn(player);
    commands.spawn(enemy);
}
