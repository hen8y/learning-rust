use std::io;

fn main() {
    println!("Enter number of students:");
    let mut students_list_input = String::new();
    let mut scores: f64 = 0.0;
    let mut weights: f64 = 0.0;

    io::stdin()
        .read_line(&mut students_list_input)
        .expect("failed to read input");

    let students_lists = students_list_input.trim().parse::<i64>().expect("Input is not a number");


    for i in 1..=students_lists {
        println!("Enter score for student {i}:");
        let mut score_input = String::new();
        io::stdin()
            .read_line(&mut score_input)
            .expect("failed to read input");

        let score = score_input.trim().parse::<f64>().expect("Input is not a number");
        scores = scores + score;

        println!("Enter weight of student {i}:");
        let mut weight_input = String::new();
        io::stdin()
            .read_line(&mut weight_input)
            .expect("failed to read input");

        let weight = weight_input.trim().parse::<f64>().expect("Input is not a number");
        weights = weights + weight;

    }

    let score_average = scores / students_lists as f64;
    let weight_average = weights / students_lists as f64;
    println!("Score Average: {score_average}");
    println!("Weight Average: {weight_average}");

}
