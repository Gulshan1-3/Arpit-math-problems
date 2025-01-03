
fn can_unlock(keys:Vec<Vec<f64>>,tolereance:f64) -> bool {
    let rows = keys.len();
    if rows == 0 {
        return false;
    }

    let cols = keys[0].len();
    if cols == 0 {
        return false; 
    }

    let mut matrix = keys.clone();

    let mut rank  = 0;
    for col in 0..cols {
        let mut pivot = None;
        for row in rank..rows{
            if matrix[row][col].abs() > tolereance{
                pivot = Some(row);
                break;
            }

        }
        if pivot.is_none() {
            continue;
        }

     let pivot = pivot.unwrap();
     matrix.swap(rank, pivot);
     let pivot_value = matrix[rank][col];
     for c in col..cols {
        matrix [rank][c] /= pivot_value;
     }
     for row in 0..rows {
        if row!= rank {
            let factor = matrix[row][col];
            for c in col..cols {
                matrix[row][c] -= factor * matrix[rank][c];
            }
        }
     }
     rank+=1;
    }
  rank == cols 
}


    
fn main() {
    let tolerance = 1e-9; // Tolerance for floating-point comparison

    let keys1 = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    let keys2 = vec![
        vec![2.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![4.0, 4.0, 0.0],
    ];
    let keys3 = vec![
        vec![1.000000001, 0.0, 0.0],
        vec![0.0, 1.000000001, 0.0],
        vec![0.0, 0.0, 1.000000001],
    ];

    let keys4 = vec![
        vec![2.0,1.0,3.0],
        vec![-1.0,4.0,0.0],
        vec![0.0,-2.0,1.0],
    ];

    println!("Keys1 can unlock: {}", can_unlock(keys1, tolerance)); // Should print true
    println!("Keys2 can unlock: {}", can_unlock(keys2, tolerance)); // Should print false
    println!("Keys3 can unlock: {}", can_unlock(keys3, tolerance)); // Should print true
    println!("Keys3 can unlock: {}", can_unlock(keys4, tolerance)); //Should print true 
}





