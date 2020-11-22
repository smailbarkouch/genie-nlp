use crate::genie::GenieError;
use rust_bert::pipelines::summarization::SummarizationModel;
use rust_bert::pipelines::question_answering::{QuestionAnsweringModel, QaInput};

const LEAST_RELEVANCE: f64 = 0.9;

pub struct RelevantAnswer {
    pub answer: String,
    pub score: f64
}

pub struct NLPHelp {}

impl NLPHelp {
    pub fn simplify(statements: &str) -> Result<Option<String>, GenieError> {
        let model = SummarizationModel::new(Default::default())?;
        let summaries = model.summarize([statements]);
        let summary = summaries.get(0);
        Ok(summary.map(|summary_ref| summary_ref.clone()))
    }

    pub fn is_relevant(question: &str, answer: String) -> Result<Vec<RelevantAnswer>, GenieError> {
        let model = QuestionAnsweringModel::new(Default::default())?;
        let predictions = model.predict(&[QaInput {
            question: String::from(question),
            context: answer.clone()
        }], 1, 32);

        let mut best_answers = Vec::<RelevantAnswer>::new();
        predictions.iter().for_each(|prediction_step| prediction_step.iter().for_each(|answer| {
            if answer.score > LEAST_RELEVANCE {
                best_answers.push(RelevantAnswer { answer: answer.answer.clone(), score: answer.score });
                return
            }
        }));

        Ok(best_answers)
    }
}