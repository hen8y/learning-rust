use std::fs;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]
struct Question {
    question: String,
    answer: String,
}

fn get_quiz() ->  Vec<Question> {
    let contents = fs::read_to_string("src/extras/quiz.json")
        .expect("unable to read file");
    let quiz: Vec<Question> = serde_json::from_str(&contents).expect("JSON Error");
    quiz
}

fn ask_question(question: &Question) ->String {
    println!("Question: {}", question.question);

    let mut answer_input = String::new();
    io::stdin()
        .read_line(&mut answer_input)
        .expect("failed to get input");
    let answer: String = answer_input.trim().to_string();
    answer
}

fn check_answer(answer: String, question: &Question) -> bool {
    answer.trim().to_lowercase() == question.answer.trim().to_lowercase()
}

fn get_results(question_asked: u16, score: u16) {
    println!("You scored {score} out of {question_asked} questions");
}

pub fn quiz() {

    println!("You have 20 questions, To cancel at anytime enter 'xCancel'\nProceed (y/n):");
    let mut choice_input = String::new();
    io::stdin()
        .read_line(&mut choice_input)
        .expect("failed to get choice");
    let choice = choice_input.trim().parse::<String>().expect("Input not accepted");
    if choice == "y" {
        let questions: Vec<Question> = get_quiz();

        let mut score: u16 = 0;
        let mut questions_asked: u16 = 0;
        for question in &questions {
            let answer: String = ask_question(question);
            if answer == "xCancel" {
                break;
            } else {
                questions_asked += 1;
                if check_answer(answer, question) {
                    score += 1;
                }
            }
        }
        get_results(questions_asked, score);
        return;
    } else if choice == "n" {
        println!("Good Bye.");
        return;
    } else {
        println!("Input not valid")
    }
}
