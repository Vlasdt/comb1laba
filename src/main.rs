
/// Создает матрицу смежности для графа специального вида.
/// 
/// # Аргументы
/// * `dim` - размерность исходной матрицы (количество вершин графа)
/// 
/// # Возврат
/// * `Vec<Vec<bool>>` - квадратная матрица смежности размером dim x dim,
///   где true означает наличие ребра между вершинами
/// 
/// # Особенности построения графа
/// * Вершины нумеруются с 1 по dim (внутренняя нумерация)
/// * Добавляются ребра: (i,i) - петли
/// * Добавляются ребра: (i,i+1) и (i+1,i) - соединение соседних вершин
/// * Если dim >= 4, добавляются ребра: (i,i+3) и (i+3,i) - соединение через 2 вершины
/// * Крайние строки и столбцы (индексы 0 и dim+1) удаляются в конце
fn create_adjacency_matrix(dim: usize) -> Vec<Vec<bool>> {
    // Создаем матрицу размером (dim+2) x (dim+2), заполненную false
    // Добавляем 2 для создания "рамки" вокруг рабочей области
    let mut matrix = vec![vec![false; dim + 2]; dim + 2];
    
    // Заполняем рабочую область (индексы с 1 по dim)
    for i in 1..=dim {
        matrix[i][i] = true;          // Петля для каждой вершины

        if i < dim {
            matrix[i][i+1] = true;     // Ребро к следующей вершине
            matrix[i+1][i] = true;     // Симметричное ребро (граф неориентированный)
        }
    }
    
    // Добавляем дополнительные ребра для вершин, отстоящих на 3 позиции
    if dim >= 4 {
        for i in 1..=dim-3 {
            matrix[i][i+3] = true;      // Ребро от i к i+3
            matrix[i+3][i] = true;      // Симметричное ребро
        }
    }

    // Удаляем "рамку" и возвращаем итоговую матрицу
    get_matrix(&matrix) 
}

/// Удаляет крайние строки и столбцы из матрицы.
/// 
/// # Аргументы
/// * `source_matrix` - исходная матрица с "рамкой" (крайними строками/столбцами)
/// 
/// # Возврат
/// * `Vec<Vec<bool>>` - матрица без первой и последней строки/столбца
/// 
/// # Предположения
/// * Исходная матрица имеет размер как минимум 2x2
/// * Все строки имеют одинаковую длину
fn get_matrix(source_matrix: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let rows = source_matrix.len() - 2;      // Количество строк без рамки
    let cols = source_matrix[0].len() - 2;   // Количество столбцов без рамки
    let mut matrix = vec![vec![false; cols]; rows];
    
    // Копируем внутреннюю часть матрицы (без рамки)
    for i in 1..source_matrix.len() - 1 {
        for j in 1..source_matrix[i].len() - 1 {
            matrix[i - 1][j - 1] = source_matrix[i][j];
        }
    }
    matrix
}

/// Выводит матрицу в консоль в виде 0 и 1.
/// 
/// # Аргументы
/// * `matrix` - ссылка на матрицу для вывода
/// 
/// # Формат вывода
/// * "1" для true, "0" для false
/// * Каждая строка на отдельной строке
fn print_matrix(matrix: &Vec<Vec<bool>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!("{} ", if matrix[i][j] { "1" } else { "0" });
        }
        println!();
    }
}

/// Генерирует следующий двоичный вектор в лексикографическом порядке.
/// 
/// # Аргументы
/// * `input_vector` - изменяемая ссылка на текущий вектор
/// 
/// # Возврат
/// * `bool` - true если текущий вектор был последним (все единицы), иначе false
/// 
/// # Принцип работы
/// * Находит самую правую позицию с false (0) и меняет её на true
/// * Все позиции правее неё сбрасывает в false
/// * Аналог инкремента двоичного числа
fn generation_of_next_nbit_binary_vector(input_vector: &mut Vec<bool>) -> bool {
    let mut m: Option<usize> = None;
    
    // Ищем самую правую позицию с false (0)
    for i in (0..input_vector.len()).rev() {
        if !input_vector[i] {  
            m = Some(i);
            break;
        }
    }
    
    match m { // Обработка результата поиска
        None => {
            // Если нет ни одного false, значит вектор из всех единиц
            true  // Сигнализируем, что это последний вектор
        }
        Some(pos) => { 
            // Меняем найденный false на true
            input_vector[pos] = true; 
            
            // Все биты справа от позиции сбрасываем в false
            for j in pos+1..input_vector.len() {
                input_vector[j] = false;
            }
            
            false  // Есть еще векторы
        }
    }
}

