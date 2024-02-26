use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_field_unit_damage_info::PlayerFieldUnitDamageInfo;
use crate::notify_player_action_info::entity::player_field_unit_death_info::PlayerFieldUnitDeathInfo;
use crate::notify_player_action_info::entity::player_field_unit_health_point_info::PlayerFieldUnitHealthPointInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeApplyDamageToSpecificOpponentUnitResponse {
    player_field_unit_damage_info: PlayerFieldUnitDamageInfo,
    player_field_unit_health_point_info: PlayerFieldUnitHealthPointInfo,
    player_field_unit_death_info: PlayerFieldUnitDeathInfo
}

impl NoticeApplyDamageToSpecificOpponentUnitResponse {
    pub fn new(player_field_unit_damage_info: PlayerFieldUnitDamageInfo,
               player_field_unit_health_point_info: PlayerFieldUnitHealthPointInfo,
               player_field_unit_death_info: PlayerFieldUnitDeathInfo) -> Self {
        NoticeApplyDamageToSpecificOpponentUnitResponse {
            player_field_unit_damage_info,
            player_field_unit_health_point_info,
            player_field_unit_death_info
        }
    }

    pub fn get_player_field_unit_damage_info(&self) -> &PlayerFieldUnitDamageInfo {
        &self.player_field_unit_damage_info
    }

    pub fn get_player_field_unit_health_point_info(&self) -> &PlayerFieldUnitHealthPointInfo {
        &self.player_field_unit_health_point_info
    }

    pub fn get_player_field_unit_death_info(&self) -> &PlayerFieldUnitDeathInfo {
        &self.player_field_unit_death_info
    }
}