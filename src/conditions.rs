//Algorithm to check which prime numbers, if any, divide the first //n-1 elements of p(x). This tells us if p(x) satisfies the first condition of Eisenstien's criterion.

//Finding prime number within given range of possible coefficients
fn _finding_primes() -> Vec<i32> {
    let possible: Vec<i32> = (1..=20).collect();
    let mut known: Vec<i32> = Vec::new();
    for i in possible {
        for j in 2..i + 1 {
            if j == i {
                known.push(i);
            }
            if i % j == 0 {
                break;
            }
        }
    }
    known
}
fn _first_condition(middle: Vec<i32>) -> Vec<i32> {
    let mut prime_divisors = Vec::<i32>::new();
    let known_primes = _finding_primes();
    //The counter makes sure that in the case that the last j does satisfy the if statement, must also check to see if the current i divides every other j prior.
    let mut counter: i32 = 0;
    for i in known_primes {
        for j in middle.clone() {
            if j % i == 0 {
                counter += 1;
            }
        }
        if counter as usize == middle.iter().len() {
            prime_divisors.push(i);
        }
        //Resetting counter for next i
        counter = 0;
    }
    println!(
        "Prime divisors that divide every
coefficient (excluding a_n): {:?}",
        prime_divisors
    );
    prime_divisors
}

//Algorithm to check if any element of the vector ouput from
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