/// Генерирует все возможные двоичные векторы заданной длины.
/// 
/// # Аргументы
/// * `n` - длина векторов
/// 
/// # Возврат
/// * `Vec<Vec<bool>>` - вектор, содержащий все 2^n возможных комбинаций
/// 
/// # Алгоритмическая сложность
/// * O(n * 2^n) по времени
/// * O(2^n) по памяти
fn generation_of_all_next_nbit_binary_vectors(n: usize) -> Vec<Vec<bool>> {
    let mut all_vectors: Vec<Vec<bool>> = Vec::new();
    let input_vector = vec![false; n];  // Начинаем с вектора из всех нулей

    all_vectors.push(input_vector.clone());
    
    loop {
        let mut current = all_vectors.last().unwrap().clone();
        
        // Генерируем следующий вектор
        if generation_of_next_nbit_binary_vector(&mut current) {
            break;  // Если это был последний, выходим из цикла
        }
        
        all_vectors.push(current);
    }
    
    all_vectors  
}

/// Выводит двоичный вектор в консоль.
/// 
/// # Аргументы
/// * `vec` - срез вектора для вывода
/// 
/// # Формат вывода
/// * "1" для true, "0" для false, разделенные пробелами
fn print_binary_vector(vec: &[bool]) {
    for &b in vec {
        print!("{} ", if b { "1" } else { "0" });
    }
    println!();
}

/// Рекурсивно генерирует код Грея заданной длины.
/// 
/// # Аргументы
/// * `n` - размер целевых векторов
/// 
/// # Возврат
/// * `Vec<Vec<bool>>` - все 2^n векторов кода Грея
/// 
/// # Принцип работы
/// * Код Грея для n получается из кода для n-1:
///   1. Добавляем 0 к каждому вектору из предыдущего кода
///   2. Добавляем 1 к каждому вектору из предыдущего кода в обратном порядке
fn grey_recursive(n: usize) -> Vec<Vec<bool>> {
    // Базовый случай: для n=1 возвращаем [0, 1]
    if n == 1 {
        return vec![vec![false], vec![true]];
    }
    
    // Рекурсивно получаем код Грея для n-1
    let prev = grey_recursive(n - 1);
    let mut result = Vec::with_capacity(2_usize.pow(n as u32));
    
    // Добавляем 0 к каждому вектору (первая половина)
    for vec in &prev {
        let mut new_vec = vec.clone();
        new_vec.push(false);
        result.push(new_vec);
    }
    
    // Добавляем 1 к каждому вектору в обратном порядке (вторая половина)
    for vec in prev.iter().rev() {
        let mut new_vec = vec.clone();
        new_vec.push(true);
        result.push(new_vec);
    }
    
    result
}

/// Нерекурсивно генерирует код Грея заданной длины.
/// 
/// # Аргументы
/// * `n` - размер целевых векторов
/// 
/// # Возврат
/// * `Vec<Vec<bool>>` - все 2^n векторов кода Грея
/// 
/// # Принцип работы
/// * Использует свойство: i-й код Грея получается из i XOR (i >> 1)
/// * Более эффективно по памяти, чем рекурсивный метод
fn grey_nonrecursive(n: usize) -> Vec<Vec<bool>> {
    let mut result = Vec::with_capacity(2_usize.pow(n as u32));
    let mut v = vec![false; n];  // Начинаем с вектора 000...0
    
    result.push(v.clone()); 
    
    for i in 1..(1 << n) {  // (1 << n) = 2^n, проходим все числа
        // Находим бит, который нужно инвертировать для получения следующего кода Грея
        // gray_bit = i & -i дает степень двойки - позицию изменяемого бита
        let gray_bit: i32 = (i & (i - 1)) ^ i;  // или i & -i
        
        // Получаем позицию бита (количество младших нулей)
        let pos = gray_bit.trailing_zeros() as usize;
        
        // Инвертируем бит на найденной позиции
        v[pos] = !v[pos];
        
        result.push(v.clone());
    }
    
    result
}

