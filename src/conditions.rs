// Helper function to find all primes within a given range of possible coefficients
// This acts as our set of possible candidates to pass all 3 conditions of E.C.
pub fn get_primes_within_poly(highest_coef: i32) -> Vec<i32> {
    let poly_coef_range: Vec<i32> = (1..=highest_coef).collect();
    let mut prime_candidates: Vec<i32> = Vec::new();
    // Logic for finding all primes within the range (1, highest_coef]
    for i in poly_coef_range {
        // Checking to see if there are any factors of i within [2, i]
        for j in 2..i + 1 {
            // If no value of j divides i, then we can conclude i is prime
            if j == i {
                prime_candidates.push(i);
            }
            // If j is a factor of i, then we know i is not prime
            if i % j == 0 {
                break;
            }
        }
    }
    println!("Prime candidates: {:?}", &prime_candidates);
    prime_candidates
} 

// Function that determines if the randomly generated polynomial satisfies the first element of E.C (i.e., the first n-1 coefficients are all divisible by some common prime number)
pub fn first_condition(prime_candidates: &Vec<i32>, full_vec: &Vec<i32>) -> Vec<i32> {
    let mut first_condition_candidates = Vec::<i32>::new();
    // The counter is used to verify that all elements in middle are divisible by some prime i in primes_within_coef. If so, push i to prime_divisors
    let mut counter: i32 = 0;
    for i in prime_candidates {
        for j in full_vec {
            if j == full_vec.last().unwrap(){
                continue;
            }
            if j % i == 0 {
                counter += 1;
            }
        }
        if counter as usize == full_vec.iter().len() - 1 {
            first_condition_candidates.push(*i);
        }
        // Resetting counter for next prime i
        counter = 0;
    }
    println!("Prime candidates that pass the first condition: {:?}", first_condition_candidates);
    return first_condition_candidates;
}

// Function that returns a sub-vector of elements from prime_divisors such that every i in the sub-vector does not divide the highest degree coefficient of full_vec (i.e., a_n)
// This sub-vector will be used to check the second condition of E.C.
pub fn second_condition(first_condition_candidates: &Vec<i32>, full_vec: &Vec<i32>) -> Vec<i32> {
    let mut second_condition_candidates: Vec<i32> = Vec::new();
    for i in first_condition_candidates {
        if full_vec.last().unwrap() % i != 0 {
            second_condition_candidates.push(*i);
        }
    }
    println!("Remaining prime candidates that pass the second condition: {:?}", second_condition_candidates);
    return second_condition_candidates;
}

// Function that returns a sub-vector of elements from prime_divisors such that the square of every i in the sub_vector does not divide the lowest degree coefficient of full_vec (i.e., a_0)
// This sub-vector will be used to check the third condition of E.C.
pub fn third_condition(second_condition_candidates: &Vec<i32>, full_vec: &Vec<i32>) -> Vec<i32> {
    let a_0 = full_vec[0];
    let mut final_candidates: Vec<i32> = Vec::new();
    for i in second_condition_candidates {
        let prime_squared: i32 = i.pow(2);
        if a_0 % prime_squared == 0 {
            final_candidates.push(*i);
        }
    }
    println!("Remaining prime candidates that pass the third and final condition: {:?}", final_candidates);
    return final_candidates;
}

// Function to check whether full_vec satisfies all 3 conditions of E.C.
pub fn check_conditions(final_candidates: &Vec<i32>) -> bool {
    if !final_candidates.is_empty() {
        println!("Passed all three conditions; satisfies E.C.");
        return true;
    } else {
        println!("Try another polynomial!");
        return false;
    }
}


#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn test_get_primes_within_poly() {
        assert_eq!(get_primes_within_poly(20), vec![2, 3, 5, 7, 11, 13, 17, 19])
    }

    #[test]
    // Case 1: at least one candidate passes the first condition
    fn test_pass_first_condition() {
        // First parameter is set of candidate primes, second parameter is coefficients of polynomial
        // First condition returns a set of candidate primes that satisfy the first condition
        assert_eq!(first_condition(&vec![2, 3, 8], &vec![6, 18, 100]), vec![2, 3]);
    }

    #[test]
    // Case 2: no candidate passes the first condition
    fn test_fail_first_condition() {
        // First parameter is set of candidate primes, second parameter is coefficients of polynomial
        // First condition returns a set of candidate primes that satisfy the first condition
        assert_eq!(first_condition(&vec![2, 3, 8], &vec![11, 12, 13]), vec![]);
    }
}
