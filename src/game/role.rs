use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Villager,
    Werewolf,
    Seer,
}

impl Role {
    pub fn description(&self) -> &'static str {
        match self {
            Role::Villager => "村人：特別な能力はありませんが、議論と投票に参加します。",
            Role::Werewolf => "人狼：夜に村人を襲撃します。昼は村人のふりをします。",
            Role::Seer => "占い師：夜に一人のプレイヤーの役割を知ることができます。",
        }
    }

    pub fn is_werewolf(&self) -> bool {
        matches!(self, Role::Werewolf)
    }
}

pub fn assign_roles(player_count: usize) -> Vec<Role> {
    let mut roles = vec![Role::Villager; player_count];
    let mut rng = thread_rng();

    // 人狼の数を決定（プレイヤー数の1/3程度）
    let werewolf_count = (player_count / 3).max(1);

    // 占い師は1人
    let seer_count = 1;

    // 役割を割り当てる
    for role in roles.iter_mut().take(werewolf_count) {
        *role = Role::Werewolf;
    }

    for role in roles.iter_mut().skip(werewolf_count).take(seer_count) {
        *role = Role::Seer;
    }

    // ランダムに並び替える
    roles.shuffle(&mut rng);

    roles
}
