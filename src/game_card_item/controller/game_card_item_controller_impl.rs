use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::battle_room::service::response::find_opponent_by_account_id_response::FindOpponentByAccountIdResponse;
use crate::card_grade::service::card_grade_service::CardGradeService;
use crate::card_grade::service::card_grade_service_impl::CardGradeServiceImpl;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::game_card_energy::controller::response_form::attach_general_energy_card_response_form::AttachGeneralEnergyCardResponseForm;
use crate::game_card_item::controller::game_card_item_controller::GameCardItemController;
use crate::game_card_item::controller::request_form::add_field_energy_with_field_unit_health_point_item_request_form::AddFieldEnergyWithFieldUnitHealthPointRequestForm;
use crate::game_card_item::controller::request_form::target_death_item_request_form::TargetDeathItemRequestForm;
use crate::game_card_item::controller::response_form::add_field_energy_with_field_unit_health_point_item_response_form::AddFieldEnergyWithFieldUnitHealthPointResponseForm;
use crate::game_card_item::controller::response_form::target_death_item_response_form::TargetDeathItemResponseForm;
use crate::game_card_item::service::game_card_item_service::GameCardItemService;

use crate::game_card_item::service::game_card_item_service_impl::GameCardItemServiceImpl;
use crate::game_card_item::service::request::summary_item_card_effect_request::SummaryItemCardEffectRequest;
use crate::game_card_item::service::response::summary_item_card_effect_response::SummaryItemCardEffectResponse;
use crate::game_card_support::controller::response_form::energy_boost_support_response_form::EnergyBoostSupportResponseForm;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_field_energy::service::game_field_energy_service::GameFieldEnergyService;
use crate::game_field_energy::service::game_field_energy_service_impl::GameFieldEnergyServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_hand::service::request::use_game_hand_item_card_request::UseGameHandItemCardRequest;
use crate::game_hand::service::request::use_game_hand_support_card_request::UseGameHandSupportCardRequest;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_item_card_request::IsItItemCardRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action::service::notify_player_action_service::NotifyPlayerActionService;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct GameCardItemControllerImpl {
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    card_grade_service: Arc<AsyncMutex<CardGradeServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_card_item_service: Arc<AsyncMutex<GameCardItemServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
}

