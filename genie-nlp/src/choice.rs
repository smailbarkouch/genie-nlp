use crate::genie::GenieError;
use rust_bert::pipelines::summarization::SummarizationModel;
use rust_bert::pipelines::question_answering::{QuestionAnsweringModel, QaInput};

const LEAST_RELEVANCE: f64 = 0.1;

pub struct RelevantAnswer {
    pub answer: String,
    pub score: f64,
}

pub struct NLPHelp {}

impl NLPHelp {
    pub fn simplify(statements: &str) -> Result<Option<String>, GenieError> {
        let model = SummarizationModel::new(Default::default())?;
        let summaries = model.summarize([statements]);
        let summary = summaries.get(0);
        Ok(summary.map(|summary_ref| summary_ref.clone()))
    }

    pub fn is_relevant(question: &str, answer: String, weight: f64) -> Result<Option<RelevantAnswer>, GenieError> {
        let model = QuestionAnsweringModel::new(Default::default())?;
        let sentences: Vec<&str> = answer.split(".").collect();
        // let mut true_answer = String::new();

        // for index in 0..10 {
        //     let sentence = sentences.get(index);
        //     if sentence.is_some() {
        //         true_answer.push_str(format!("{}. ", sentence.unwrap()).as_str());
        //     } else {
        //         break
        //     }
        // }

        let predictions = model.predict(&[QaInput {
            question: String::from(question),
            context: answer.clone(),
        }], 1, 32);

        let mut score: f64 = 0.0;
        let mut word_count: f64 = 0.0;
        predictions.iter().for_each(|prediction_step| prediction_step.iter().for_each(|model_answer| {
            word_count = word_count + 1.0;
            score += model_answer.score;
        }));

        let avg_score = score * weight / word_count ;
        println!("\n\nScore: {}, Answer: {}", avg_score, answer);
        if avg_score > LEAST_RELEVANCE {
            Ok(Some(RelevantAnswer { answer: answer.clone(), score: avg_score }))
        } else {
            Ok(None)
        }
    }
}