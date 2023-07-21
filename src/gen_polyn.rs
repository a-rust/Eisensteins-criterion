use rand::Rng;
use std::io;

//Giving the user the option to set the degree of the polynomial
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
//Giving the user the option to set the upper bound on the rng for the coefficients of the polynomial.
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

//Generating the degree if the user does not want to set it themselves.
pub fn gen_degree() -> i32 {
    let mut degree = rand::thread_rng();
    let set_degree: i32 = degree.gen_range(2..10);
    println!("Degree of polynomial p(x) is set to {}", set_degree);
    set_degree
}

//Randomly generating the n coefficients of p(x) into a vector
pub fn gen_polynomial(deg: i32, upper_bound: i32) -> Vec<i32> {
    let mut vec_of_coef: Vec<i32> = Vec::<i32>::new();
    for _i in 1..deg + 1 {
        vec_of_coef.push(rand::thread_rng().gen_range(1..upper_bound));
    }
    println!(
        "Polynomial coefficients (a_0, a_1, ... , a_n): 
{:?}",
        vec_of_coef
    );
    vec_of_coef
}

// Getter method that returns the highest coefficient within the vector of coefficients
pub fn get_highest_coef(full_vec: Vec<i32>) -> i32 {
    let mut initial_highest_coef: i32 = full_vec[0];
    for i in 1..full_vec.len() {
        if i > (initial_highest_coef as usize) {
                initial_highest_coef = full_vec[i];
            }
    }
    return initial_highest_coef;
}

//Creating a vector of first n-1 coefficients of p(x)
pub fn first_n_minus_one_coef(full_vec: Vec<i32>, deg: i32) -> Vec<i32> {
    let middle_coef: Vec<i32> = full_vec
        .clone()
        .into_iter()
        .take(deg as usize - 1)
        .collect();
    println!("First n-1 coefficients: {:?}", middle_coef);
    middle_coef
}