impl GameCardItemControllerImpl {
    pub fn new(game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               card_grade_service: Arc<AsyncMutex<CardGradeServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_card_item_service: Arc<AsyncMutex<GameCardItemServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,) -> Self {

        GameCardItemControllerImpl {
            game_hand_service,
            game_tomb_service,
            card_grade_service,
            battle_room_service,
            game_card_item_service,
            game_field_unit_service,
            game_protocol_validation_service,
            redis_in_memory_service,
            notify_player_action_service,
            game_field_energy_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardItemControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardItemControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardItemControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            GameTombServiceImpl::get_instance(),
                            CardGradeServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            GameCardItemServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameFieldEnergyServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    // TODO: 모든 Controller는 검증 로직 때문에 반복을 줄이기 위해 추후 Aspect 처리 필요함
    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;
        drop(redis_in_memory_service_guard);
        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }

    async fn is_valid_protocol(&self, check_protocol_hacking_request: CheckProtocolHackingRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let check_protocol_hacking_response = game_protocol_validation_service_guard.check_protocol_hacking(check_protocol_hacking_request).await;
        drop(game_protocol_validation_service_guard);
        check_protocol_hacking_response.is_success()
    }

    async fn is_it_item_card(&self, is_it_item_card_request: IsItItemCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let is_it_item_card_response = game_protocol_validation_service_guard.is_it_item_card(is_it_item_card_request).await;
        drop(game_protocol_validation_service_guard);
        is_it_item_card_response.is_success()
    }

    async fn is_able_to_use(&self, can_use_card_request: CanUseCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let can_use_card_response = game_protocol_validation_service_guard.can_use_card(can_use_card_request).await;
        drop(game_protocol_validation_service_guard);
        can_use_card_response.is_success()
    }

    async fn use_item_card(&self, use_game_hand_item_card_request: UseGameHandItemCardRequest) -> i32 {
        let mut game_hand_service_guard = self.game_hand_service.lock().await;
        let use_game_hand_item_card_response = game_hand_service_guard.use_item_card(use_game_hand_item_card_request).await;
        drop(game_hand_service_guard);
        use_game_hand_item_card_response.get_found_item_card_id()
    }

    async fn place_used_card_to_tomb(&self, place_to_tomb_request: PlaceToTombRequest) {
        let mut game_tomb_service_guard = self.game_tomb_service.lock().await;
        game_tomb_service_guard.add_used_card_to_tomb(place_to_tomb_request).await;
    }

    async fn get_summary_of_item_card(&self, summary_item_card_effect_request: SummaryItemCardEffectRequest) -> SummaryItemCardEffectResponse {
        let mut game_card_item_service_guard = self.game_card_item_service.lock().await;
        let summary_item_card_effect_response = game_card_item_service_guard.summary_item_card(summary_item_card_effect_request).await;
        drop(game_card_item_service_guard);
        summary_item_card_effect_response
    }
}

#[async_trait]
impl GameCardItemController for GameCardItemControllerImpl {
    async fn request_to_use_target_death_item(&self, target_death_item_request_form: TargetDeathItemRequestForm) -> TargetDeathItemResponseForm {
        println!("GameCardItemControllerImpl: request_to_use_target_death_item()");

        // 1. Redis에서 토큰을 가지고 있는지 검증
        let account_unique_id = self.is_valid_session(target_death_item_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session");
            return TargetDeathItemResponseForm::new(false)
        }

        // TODO: 세션을 제외하고 애초에 UI 에서 숫자로 전송하면 더 좋다.
        let item_card_id_string = target_death_item_request_form.get_item_card_id();
        let item_card_id = item_card_id_string.parse::<i32>().unwrap();

        // 2. Hand 에 있는지 확인하여 해킹 여부 검증
        let check_protocol_hacking_response = self.is_valid_protocol(
            target_death_item_request_form.to_check_protocol_hacking_request(account_unique_id, item_card_id)).await;
        if !check_protocol_hacking_response {
            println!("해킹범을 검거합니다!");
            return TargetDeathItemResponseForm::new(false)
        }

        // 3. 실제 아이템 카드가 맞는지 확인
        let is_it_item_response = self.is_it_item_card(
            target_death_item_request_form.to_is_it_item_card_request(item_card_id)).await;
        if !is_it_item_response {
            println!("서포트 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return TargetDeathItemResponseForm::new(false)
        }

        // 4. GameProtocolValidation Service 호출하여 사용 가능한지 조건 검사 (신화 > 4라운드 제약)
        let can_use_card_response = self.is_able_to_use(
            target_death_item_request_form.to_can_use_card_request(account_unique_id, item_card_id)).await;
        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return TargetDeathItemResponseForm::new(false)
        }

        // 5. 효과를 적용하기 위해 Game Card Item Service 호출하여 필요 효과 설정
        let summarized_item_effect_response = self.get_summary_of_item_card(
            target_death_item_request_form.to_summary_item_effect_request(item_card_id)).await;

        // TODO: 사용 방식에 대한 논의 필요 <- UI에서 어떻게 보여줄 것인가
        // 6. 필드 에너지 혹은 손패의 에너지가 충분한지 확인
        // 여기서 summarized_item_effect_response의 required_energy 처리

        // 7. Hand Service 호출하여 카드 사용
        let usage_hand_card = self.use_item_card(
            target_death_item_request_form.to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

        // 8. Item 카드 사용이므로 Tomb Service 호출하여 무덤 배치
        self.place_used_card_to_tomb(
            target_death_item_request_form.to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        // 9. 즉사 스킬 적용을 위해 상대방의 고유 id 값을 확보
        let battle_room_service_guard = self.battle_room_service.lock().await;
        let find_opponent_by_account_id_response = battle_room_service_guard.find_opponent_by_account_unique_id(
            target_death_item_request_form.to_find_opponent_by_account_id_request(account_unique_id)).await;

        // TODO: 세션을 제외하고 애초에 UI에서 숫자로 전송하면 더 좋다.
        let opponent_target_unit_index_string = target_death_item_request_form.get_opponent_target_unit_index();
        let opponent_target_unit_index = opponent_target_unit_index_string.parse::<i32>().unwrap();

        // TODO: 추후 즉사 면역인 언데드 등등에 대한 조건도 필요함
        // 10. 타겟 인덱스 유닛이 신화 미만인지 확인
        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let find_target_unit_id_by_index_response = game_field_unit_service_guard.find_target_unit_id_by_index(
            target_death_item_request_form.to_find_target_unit_id_by_index_request(
                find_opponent_by_account_id_response.get_opponent_unique_id(),
                opponent_target_unit_index)).await;

        let card_grade_service_guard = self.card_grade_service.lock().await;
        let opponent_target_unit_grade = card_grade_service_guard.get_card_grade(
            &find_target_unit_id_by_index_response.get_found_opponent_unit_id()).await;

        // 즉사 아이템 (죽음의 낫) 적용
        // self.apply_instant_death_to_opponent_unit(
        //     &find_opponent_by_account_id_response,
        //     target_death_item_request_form,
        //     opponent_target_unit_index,
        //     &summarized_item_effect_response,
        //     opponent_target_unit_grade,
        //     usage_hand_card).await;

        // 11. Field Unit Service를 호출하여 상대 유닛에 Alternatives 적용
        if opponent_target_unit_grade == GradeEnum::Mythical {
            game_field_unit_service_guard.apply_damage_to_target_unit_index(
                target_death_item_request_form.to_apply_damage_to_target_unit_index(
                    find_opponent_by_account_id_response.get_opponent_unique_id(),
                    opponent_target_unit_index,
                    summarized_item_effect_response.get_alternatives_damage())).await;

            // 즉사기에 면역되어 alternatives로 작용하였음을 알림
            let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
            let notify_to_opponent_you_use_item_card_response = notify_player_action_service_guard
                .notify_to_opponent_you_use_item_instant_death_alternatives(
                    target_death_item_request_form.to_notify_to_opponent_you_use_item_instant_death_alternatives_request(
                        find_opponent_by_account_id_response.get_opponent_unique_id(),
                        opponent_target_unit_index,
                        usage_hand_card,
                        summarized_item_effect_response.get_alternatives_damage())).await;

            if !notify_to_opponent_you_use_item_card_response.is_success() {
                println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
                return TargetDeathItemResponseForm::new(false)
            }

            return TargetDeathItemResponseForm::new(true)
        }

        // 12. Field Unit Service를 호출하여 상대 유닛에 즉사 적용
        let apply_instant_death_to_target_unit_index_response = game_field_unit_service_guard.apply_instant_death_to_target_unit_index(
            target_death_item_request_form.to_apply_instant_death_to_target_unit_index_request(
                find_opponent_by_account_id_response.get_opponent_unique_id(),
                opponent_target_unit_index)).await;

        // 즉사기가 적용되어 실제 사망 처리 되었음을 알림
        let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
        let notify_to_opponent_you_use_item_card_response = notify_player_action_service_guard
            .notify_to_opponent_you_use_item_instant_death(
                target_death_item_request_form.to_notify_to_opponent_you_use_item_instant_death_request(
                    find_opponent_by_account_id_response.get_opponent_unique_id(),
                    opponent_target_unit_index,
                    usage_hand_card)).await;

        if !notify_to_opponent_you_use_item_card_response.is_success() {
            println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
            return TargetDeathItemResponseForm::new(false)
        }

        TargetDeathItemResponseForm::new(true)
    }

    async fn request_to_use_add_field_energy_with_field_unit_health_point(&self, add_field_energy_with_field_unit_health_point_request_form: AddFieldEnergyWithFieldUnitHealthPointRequestForm) -> AddFieldEnergyWithFieldUnitHealthPointResponseForm {
        println!("GameCardItemControllerImpl: request_to_use_add_field_energy_with_field_unit_health_point()");

        let account_unique_id = self.is_valid_session(add_field_energy_with_field_unit_health_point_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        let item_card_id_string = add_field_energy_with_field_unit_health_point_request_form.get_item_card_id();
        let item_card_id = item_card_id_string.parse::<i32>().unwrap();

        let check_protocol_hacking_response = self.is_valid_protocol(
            add_field_energy_with_field_unit_health_point_request_form.to_check_protocol_hacking_request(account_unique_id, item_card_id)).await;
        if !check_protocol_hacking_response {
            println!("해킹범을 검거합니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        let is_it_item_response = self.is_it_item_card(
            add_field_energy_with_field_unit_health_point_request_form.to_is_it_item_card_request(item_card_id)).await;
        if !is_it_item_response {
            println!("아이템 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        let can_use_card_response = self.is_able_to_use(
            add_field_energy_with_field_unit_health_point_request_form.to_can_use_card_request(account_unique_id, item_card_id)).await;
        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        let field_unit_index_string = add_field_energy_with_field_unit_health_point_request_form.get_field_unit_index();
        let field_unit_index = field_unit_index_string.parse::<i32>().unwrap();

        let game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let get_current_health_point_of_field_unit_by_index_response = game_field_unit_service_guard
            .get_current_health_point_of_field_unit_by_index(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_get_field_unit_health_point_request(account_unique_id, field_unit_index)).await;

        let current_health_point_of_field_unit = get_current_health_point_of_field_unit_by_index_response.get_current_unit_health_point();

        if current_health_point_of_field_unit == -1 {
            println!("필드에 존재하지 않는 유닛을 지정하여 보냈으므로 당신도 해킹범입니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        drop(game_field_unit_service_guard);

        let mut summarized_item_effect_response = self.get_summary_of_item_card(
            add_field_energy_with_field_unit_health_point_request_form.to_summary_item_effect_request(item_card_id)).await;

        let field_energy_amount_to_increase = summarized_item_effect_response
            .get_field_energy_addition_calculator()
                .calculation_of_field_energy_increment(current_health_point_of_field_unit);

        let game_field_energy_service_guard = self.game_field_energy_service.lock().await;
        let add_field_energy_with_amount_response = game_field_energy_service_guard.add_field_energy_with_amount(
            add_field_energy_with_field_unit_health_point_request_form
                .to_add_field_energy_with_amount_request(account_unique_id, field_energy_amount_to_increase)).await;
        if !add_field_energy_with_amount_response.is_success() {
            println!("필드에 에너지를 추가하는 과정에서 문제가 발생했습니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        drop(game_field_energy_service_guard);

        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id = battle_room_service_guard
            .find_opponent_by_account_unique_id(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
        let notify_opponent_you_use_item_card_response = notify_player_action_service_guard
            .notify_opponent_you_use_item_field_energy_increase_item_card(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_notify_opponent_you_use_item_field_energy_increase_request(opponent_unique_id, item_card_id, field_energy_amount_to_increase)).await;
        if !notify_opponent_you_use_item_card_response.is_success() {
            println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(false)
        }

        drop(notify_player_action_service_guard);

        let usage_hand_card = self.use_item_card(
            add_field_energy_with_field_unit_health_point_request_form.to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

        self.place_used_card_to_tomb(
            add_field_energy_with_field_unit_health_point_request_form.to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        AddFieldEnergyWithFieldUnitHealthPointResponseForm::new(true)
    }
}