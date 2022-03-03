use std::fmt;

pub struct Matrix{
    pub(in crate) matrix_array: Vec<Vec<f32>>,
}

impl Matrix{
    pub fn new(row: usize, col: usize) -> Matrix{
        Matrix{matrix_array: vec![vec![0.0; col]; row],}
    }

    pub fn multiply_matrixes(&mut self, m1: Matrix){
        let mut matrix_result = Matrix::new(m1.matrix_array.len(), self.matrix_array[0].len());
        for result_i in 0..matrix_result.matrix_array.len(){
            for result_v in 0..matrix_result.matrix_array[result_i].len(){
                for m2_down_num in 0..self.matrix_array.len(){
                    matrix_result.matrix_array[result_i][result_v] += self.matrix_array[m2_down_num][result_v] * m1.matrix_array[result_i][m2_down_num];
                }
            }
        }

        *self = matrix_result;
    }

    pub fn identity(&mut self){
        for i in 0..self.matrix_array.len(){
            for v in 0..self.matrix_array[0].len(){
                if i == v{
                    self.matrix_array[i][v] = 1.0;
                }else{
                    self.matrix_array[i][v] = 0.0;
                }
            }
        }
    }

    pub fn print_matrix(&self){
        println!("{}", self);
    }
}

impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let mut result: String = "".to_owned();
        for i in 0..self.matrix_array.len(){
            for v in 0..self.matrix_array[i].len(){
                result.push_str(&(format!("{} ",self.matrix_array[i][v]).to_string()));
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}