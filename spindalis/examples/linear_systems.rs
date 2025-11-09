use spindalis::solvers::gaussian_elimination;

fn main() {
    let coeff_matrix = vec![
        vec![8.0, 2.0, -2.0],
        vec![10.0, 2.0, 4.0],
        vec![12.0, 2.0, 2.0],
    ];

    let mut rhs_vector = vec![8.0, 16.0, 16.0];
    let tol = 1e-12;

    let solution = gaussian_elimination(&coeff_matrix, &mut rhs_vector, tol).unwrap();
    println!("Solution:");
    for (i, sol) in solution.iter().enumerate() {
        print!("x{} = {sol}", i + 1);
        if i != solution.len() - 1 {
            print!(", ")
        }
    }
}
