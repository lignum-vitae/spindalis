use spindalis::regressors::LinearRegressor;
use spindalis::regressors::{
    GradientDescentRegression, LeastSquaresRegression, PolynomialRegression,
};

fn main() {
    let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let y: Vec<f64> = vec![1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 8.0, 10.0, 13.0];

    let grad_descent = GradientDescentRegression {
        steps: 10000,
        step_size: 0.01,
    };

    let model = grad_descent.fit(&x, &y);

    // Gradient Descent
    println!("Linear Regression using Gradient Descent");
    println!(
        "Slope = {:.2}\nIntercept = {:.2}",
        model.slope().unwrap(),
        model.intercept()
    );
    println!(
        "Standard Error = {:.3}\nR2 Score = {:.3}",
        model.std_err, model.r2
    );
    println!(
        "The polynomial for the gradient descent regression line is {}\n",
        model.to_polynomial_string()
    );

    // Least Squares
    println!("Linear Regression using Least Squares");
    let least_squares = LeastSquaresRegression;

    let model = least_squares.fit(&x, &y);

    println!(
        "Slope = {:.2}\nIntercept = {:.2}",
        model.slope().unwrap(),
        model.intercept()
    );
    println!(
        "Standard Error = {:.3}\nR2 Score = {:.3}",
        model.std_err, model.r2
    );
    println!(
        "The polynomial for the least squares regression line is {}\n",
        model.to_polynomial_string()
    );

    // Polynomial Regression
    println!("Linear regression using Polynomial Regression");
    let poly_regression = PolynomialRegression { order: 2 };

    let model = poly_regression.fit(&x, &y);

    let slopes = model.slopes().unwrap();
    for (i, slope) in slopes.iter().enumerate() {
        println!("The slope for x^{} is {slope:.2}", i + 1)
    }
    println!("Intercept = {:.2}", model.intercept());
    println!(
        "Standard Error = {:.3}\nR2 Score = {:.3}",
        model.std_err, model.r2
    );
    println!(
        "The polynomial for the second order polynomial regression line is {}\n",
        model.to_polynomial_string()
    );
}
