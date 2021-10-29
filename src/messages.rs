use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum Messages {
    EmptySearchResult,
    NotExistsMart,
    NotExistsMartNearby
}

impl Serialize for Messages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serializer.serialize_str(
                match *self {
                    Messages::EmptySearchResult => "검색 결과가 없어요",
                    Messages::NotExistsMart => "검색한 점포가 없어요",
                    Messages::NotExistsMartNearby => "가까운 마트가 없어요",
                }
            )
    }
}