mod conditions;
mod gen_polyn;
use conditions::*;
use gen_polyn::*;


fn main() {
    let user_degree = choose_degree();
    let user_upper_bound = choose_upper_bound();
    let gen_polyn = gen_polynomial(user_degree, user_upper_bound);
    let highest_coef = get_highest_coef(&gen_polyn);
    let prime_candidates = get_primes_within_poly(highest_coef);
    let first_condition_candidates = first_condition(&prime_candidates, &gen_polyn);
    let second_condition_candidates = second_condition(&gen_polyn, &first_condition_candidates);
    let third_condition_candidates = third_condition(&gen_polyn, &second_condition_candidates);
    check_conditions(&third_condition_candidates);
}
