struct Conv2D{
    kernel: (i64,i64),
    kernel_weights: Vec<Vec<f64>>,
    input_array: Vec<Vec<f64>>
}

impl Conv2D{

 fn new(kernel: (i64, i64), input_array: Vec<Vec<f64>>) -> Self {
    let kernel_rows = kernel.0 as usize;
    let kernel_cols = kernel.1 as usize;
    
    let mut kernel_weights = vec![vec![0.0; kernel_cols]; kernel_rows];
    for i in 0..kernel_rows {
        for j in 0..kernel_cols {
            kernel_weights[i][j] = if (i + j) % 2 == 0 { 0.5 } else { -0.3 };
        }
    }
    
    Conv2D {
        kernel,
        kernel_weights,
        input_array
    }
}

 fn convolve(&self) -> Vec<Vec<f64>> {
    let input_rows = self.input_array.len();
    let input_cols = self.input_array[0].len();
    let kernel_rows = self.kernel.0 as usize;
    let kernel_cols = self.kernel.1 as usize;
    
    let output_rows: usize = input_rows - kernel_rows + 1;
    let output_cols = input_cols - kernel_cols + 1;
    
    let mut output: Vec<Vec<f64>> = vec![vec![0.0; output_cols]; output_rows];
    
    for i in 0..output_rows {
        for j in 0..output_cols {
            let mut sum = 0.0;
            
            for ki in 0..kernel_rows {
                for kj in 0..kernel_cols {
                    sum += self.input_array[i + ki][j + kj] * self.kernel_weights[ki][kj];
                }
            }
            
            output[i][j] = sum;
            
        }
    }
    
    output
}
}

fn main() {
    let conv1 = Conv2D::new(
        (3, 1),
        vec![
            vec![5.5, 8.2, 3.7, 9.1],
            vec![2.3, 6.8, 1.4, 7.9],
            vec![4.6, 0.9, 5.2, 8.3],
            vec![1.7, 3.5, 9.8, 2.1]
        ]
    );

}




