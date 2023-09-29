

#[derive(Debug)]
pub struct Polynomial {
    pub coefficients : Vec<f64>
}


impl Polynomial {
    pub fn new(coeff: Vec<f64>) -> Self {
        Polynomial {coefficients: coeff}
    }
    // evaluation using horner method in O(N) time complexity.
    pub fn evaluate(&self, x: &f64) -> f64 {
        let mut result: f64 = 0.0;
        for(degree, &coeff) in self.coefficients.iter().enumerate() {
            result += coeff*x.powi(degree as i32);
        }
        result
    }

    pub fn addition(&self, other: &Polynomial) -> Polynomial {

        // padding with zeroes
        let max_degree = self.coefficients.len().max(other.coefficients.len());
        let mut res = vec![0.0; max_degree];

        for(i, &coeffs) in self.coefficients.iter().enumerate() {
            res[i] += coeffs;
        }

        for(i, &coeffs) in other.coefficients.iter().enumerate() {
            res[i] += coeffs;
        }

        Polynomial { coefficients: res }
    }

    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let mut result_coefficients = vec![0.0; self.coefficients.len() + other.coefficients.len() - 1];

        for (i, &coeff1) in self.coefficients.iter().enumerate() {
            for (j, &coeff2) in other.coefficients.iter().enumerate() {
                result_coefficients[i + j] += coeff1 * coeff2;
            }
        }

        Polynomial::new(result_coefficients)
    }


    



    
}
