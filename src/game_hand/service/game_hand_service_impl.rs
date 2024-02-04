use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::card_race::repository::card_race_repository::CardRaceRepository;
use crate::card_race::repository::card_race_repository_impl::CardRaceRepositoryImpl;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;
use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::game_hand::repository::game_hand_repository::GameHandRepository;

use crate::game_hand::repository::game_hand_repository_impl::GameHandRepositoryImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::request::use_game_hand_energy_card_request::UseGameHandEnergyCardRequest;
use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;
use crate::game_hand::service::response::use_game_hand_energy_card_response::UseGameHandEnergyCardResponse;
use crate::game_hand::service::response::use_game_hand_unit_card_response::UseGameHandUnitCardResponse;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameHandServiceImpl {
    game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
    card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
    card_race_repository: Arc<AsyncMutex<CardRaceRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
}

impl GameHandServiceImpl {
    pub fn new(game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
               game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
               card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
               card_race_repository: Arc<AsyncMutex<CardRaceRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    ) -> Self {
        GameHandServiceImpl {
            game_hand_repository,
            game_field_unit_repository,
            card_kinds_repository,
            card_race_repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameHandServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameHandServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameHandServiceImpl::new(
                            GameHandRepositoryImpl::get_instance(),
                            GameFieldUnitRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance(),
                            CardRaceRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn get_account_unique_id(&self, session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");
        account_unique_id
    }

    async fn check_protocol_hacking(&mut self, account_unique_id: i32, unit_card_number: i32) -> bool {
        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let maybe_exist_hand_unit = game_field_unit_repository_guard.find_unit_by_id(account_unique_id, unit_card_number);

        if maybe_exist_hand_unit.is_none() {
            // TODO: 해킹 감지 (3회 이상 계정 영구 정지, 1회, 2회 -> 1시간 접속 불가)
            return true;
        }

        return false
    }

    async fn convert_race_string_to_enum(race_str: &str) -> RaceEnumValue {
        match race_str.to_lowercase().as_str() {
            "언데드" => RaceEnumValue::Undead,
            "휴먼" => RaceEnumValue::Human,
            "천사" => RaceEnumValue::Angel,
            "기계" => RaceEnumValue::Machine,
            "트랜트" => RaceEnumValue::Trent,
            _ => panic!("Invalid race string"),
        }
    }
}

#[async_trait]
impl GameHandService for GameHandServiceImpl {
    async fn use_specific_card(&mut self, use_game_hand_unit_card_request: UseGameHandUnitCardRequest) -> UseGameHandUnitCardResponse {
        println!("GameHandServiceImpl: use_specific_card()");

        let session_id = use_game_hand_unit_card_request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        let unit_card_number_string = use_game_hand_unit_card_request.get_unit_number();
        let unit_card_number = unit_card_number_string.parse::<i32>().unwrap();

        if self.check_protocol_hacking(account_unique_id, unit_card_number).await {
            println!("프로토콜 조작 감지: 해킹범을 검거합시다!");
            return UseGameHandUnitCardResponse::new(false)
        }

        let card_kinds_repository_guard = self.card_kinds_repository.lock().await;
        let maybe_unit_card = card_kinds_repository_guard.get_card_kind(&unit_card_number).await;
        if maybe_unit_card.unwrap() != "유닛" {
            return UseGameHandUnitCardResponse::new(false)
        }

        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        let specific_card_option = game_hand_repository_guard.use_specific_card(account_unique_id, unit_card_number);
        let specific_card = specific_card_option.unwrap();

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        game_field_unit_repository_guard.add_unit_to_game_field(account_unique_id, specific_card.get_card());

        UseGameHandUnitCardResponse::new(true)
    }

    // 에너지 카드 직접 1장 붙이기
    async fn attach_energy_card_to_field_unit(&mut self, use_game_hand_energy_card_request: UseGameHandEnergyCardRequest) -> UseGameHandEnergyCardResponse {
        println!("GameHandServiceImpl: attach_energy_card_to_field_unit()");

        let session_id = use_game_hand_energy_card_request.get_session_id();
        let account_unique_id = self.get_account_unique_id(session_id).await;

        let unit_card_number_string = use_game_hand_energy_card_request.get_unit_number();
        let unit_card_number = unit_card_number_string.parse::<i32>().unwrap();

        let energy_card_id_string = use_game_hand_energy_card_request.get_energy_card_id();
        let energy_card_id = energy_card_id_string.parse::<i32>().unwrap();

        let card_kinds_repository_guard = self.card_kinds_repository.lock().await;
        let maybe_energy_card = card_kinds_repository_guard.get_card_kind(&energy_card_id).await;
        if maybe_energy_card.unwrap() != "에너지" {
            return UseGameHandEnergyCardResponse::new(false)
        }

        let maybe_unit_card = card_kinds_repository_guard.get_card_kind(&unit_card_number).await;
        if maybe_unit_card.unwrap() != "유닛" {
            return UseGameHandEnergyCardResponse::new(false)
        }

        // TODO: Dictionary 값 아직 enum 처리 안되어 있음
        let card_race_repository_guard = self.card_race_repository.lock().await;
        let race_option = card_race_repository_guard.get_card_race(&energy_card_id).await;
        let race_string = race_option.unwrap();

        // TODO: 그로 인해 race_option 값을 문자열 기반으로 매칭해야함
        let race_enum = GameHandServiceImpl::convert_race_string_to_enum(&race_string).await;

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        game_field_unit_repository_guard.attach_energy_to_game_field_unit(account_unique_id, unit_card_number, race_enum, 1);

        UseGameHandEnergyCardResponse::new(true)
    }
}