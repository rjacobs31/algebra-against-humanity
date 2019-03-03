#[derive(Clone)]
pub struct QuestionCard {
    pub id: usize,
    pub text: String,
    pub num_blanks: usize,
}

#[derive(Clone)]
pub struct AnswerCard {
    pub id: usize,
    pub text: String,
}
