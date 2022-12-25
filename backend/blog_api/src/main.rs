#[derive(Debug)]
struct QuestionId(String);

#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

fn main() {
  let q1 = Question::new(
    QuestionId("1234".to_string()),
    "What's the answer for the life, the universe and everything?".to_string(),
    "some content".to_string(),
    Some(vec!["faq".to_string(), "fiction".to_string()]),
  );

  println!("{:?}", q1);
}
