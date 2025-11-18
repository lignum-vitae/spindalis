use spindalis::eigen::power_method;
use spindalis::utils::{Arr2D, Rounding};

fn main() {
    let matrix = Arr2D::from(&[[2.0, 8.0, 10.0], [8.0, 4.0, 5.0], [10.0, 5.0, 7.0]]);
    println!("Original Matrix:\n{matrix}\n");
    let result = power_method(&matrix, 1e-10);
    match result {
        Ok((value, vector)) => println!(
            "Largest Eigenvalue = {value:.4}\nAssociated Eigenvector:\n{}\n",
            vector.round_to_decimal(5)
        ),
        Err(e) => eprintln!("{e:?}"),
    }
    let inverse_matrix = match matrix.inverse() {
        Ok(inverted) => inverted,
        Err(e) => {
            eprintln!("Unable to invert matrix: {e:?}");
            Arr2D::from(&[[]])
        }
    };
    if !inverse_matrix.is_empty() {
        let result = power_method(&inverse_matrix, 1e-10);
        match result {
            Ok((value, vector)) => println!(
                "Smallest Eigenvalue = {:.4}\nAssociated Eigenvector:\n{}",
                1_f64 / value,
                vector.round_to_decimal(5),
            ),
            Err(e) => eprintln!("{e:?}"),
        }
    }
}
