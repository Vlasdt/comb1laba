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

// возвращает матрицу без крайних строк и столбцов
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

fn generation_of_next_nbit_binary_vector(input_vector: &mut Vec<bool>) -> bool { // n
    """
    вход: указатель на bool вектор
         в векторе на входном указателе делает следующий вектор
    выход: true если входной вектор последний иначе false

    """
    let mut m: Option<usize> = None;
    
    for i in (0..input_vector.len()).rev() { // n
        if !input_vector[i] {  
            m = Some(i);
            break;
        }
    }
    match m { //типа switch case 
        None => {
            true
        }
        Some(pos) => { 
            input_vector[pos] = true; 
            
            for j in pos+1..input_vector.len() { // n
                input_vector[j] = false;
            }
            
            false
        }
    }
}

fn generation_of_all_next_nbit_binary_vectors(n:usize) -> Vec<Vec<bool>> {// n * 2 ^n
    let mut all_vectors: Vec<Vec<bool>> = Vec::new();
    let input_vector = vec![false; n];  

    all_vectors.push(input_vector.clone());
    
    loop {
        let mut current = all_vectors.last().unwrap().clone();
        
        if generation_of_next_nbit_binary_vector(&mut current) {
            break;  
        }
        
        all_vectors.push(current);
    }
    
    all_vectors  
}

fn print_binary_vector(vec: &[bool]) {
    for &b in vec {
        print!("{} ", if b { "1" } else { "0" });
    }
    println!();
}

fn grey_recursive(n: usize) -> Vec<Vec<bool>> { //2^n
    """
    вход: размер целевых векторов

    выход вектор всех векторов 


    """
    if n == 1 {
        return vec![vec![false], vec![true]];
    }
    
    let prev = grey_recursive(n - 1);
    let mut result = Vec::with_capacity(2_usize.pow(n as u32));
    
    for vec in &prev { //2 ^ n
        let mut new_vec = vec.clone();
        new_vec.push(false);
        result.push(new_vec);
    }
    
    for vec in &prev {// 2 ^ n
        let mut new_vec = vec.clone();
        new_vec.push(true);
        result.push(new_vec);
    }
    
    result
}


/*



*/
fn grey_nonrecursive(n: usize) -> Vec<Vec<bool>> { //2^n
    """
    вход: размер целевых векторов

    выход вектор всех векторов 
    

    """
    let mut result = Vec::with_capacity(2_usize.pow(n as u32));
    let mut v = vec![false; n];  //  000...0
    
    result.push(v.clone()); 
    
    for i in 1..(1 << n) {  // (1 << n) = 2^n 

        let  gray_bit:i32 = (i & (i - 1)) ^ i;  // i & -i
        
        let pos = gray_bit.trailing_zeros() as usize; // количество младших нулевых битов
        
        v[pos] = !v[pos];
        
        result.push(v.clone());
    }
    
    result
}


fn check_coating(matrix: &Vec<Vec<bool>>, vector: &Vec<bool>) -> bool { 
    let n = vector.len();
    let mut covered = vec![false; n];
    
    for i in 0..n {
        if vector[i] {
            for j in 0..n {
                if matrix[i][j] {
                    covered[j] = true;
                }
            }
        }
    }
        covered.iter().all(|&c| c)
}

fn cost_function(vector:&Vec<bool>, c:&Vec<i32>) -> i32{
    let mut result:i32 = 0;
    for i in 0..vector.len(){
        if vector[i] {
            result += c[i];
        }
    }
    result
}


/*fn check_full_coating_all_vectors(matrix:&Vec<Vec<bool>>, coombinations:&Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let mut result: Vec<Vec<bool>> = Vec::new();
    for a in coombinations.iter(){
        if check_coating(matrix,a) {
            result.push(a.to_vec());
        }
    }
    result
}
*/
fn check_full_coating(matrix: &Vec<Vec<bool>>, combinations: &Vec<Vec<bool>>, c: &Vec<i32>) -> Vec<Vec<bool>> {
    let mut best_solutions: Vec<Vec<bool>> = Vec::new();
    let mut min_cost = i32::MAX;
    
    for candidate in combinations {
        if check_coating(matrix, candidate) {
            let current_cost = cost_function(candidate, c);
            
            if current_cost < min_cost {
                min_cost = current_cost;
                best_solutions.clear();
                best_solutions.push(candidate.clone());
            } else if current_cost == min_cost {
                best_solutions.push(candidate.clone());
            }
        }
    }
    
    best_solutions
}










fn main() {
    let n:usize = 4;
    let  c = vec![(1) as i32; n];  //  111...1
    let first = generation_of_all_next_nbit_binary_vectors(n);
    let matrix = create_adjacency_matrix(n);
    let result = check_full_coating(&matrix,&first,&c);


    //print_matrix(&result);

    for a in result.iter(){
        print_binary_vector(a);
    }
    println!();
    println!("{}",cost_function(&result[0],&c));
    println!();
    print_matrix(&matrix);


    /*
    
    
    let first = generation_of_all_next_nbit_binary_vectors(n);
    let second = grey_recursive(n);
    let third = grey_nonrecursive(n);


    println!("Первый адлгоритм n = {}:", n);
    for (i, vec) in first.iter().enumerate() {
                print_binary_vector(vec);
    }


    println!("второй адлгоритм n = {}:", n);
    for (i, vec) in second.iter().enumerate() {
                print_binary_vector(vec);
    }


    println!("третий адлгоритм n = {}:", n);
    for (i, vec) in third.iter().enumerate() {
                print_binary_vector(vec);
    }


    */






}



