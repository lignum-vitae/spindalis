use crate::utils::StdDevType;

pub fn arith_mean(samples: &[f64]) -> f64 {
    let n = samples.len();
    if n == 0 {
        return f64::NAN;
    }
    samples.iter().sum::<f64>() / n as f64
}

pub fn geom_mean(samples: &[f64]) -> f64 {
    let n = samples.len();
    if n == 0 {
        return f64::NAN;
    }
    samples.iter().product::<f64>().powf(1_f64 / n as f64)
}

pub fn std_dev(samples: &[f64], correction: StdDevType) -> f64 {
    let n = samples.len();
    let denomiator = match correction {
        StdDevType::Poulation => n,
        StdDevType::Sample => n.saturating_sub(1),
    };
    if denomiator == 0 {
        return f64::NAN;
    }
    let mean = arith_mean(samples);
    let variance = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / denomiator as f64;

    variance.sqrt()
}
