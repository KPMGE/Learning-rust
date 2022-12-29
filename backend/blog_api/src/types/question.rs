use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct QuestionId(pub String);

// impl FromStr for QuestionId {
//   type Err = Error;

//   fn from_str(id: &str) -> Result<Self, Self::Err> {
//     match id.is_empty() {
//       false => Ok(QuestionId(id.to_string())),
//       true => Err(Error::new(ErrorKind::InvalidInput, "No id provided"))
//     }
//   }
// }
