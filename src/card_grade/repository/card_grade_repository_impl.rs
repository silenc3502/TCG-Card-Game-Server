use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_grade::repository::card_grade_repository::CardGradeRepository;
use crate::common::csv::csv_reader::{build_card_grade_dictionary, csv_read};
use crate::common::path::root_path::RootPath;

pub struct CardGradeRepositoryImpl {
    card_grade_map: Arc<AsyncMutex<HashMap<i32, i32>>>,
}

impl CardGradeRepositoryImpl {
    pub fn new() -> Self {
        let filename = RootPath::make_full_path("resources/csv/card_data.csv")
            .unwrap_or_else(|| {
                eprintln!("Failed to get file path.");
                std::process::exit(1);
            });
        let filename_path = &filename.to_string_lossy();

        let csv_content = match csv_read(filename_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading CSV file: {}", err);
                std::process::exit(1);
            }
        };

        let card_grade_dictionary = build_card_grade_dictionary(&csv_content);

        CardGradeRepositoryImpl {
            card_grade_map: Arc::new(AsyncMutex::new(card_grade_dictionary)),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<CardGradeRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardGradeRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardGradeRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardGradeRepository for CardGradeRepositoryImpl {
    async fn get_card_grade(&self, card_number: &i32) -> Option<i32> {
        let card_grade_map_guard = self.card_grade_map.lock().await;
        card_grade_map_guard.get(card_number).cloned()
    }

    async fn get_legend_mythical_card_list(&self, card_list: Vec<i32>) -> Vec<(i32)>{
        let card_grade_map_guard = self.card_grade_map.lock().await;
        let mut legend_mythical_card_list : Vec<(i32)> = Vec::new();
        println!("{}", card_grade_map_guard[&58]);

        for card_num in card_list {
            if(card_grade_map_guard[&card_num] == 4 || card_grade_map_guard[&card_num] == 5) {
                legend_mythical_card_list.push(card_num);
            }
        }
        legend_mythical_card_list
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_card_grade() {
        let card_grade_repository_mutex = CardGradeRepositoryImpl::get_instance();
        let card_grade_repository_guard = card_grade_repository_mutex.lock().await;
        let card_number: i32 = 6;
        let card_grade = card_grade_repository_guard.get_card_grade(&card_number).await;

        match card_grade {
            Some(grade) => {
                println!("Card Grade: {}", grade);
                assert_eq!(grade, 1);
            }
            None => println!("Card not found."),
        }
    }
}