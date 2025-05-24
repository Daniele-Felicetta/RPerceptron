pub fn scalar_product(x:Vec<i32>, y:Vec<i32>) -> Vec<i32>{
    let mut result = Vec::new();
    for (index,x_i) in x.iter().enumerate() {
        let y_i = y[index];

        result[index] = x_i * y_i;
    }  

    return result;
}


pub fn flatVec(matrix:Vec<[i32;2]>) -> Vec<i32> {
    let mut flat = Vec::new();

    for row in matrix {
        for val in row {
            flat.push(val)
        }
    }
    return flat;
} 