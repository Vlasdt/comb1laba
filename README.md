

G = (V, E)

где

- V — множество вершин
- E — множество рёбер

Граф задаётся **матрицей смежности**.

Каждой вершине `v_i` сопоставлена стоимость `c_i`.

Необходимо найти такое подмножество вершин `S`, что

- каждая вершина графа либо принадлежит `S`
- либо смежна хотя бы с одной вершиной из `S`

и при этом суммарная стоимость

C(S) = Σ c_i

минимальна.

---

# Представление решения

Каждое подмножество вершин кодируется **бинарным вектором**


x = (x1, x2, ..., xn)


где


xi = 1 → вершина выбрана
xi = 0 → вершина не выбрана


Количество всех возможных подмножеств:


2^n


---

# Алгоритмы генерации бинарных векторов

## 1. Лексикографическая генерация

Бинарный вектор рассматривается как **двоичное число**.  
Следующий вектор получается операцией инкремента.

### Пример


000
001
010
011
100
101
110
111


### Псевдокод


Algorithm NextBinaryVector(v)

Input: binary vector v of length n

m ← index of the rightmost 0 in v

if such position does not exist

return LAST_VECTOR

v[m] ← 1

for j ← m+1 to n

v[j] ← 0

return v


### Генерация всех векторов


Algorithm GenerateAllBinaryVectors(n)

v ← (0,0,...,0)

add v to result

while true

v ← NextBinaryVector(v)
if v is LAST_VECTOR
    break
add v to result

return result


---

## 2. Рекурсивный код Грея

**Код Грея** — последовательность бинарных векторов, где соседние векторы отличаются **ровно одним битом**.

Код Грея для `n` строится из кода для `n-1`:

1. к каждому вектору добавляется `0`
2. затем к векторам в обратном порядке добавляется `1`

### Пример


n = 2

00
10
11
01


### Псевдокод


Algorithm GrayRecursive(n)

if n = 1

return {0,1}

prev ← GrayRecursive(n-1)

result ← empty list

for each vector v in prev

append (v,0) to result

for each vector v in reverse(prev)

append (v,1) to result

return result


---

## 3. Нерекурсивный код Грея

Используется формула


G(i) = i XOR (i >> 1)


где

- `>>` — побитовый сдвиг
- `XOR` — исключающее ИЛИ

### Псевдокод


Algorithm GrayNonRecursive(n)

for i ← 0 to 2^n − 1

g ← i XOR (i >> 1)
convert g to binary vector
add vector to result

return result


---

# Проверка покрытия графа

Для каждого бинарного вектора проверяется, покрывает ли он граф.

Вершина считается покрытой, если

- она выбрана
- или смежна с выбранной вершиной

### Псевдокод


Algorithm CheckCover(matrix, vector)

covered ← array of false

for i ← 1..n

if vector[i] = 1
    for j ← 1..n
        if matrix[i][j] = 1
            covered[j] ← true

if all vertices covered

return true

else

return false


---

# Полный перебор решений


Algorithm SolveCover(matrix, combinations, cost_vector)

best_cost ← +∞

best_solutions ← empty list

for each vector v in combinations

if CheckCover(matrix, v)
    cost ← Cost(v)
    if cost < best_cost
        best_cost ← cost
        best_solutions ← {v}
    else if cost = best_cost
       add v to best_solutions

return best_solutions


---

# Оценка сложности

## Генерация бинарных векторов

Количество векторов:


2^n


Время:


O(n · 2^n)


Память:


O(n · 2^n)


---

## Проверка покрытия

При использовании матрицы смежности:


O(n^2)


---

## Полный перебор

Для каждого из `2^n` векторов выполняется проверка покрытия и вычисление стоимости.

Итоговая сложность:


O(2^n · n^2)


- нерекурсивный код Грея

Все алгоритмы используются для **полного перебора решений задачи минимального покрытия графа**.
