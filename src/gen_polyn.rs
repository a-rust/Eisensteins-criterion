use rand::Rng;
use std::io;

// Function that sets the degree of the randomly generated polynomial
pub fn choose_degree() -> i32 {
    let mut input = String::new();
    println!(
        "Please choose the degree (positive integer) 
of the polynomial, or enter x to generate 
a random degree from 1-10."
    );
    io::stdin().read_line(&mut input);
    if input.trim() == "x" {
        let rand_deg = gen_degree();
        return rand_deg;
    } else {
        let degree: i32 = input
            .trim()
            .parse()
            .expect("Please type in an integer (ideally from 1-10).");
        println!("Degree of polynomial p(x) is set to {}", degree);
        return degree;
    }
}
// Function that sets the upper bound of the coefficients for the polynomial
pub fn choose_upper_bound() -> i32 {
    println!(
        "Please choose the upper bound 
on the rng range for all coefficients, 
or enter x set the default to 100."
    );
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    if input.trim() == "x" {
        println!("Upper bound is set to 100");
        return 100;
    } else {
        let user_upper_bound: i32 = input
            .trim()
            .parse()
            .expect("Please type in an integer (ideally from 1-10).");
        println!("Upper bound of coefficients is set to {}", user_upper_bound);
        return user_upper_bound;
    }
}

// Function that sets a random degree if the user doesn't explicitly give a value
pub fn gen_degree() -> i32 {
    let mut degree = rand::thread_rng();
    let set_degree: i32 = degree.gen_range(2..10);
    println!("Degree of polynomial p(x) is set to {}", set_degree);
    set_degree
}

// Function that returns the coefficients of the randomly generated polynomial as a vector
pub fn gen_polynomial(deg: i32, upper_bound: i32) -> Vec<i32> {
    let mut vec_of_coef: Vec<i32> = Vec::<i32>::new();
    for _i in 1..deg + 2 {
        vec_of_coef.push(rand::thread_rng().gen_range(1..upper_bound));
    }
    println!(
        "Polynomial coefficients (a_0X + a_1X + ... + a_{}X): 
{:?}", deg, vec_of_coef
    );
    vec_of_coef
}

// Getter method that returns the highest coefficient within the vector of coefficients
pub fn get_highest_coef(full_vec: &Vec<i32>) -> i32 {
    let mut initial_highest_coef: i32 = full_vec[0];
    for i in 1..full_vec.len() {
        if full_vec[i] > (initial_highest_coef) {
                initial_highest_coef = full_vec[i];
            }
    }
    return initial_highest_coef;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_highest_coef() {
        assert_eq!(get_highest_coef(&vec![-100, -10, 0, 20, 30, 40, 99]), 99);
    }
}
