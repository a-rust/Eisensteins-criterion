# Eisenstein's criterion
This is an implementation of applying Eisenstein's criterion to randomly generated polynomials over $Q$

Irreducibile polynomials are important for constructing finite fields (here are [some notes](https://github.com/a-rust/constructing-finite-fields) I wrote about constructing finite fields from irreducible polynomials). There are many tests to see if a given polynomial is irreducible over some field $F$. Eisenstein's criterion determines if a polynomial with integer coefficients is irreducible over $Q$.

## Theorem
Consider the polynomial $p(x) = a_0 + a_1(x) + a_2(x^2) + ... + a_n(x^n)$ where $a_i \in Q$.

Eisenstein's criterion states that $p(x)$ is irreducible over $Q$ if there exists a prime number $p\in Z$ such that:
1. $p$ divides each $a_i$ for $0 \leq i < n$,
2. $p$ does not divide $a_n$
3. $p^2$ does not divide $a_0$

* Note that some polynomials do not directly satisfy Eisenstein's criterion, but can be transformed into a new polynomail that does satisfy Eisenstein's criterion; the original polynomial "indirectly" satisfies Eisenstein's criterion, and is thus irreducible.

Check out https://en.wikipedia.org/wiki/Eisenstein's_criterion for more details.

## Running
~~~
git clone https://github.com/a-rust/eisensteins-criterion.git
cd eisensteins-criterion
cargo run
~~~

## Setup
* The program takes a randomly generated polynomial, and determines if it directly satisfies Eisenstein's criterion.
* The user is given a choice to set the degree $n$ of $p(x)$, or default to a random $2\leq n\leq 10$. 
  * $2\leq n$ because irreducible polynomials, by definition, must be of at least degree 2.
* Generally speaking, randomly generated polynomials of higher degree will be less likely to satisfy Eisenstein's criterion.
  * The first condition requires a lot.
  * Observation: setting $n\leq 4$ is best to get a feel for how Eisenstein's criterion works.
* The user is also given a choice to set the upper bound on the random number generator for the coefficients of $p(x)$, or default to $100$. 
  * Intuitively, a higher upper bound should grant more opportunity for potential $p$'s to satisfy all three conditions.