/// Проверяет, покрывает ли выбранное подмножество вершин все вершины графа.
/// 
/// # Аргументы
/// * `matrix` - матрица смежности графа
/// * `vector` - двоичный вектор, где vector[i] = true означает, что вершина i выбрана
/// 
/// # Возврат
/// * `bool` - true если каждая вершина либо выбрана, либо смежна с выбранной
/// 
/// # Принцип работы
/// * Для каждой выбранной вершины помечаем её саму и всех соседей как покрытые
/// * Проверяем, все ли вершины покрыты
fn check_coating(matrix: &Vec<Vec<bool>>, vector: &Vec<bool>) -> bool {
    let n = vector.len();
    let mut covered = vec![false; n];
    
    // Для каждой выбранной вершины
    for i in 0..n {
        if vector[i] {
            // Помечаем все смежные вершины как покрытые
            for j in 0..n {
                if matrix[i][j] {
                    covered[j] = true;
                }
            }
        }
    }
    
    // Проверяем, все ли вершины покрыты
    covered.iter().all(|&c| c)
}

/// Вычисляет стоимость выбранного подмножества вершин.
/// 
/// # Аргументы
/// * `vector` - двоичный вектор выбранных вершин
/// * `c` - вектор стоимостей каждой вершины
/// 
/// # Возврат
/// * `i32` - суммарная стоимость выбранных вершин
fn cost_function(vector: &Vec<bool>, c: &Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for i in 0..vector.len() {
        if vector[i] {
            result += c[i];
        }
    }
    result
}

/// Находит оптимальные решения задачи о покрытии с минимальной стоимостью.
/// 
/// # Аргументы
/// * `matrix` - матрица смежности графа
/// * `combinations` - все возможные подмножества вершин
/// * `c` - вектор стоимостей вершин
/// 
/// # Возврат
/// * `Vec<Vec<bool>>` - все решения с минимальной стоимостью
/// 
/// # Принцип работы
/// * Перебирает все комбинации, проверяет их на покрытие
/// * Находит минимальную стоимость среди покрывающих
/// * Возвращает все решения с этой минимальной стоимостью
fn check_full_coating(matrix: &Vec<Vec<bool>>, combinations: &Vec<Vec<bool>>, c: &Vec<i32>) -> Vec<Vec<bool>> {
    let mut best_solutions: Vec<Vec<bool>> = Vec::new();
    let mut min_cost = i32::MAX;
    
    for candidate in combinations {
        if check_coating(matrix, candidate) {
            let current_cost = cost_function(candidate, c);
            
            if current_cost < min_cost {
                // Нашли лучшее решение
                min_cost = current_cost;
                best_solutions.clear();
                best_solutions.push(candidate.clone());
            } else if current_cost == min_cost {
                // Нашли альтернативное решение с той же стоимостью
                best_solutions.push(candidate.clone());
            }
        }
    }
    
    best_solutions
}

/// Главная функция программы.
/// 
/// Демонстрирует решение задачи о минимальном покрытии для графа специального вида:
/// 1. Создает граф размером 4
/// 2. Задает одинаковые стоимости для всех вершин
/// 3. Генерирует все возможные подмножества вершин
/// 4. Находит минимальные покрытия
/// 5. Выводит результаты
fn main() {
    let n: usize = 4;                    // Количество вершин
    let c = vec![(1) as i32; n];          // Стоимость каждой вершины = 1
    
    // Генерируем все возможные подмножества вершин
    let first = generation_of_all_next_nbit_binary_vectors(n);
    
    // Создаем матрицу смежности
    let matrix = create_adjacency_matrix(n);
    
    // Находим оптимальные решения
    let result = check_full_coating(&matrix, &first, &c);

    // Выводим все найденные оптимальные решения
    for a in result.iter() {
        print_binary_vector(a);
    }
    println!();
    
    // Выводим стоимость первого решения (все решения имеют одинаковую стоимость)
    println!("{}", cost_function(&result[0], &c));
    println!();
    
    // Выводим матрицу смежности
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



