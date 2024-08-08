use crate::game::role::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub role: Role,
    pub is_alive: bool,
    pub death_day: Option<u32>,
}

impl Player {
    pub fn new(id: usize, name: String, role: Role) -> Self {
        Self {
            id,
            name,
            role,
            is_alive: true,
            death_day: None,
        }
    }

    pub fn kill(&mut self, day: u32) {
        self.is_alive = false;
        self.death_day = Some(day);
    }

    pub fn is_werewolf(&self) -> bool {
        self.role.is_werewolf()
    }
}

pub fn create_players(names: Vec<String>, roles: Vec<Role>) -> Vec<Player> {
    names
        .into_iter()
        .zip(roles.into_iter())
        .enumerate()
        .map(|(id, (name, role))| Player::new(id, name, role))
        .collect()
}
