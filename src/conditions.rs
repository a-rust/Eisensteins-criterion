use crate::gen_polyn::get_highest_coef;



/// Finding all prime numbers within a given range of possible coefficients
/// We only need to check these prime numbers moving forward
fn _get_primes_within_poly(highest_coef: i32) -> Vec<i32> {
    let poly_coef_range: Vec<i32> = (1..=highest_coef).collect();
    let mut primes: Vec<i32> = Vec::new();
    // Logic for finding all primes within the range (1, highest_coef]
    for i in poly_coef_range {
        for j in 2..i + 1 {
            if j == i {
                primes.push(i);
            }
            if i % j == 0 {
                break;
            }
        }
    }
    primes
}

fn _first_condition(middle: Vec<i32>) -> Vec<i32> {
    let mut prime_divisors = Vec::<i32>::new();
    let highest_coef = get_highest_coef(middle.clone());
    let primes_within_coef = _get_primes_within_poly(highest_coef);
    //The counter makes sure that in the case that the last j does satisfy the if statement, must also check to see if the current i divides every other j prior.
    let mut counter: i32 = 0;
    for i in primes_within_coef {
        for j in middle.clone() {
            if j % i == 0 {
                counter += 1;
            }
        }
        if counter as usize == middle.iter().len() {
            prime_divisors.push(i);
        }
        // Resetting counter for next i
        counter = 0;
    }
    println!(
        "Prime divisors that divide every
coefficient (excluding a_n): {:?}",
        prime_divisors
    );
    prime_divisors
}

// Algorithm to check if any element of the vector ouput from
//_first_condition divides a_n. If yes, Eisenstien's criterion
//doesn't satisfy the second condition
fn _second_condition(full_vec: Vec<i32>, prime_divisors: Vec<i32>) -> bool {
    let mut divisors_of_a_n: Vec<i32> = Vec::new();
    for i in prime_divisors {
        if full_vec.last().unwrap() % i == 0 {
            divisors_of_a_n.push(i);
        }
    }
    if divisors_of_a_n.is_empty() {
        return true;
    } else {
        return false;
    }
}

//Algorithm to check if the square of any element from the vector
//output of _first_condition divides a_0. If yes, Eisenstien's
//crtierion doesn't satisfy the last condition.
fn _third_condition(full_vec: Vec<i32>, prime_divisors: Vec<i32>) -> bool {
    let mut squared_divisors: Vec<i32> = Vec::new();
    for i in prime_divisors {
        if full_vec.first().unwrap() % i ^ 2 == 0 {
            squared_divisors.push(i);
        }
    }
    if squared_divisors.is_empty() {
        return true;
    } else {
        return false;
    }
}

//Algorithm to check whether a randomly generated polynomial satisfies Eisenstien's criterion.
pub fn check_conditions(full_vec: Vec<i32>, smaller_vec: Vec<i32>) {
    let divisors = _first_condition(smaller_vec.clone());
    if divisors.is_empty() {
        println!("Did not pass the first condition; fails E.C.");
        return;
    } else {
        println!("Passed the first condition.");
        if _second_condition(full_vec.clone(), divisors.clone()) == true {
            println!("Passed the second condition.");
            if _third_condition(full_vec, divisors) == true {
                println!("Passed all three conditions; satisfies E.C.")
            } else {
                println!("Did not pass the third condition; fails E.C.");
                return;
            }
        } else {
            println!("Did not pass the second condition; fails E.C.");
            return;
        }
    }
}
