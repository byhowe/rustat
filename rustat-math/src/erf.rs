const P: f64 = 0.47047;
const A1: f64 = 0.3480242;
const A2: f64 = -0.0958798;
const A3: f64 = 0.7478556;

pub fn erf(x: f64) -> f64
{
    let t = 1.0 / (1.0 + P * x.abs());
    let y = 1.0 - (A1 * t + A2 * t.powi(2) + A3 * t.powi(3)) * (-x.powi(2)).exp();
    if x.is_sign_positive() {
        y
    } else {
        -y
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn erf_tests()
    {
        let vectors = vec![
            (-1.192007, -0.908156),
            (-0.416572, -0.444200),
            (-1.414748, -0.954564),
            (-0.800789, -0.742591),
            (1.006098, 0.845233),
            (-0.893251, -0.793521),
            (-0.559721, -0.571381),
            (0.673620, 0.659240),
            (-0.267749, -0.295042),
            (1.054527, 0.864136),
        ];

        let diff = vectors
            .iter()
            .map(|(x, y)| y - erf(*x))
            .collect::<Vec<f64>>();
        let pass = diff.iter().filter(|x| **x > 1e-6).next().is_none();
        assert!(
            pass,
            "ERF values are not within acceptable range: {:#?}",
            diff
        );
    }
}
