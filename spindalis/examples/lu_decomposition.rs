use spindalis::decomposition::{lu_decomposition, lu_pivot_decomposition};
use spindalis::utils::Arr2D;

fn main() {
    let mat = Arr2D::from(&[[2, -1, -2], [-4, 6, 3], [-4, -2, 8]]);
    // LU Decomposition
    let (lower, upper) = lu_decomposition(&mat).unwrap();
    println!("LU Decomposition");
    println!("Lower Triangle Matrix:\n{lower}\nUpper Triange Matrix:\n{upper}\n");

    // PLU Decomposition
    let (lower, upper, permutation) = lu_pivot_decomposition(&mat).unwrap();
    println!("LU Decomposition with partial pivoting (PLU decomposition)");
    println!(
        "Lower Triange Matrix:\n{lower}\nUpper Triange Matrix:\n{upper}\nPermutation matrix:\n{permutation}"
    );
}
