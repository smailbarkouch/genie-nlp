use crate::genie::GenieError;
use rust_bert::pipelines::summarization::SummarizationModel;
use rust_bert::pipelines::question_answering::{QuestionAnsweringModel, QaInput};

const LEAST_RELEVANCE: f64 = 0.95;

pub struct NLPHelp {}

impl NLPHelp {
    pub fn simplify(statements: &str) -> Result<Vec<String>, GenieError> {
        let model = SummarizationModel::new(Default::default())?;
        Ok(model.summarize([statements]))
    }

    pub fn is_relevant(question: &str, answers: Vec<String>) -> Result<Option<Vec<String>>, GenieError> {
        let model = QuestionAnsweringModel::new(Default::default())?;
        let qa_inputs: Vec<QaInput> = answers.iter().map(|answer| {
            QaInput {
                question: String::from(question),
                context: answer.clone()
            }
        }).collect();
        let predictions = model.predict(&qa_inputs, 1, 32);
        let mut best_answers = Vec::<String>::new();
        predictions.iter().for_each(|prediction_step| prediction_step.iter().for_each(|answer| {
            if answer.score > LEAST_RELEVANCE {
                best_answers.push(answer.answer.clone())
            }
        }));

        Ok(Some(best_answers))
    }
}