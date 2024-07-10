use std::io;

#[derive(Debug)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<i32>>
}

impl Matrix {

    fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0; cols]; rows]
        }
    }

    fn collect_matrix_input(&mut self, name: &str) {
        println!("Enter numbers for the {} {}x{} matrix", name, self.rows, self.cols);
        for i in 0..self.rows {
            let mut data_input = String::new();
            io::stdin()
                .read_line(&mut data_input).expect("failed to read input");

            let row: Vec<i32> = data_input.trim().split_whitespace()
                    .map(|x| x.parse::<i32>().expect("Input not a number"))
                    .collect();
            self.data[i] = row;
        }
    }

    fn add(&self, other_matrix: Matrix) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);

        for r in 0..self.rows {
            for c in 0..self.cols {
                result.data[r][c] = self.data[r][c] + other_matrix.data[r][c]
            }
        }
        result
    }

    fn subtract(&self, other_matrix: Matrix) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);

        for r in 0..self.rows {
            for c in 0..self.cols {
                result.data[r][c] = self.data[r][c] - other_matrix.data[r][c]
            }
        }
        result
    }

    fn multiply(&self, other_matrix: Matrix) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);

        for r in 0..self.rows {
            for co in 0..self.cols{
                for c in 0..self.cols {
                    result.data[r][c] += self.data[r][c] * other_matrix.data[c][co]
                }
            }
        }
        result
    }

    fn show_result(&self) {
        println!("Your result:\n");
        for r in &self.data {
            let result: String = r.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
            println!("{result}")
        }
    }

}


fn get_matrix_dimension(name: &str) -> (usize, usize) {
    println!("Enter the row and col for {name} matrix\nFormat:(n n)");
    let mut dimensions_input = String::new();
    io::stdin()
        .read_line(&mut dimensions_input)
        .expect("failed to read input");

    let dimension: Vec<usize> = dimensions_input.trim()
                .split_whitespace()
                .map(|x| x.parse().expect("Input is not a number"))
                .collect();
    (dimension[0], dimension[1])
}

pub fn calculate() {
    let (row1, col1) = get_matrix_dimension("first");
    let (row2, col2) = get_matrix_dimension("second");

    let mut first_matrix = Matrix::new(row1, col1);
    let mut second_matrix = Matrix::new(row2, col2);

    first_matrix.collect_matrix_input("first");
    second_matrix.collect_matrix_input("second");
    println!("Enter an operation sign\nOptions: ('add', 'subtract', 'multiply')");
    let mut operation_input = String::new();
    io::stdin()
        .read_line(&mut operation_input)
        .expect("failed to read input");
    let operation: &str = operation_input.trim();
    match operation {
        "add" => first_matrix.add(second_matrix).show_result(),
        "subtract" => first_matrix.subtract(second_matrix).show_result(),
        "multiply" => first_matrix.multiply(second_matrix).show_result(),
        _ => println!("Select from options: ('add', 'subtract', 'multiply')")
    };
}
