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

// Function that determines if the randomly generated polynomial satisfies the first element of E.C (i.e., the first n-1 coefficients are all divisible by some common prime number)
fn _first_condition(middle: Vec<i32>) -> Vec<i32> {
    let mut prime_divisors = Vec::<i32>::new();
    let highest_coef = get_highest_coef(middle.clone());
    let primes_within_coef = _get_primes_within_poly(highest_coef);
    // The counter is used to verify that all elements in middle are divisible by some prime i in primes_within_coef. If so, push i to prime_divisors
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
        // Resetting counter for next prime i
        counter = 0;
    }
    println!(
        "Prime divisors that divide every
coefficient (excluding a_n): {:?}",
        prime_divisors
    );
    prime_divisors
}

// Function that returns a sub-vector of elements from prime_divisors such that every i in the sub-vector does not divide the highest degree coefficient of full_vec (i.e., a_n)
// This sub-vector will be used to check the second condition of E.C.
fn _second_condition(full_vec: Vec<i32>, prime_divisors: Vec<i32>) -> Vec<i32> {
    let mut non_divisors_of_a_n: Vec<i32> = Vec::new();
    for i in prime_divisors {
        if full_vec.last().unwrap() % i != 0 {
            non_divisors_of_a_n.push(i);
        }
    }
    return non_divisors_of_a_n
}

// Function that returns a sub-vector of elements from prime_divisors such that the square of every i in the sub_vector does not divide the lowest degree coefficient of full_vec (i.e., a_0)
// This sub-vector will be used to check the third condition of E.C.
fn _third_condition(full_vec: Vec<i32>, prime_divisors: Vec<i32>) -> Vec<i32> {
    let mut squared_divisors: Vec<i32> = Vec::new();
    for i in prime_divisors {
        if full_vec.first().unwrap() % (i ^ 2) == 0 {
            squared_divisors.push(i);
        }
    }
    return squared_divisors;
}

// Function to check whether full_vec satisfies all 3 conditions of E.C.
pub fn check_conditions(full_vec: Vec<i32>, middle_vec: Vec<i32>) {
    let prime_divisors: Vec<i32> = _first_condition(middle_vec.clone());
    if prime_divisors.is_empty() {
        println!("Did not pass the first condition; fails E.C.");
        return;
    } else {
        println!("Passed the first condition.");
        let second_cond_vec: Vec<i32> = _second_condition(full_vec.clone(), prime_divisors.clone());
        if second_cond_vec.is_empty() {
            println!("Did not pass the second condition; fails E.C.");
        } else {
            println!("Passed the second condition.");
            let third_cond_vec: Vec<i32> = _third_condition(full_vec, prime_divisors.clone());
            if third_cond_vec.is_empty() {
                println!("Did not pass the third condition; fails E.C.");
            }
            else {
                println!("Passed all three conditions; satisfies E.C.")
            }
        }
    }
}
