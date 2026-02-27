fn create_adjacency_matrix(dim: usize) -> Vec<Vec<bool>> {
    let mut matrix = vec![vec![false; dim + 2]; dim + 2];
    
    for i in 1..=dim {
        matrix[i][i] = true;          

        if i < dim {
            matrix[i][i+1] = true;     
            matrix[i+1][i] = true;     
        }
    }
    
    if dim >= 4 {
        for i in 1..=dim-3 {
            matrix[i][i+3] = true;
            matrix[i+3][i] = true;
        }
    }

    get_matrix(&matrix) 
}

fn get_matrix(source_matrix: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let rows = source_matrix.len() - 2;
    let cols = source_matrix[0].len() - 2;
    let mut matrix = vec![vec![false; cols]; rows];
    
    for i in 1..source_matrix.len() - 1 {
        for j in 1..source_matrix[i].len() - 1 {
            matrix[i - 1][j - 1] = source_matrix[i][j];
        }
    }
    matrix
}

fn print_matrix(matrix: &Vec<Vec<bool>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!("{} ", if matrix[i][j] { "1" } else { "0" });
        }
        println!();
    }
}

fn generation_of_next_nbit_binary_vector(input_vector: &mut Vec<bool>) -> bool {
    let mut m: Option<usize> = None;
    
    for i in (0..input_vector.len()).rev() {
        if !input_vector[i] {  
            m = Some(i);
            break;
        }
    }
    
    match m {
        None => {
            true
        }
        Some(pos) => {
            input_vector[pos] = true;
            
            for j in pos+1..input_vector.len() {
                input_vector[j] = false;
            }
            
            false
        }
    }
}

fn print_binary_vector(vec: &[bool]) {
    for &b in vec {
        print!("{} ", if b { "1" } else { "0" });
    }
    println!();
}

fn main() {
    let n = 3;
    let mut vector = vec![false; n];  
        
    loop {
        print_binary_vector(&vector);
        
        if generation_of_next_nbit_binary_vector(&mut vector) {
            break;  
        }
    }
}



