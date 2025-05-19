fn main() {
    let dimension_data = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let points = vec![5.0, 11.0, 23.0, 87.0, 99.0];
    dbg!(center_data(&dimension_data));
    dbg!(variance(&points));
}

fn center_data(data: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let new_data: Vec<Vec<f64>> = data
        .iter()
        .map(|dimension| {
            let mean = dimension.iter().sum::<f64>() / dimension.len() as f64;
            dimension.iter().map(|&x| x - mean).collect()
        })
        .collect();
    return new_data;
}

fn variance(data: &Vec<f64>) -> f64 {
    let length = data.len() as f64;
    let new_data: Vec<f64> = data
        .iter()
        .map(|x| {
            let mean: f64 = data.iter().sum::<f64>() / length;
            let base: f64 = x - mean;
            base.powi(2)
        })
        .collect();
    let variance: f64 = new_data.iter().sum::<f64>() / (length - 1.0);
    return variance;
}
