use rand::Rng;

pub mod player;
pub mod role;

use player::Player;
use role::Role;

pub struct Game {
    pub players: Vec<Player>,
    pub current_phase: GamePhase,
    pub day: u32,
}

pub enum GamePhase {
    Night,
    Morning,
    Discussion,
    Voting,
}

impl Game {
    pub fn new(player_names: Vec<String>) -> Self {
        let roles = role::assign_roles(player_names.len());
        let players = player::create_players(player_names, roles);

        Self {
            players,
            current_phase: GamePhase::Night,
            day: 1,
        }
    }

    pub fn next_phase(&mut self) {
        self.current_phase = match self.current_phase {
            GamePhase::Night => GamePhase::Morning,
            GamePhase::Morning => GamePhase::Discussion,
            GamePhase::Discussion => GamePhase::Voting,
            GamePhase::Voting => {
                self.day += 1;
                GamePhase::Night
            }
        };
    }

    pub fn check_victory_condition(&self) -> Option<String> {
        let alive_players: Vec<&Player> = self.players.iter().filter(|p| p.is_alive).collect();
        let werewolf_count = alive_players.iter().filter(|p| p.is_werewolf()).count();
        let villager_count = alive_players.len() - werewolf_count;

        if werewolf_count == 0 {
            Some("村人".to_string())
        } else if werewolf_count >= villager_count {
            Some("人狼".to_string())
        } else {
            None
        }
    }

    pub fn werewolf_attack(&mut self, target_id: usize) -> Vec<String> {
        let mut events = Vec::new();

        if let Some(_werewolf) = self.players.iter().find(|p| p.is_werewolf() && p.is_alive) {
            if let Some(target) = self
                .players
                .iter_mut()
                .find(|p| p.id == target_id && p.is_alive && !p.is_werewolf())
            {
                target.kill(self.day);
                events.push(format!("人狼が{}を襲撃しました。", target.name));
            } else {
                events.push("無効な襲撃対象が選択されました。".to_string());
            }
        }

        events
    }

    pub fn seer_divination(&self, target_id: usize) -> Vec<String> {
        let mut events = Vec::new();

        if let Some(seer) = self
            .players
            .iter()
            .find(|p| p.role == Role::Seer && p.is_alive)
        {
            if let Some(target) = self
                .players
                .iter()
                .find(|p| p.id == target_id && p.is_alive && p.id != seer.id)
            {
                let role_name = if target.is_werewolf() {
                    "人狼"
                } else {
                    "人狼ではない"
                };
                events.push(format!(
                    "占い師が{}を占いました。結果：{}",
                    target.name, role_name
                ));
            } else {
                events.push("無効な占い対象が選択されました。".to_string());
            }
        }

        events
    }

    pub fn morning_phase(&mut self) -> Vec<String> {
        let mut events = Vec::new();

        // 夜に襲撃されたプレイヤーの処理
        let killed_players: Vec<&Player> = self
            .players
            .iter()
            .filter(|p| !p.is_alive && p.death_day == Some(self.day))
            .collect();

        if killed_players.is_empty() {
            events.push("昨夜は誰も襲撃されませんでした。".to_string());
        } else {
            for player in killed_players {
                events.push(format!("{}が無残な姿で発見されました。", player.name));
            }
        }

        events
    }

    pub fn discussion_phase(&self) -> Vec<String> {
        vec!["討論フェーズが始まりました。".to_string()]
    }

    pub fn voting_phase(&mut self, votes: Vec<usize>) -> Vec<String> {
        let mut events = Vec::new();
        let mut vote_count = vec![0; self.players.len()];

        for (voter, &target) in self.players.iter().zip(votes.iter()) {
            if voter.is_alive {
                vote_count[target] += 1;
                events.push(format!(
                    "{}が{}に投票しました。",
                    voter.name, self.players[target].name
                ));
            }
        }

        let max_votes = *vote_count.iter().max().unwrap();
        // 最大票数を持つプレイヤーを見つける。投票が同数の場合は
        let max_voted_indexes = self
            .players
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if p.is_alive && vote_count[i] == max_votes {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        assert!(max_voted_indexes.len() >= 1);
        let executed_index = if max_voted_indexes.len() == 1 {
            max_voted_indexes[0]
        } else {
            // 投票が同数の場合は、ランダムに一人処刑される
            let random_index = rand::thread_rng().gen_range(0..max_voted_indexes.len());
            max_voted_indexes[random_index]
        };

        let player = &mut self.players[executed_index];
        player.kill(self.day);
        events.push(format!("{}が処刑されました。", player.name));

        events
    }
}
