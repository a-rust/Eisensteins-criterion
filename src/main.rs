use crate::conditions::check_conditions;
use crate::gen_polyn::*;
mod conditions;
mod gen_polyn;

fn main() {
    let user_degree = choose_degree();
    let user_upper_bound = choose_upper_bound();
    let gen_polyn = gen_polynomial(user_degree, user_upper_bound);
    let first_n_minus_one = first_n_minus_one_coef(gen_polyn.clone(), user_degree);
    check_conditions(gen_polyn, first_n_minus_one);
}
