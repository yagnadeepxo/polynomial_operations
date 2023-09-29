
mod polynomial;

use polynomial::Polynomial;

fn main(){
    let poly1 = Polynomial::new(vec![1.0, 2.0, 3.0]);
    let x = 5.0;
    let ans = poly1.evaluate(&x);
    println!("{}", ans);

    let poly2 = Polynomial::new(vec![1.0, 2.0, 3.0]); // 1 + 2x + 3x^2
    let poly3 = Polynomial::new(vec![4.0, 5.0, 6.0]); // 4 + 5x + 6x^2

    // Perform polynomial multiplication using the O(n^2) approach.
    let result = poly2.multiply(&poly3);

    println!("Result of multiplication: {:?}", result.coefficients);

    let poly4 = Polynomial::new(vec![6.0, 7.2, 8.6]);
    let poly5 = Polynomial::new(vec![6.7, 8.9, 3.0]);

    let res = poly4.addition(&poly5);
    println!("the addition of 2 polynomials: {:?}", res.coefficients);
}
