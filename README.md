# Eisenstein's criterion
This is an implementation of applying Eisenstein's criterion to randomly generated polynomials in $\mathbb{Z}[x]$.

Irreducibile polynomials are important for many reasons (for example, constructing finite fields). There are tests to see if a given polynomial is irreducible over some field $\mathbb{F}$. Eisenstein's criterion determines if a polynomial with integer coefficients is irreducible strictly over the field of rationals, $\mathbb{Q}$

## Theorem
Consider the polynomial $p(x) = a_0 + a_1(x) + a_2(x^2) + ... + a_n(x^n)$ where $a_i \in \mathbb{Z}$.

Eisenstein's criterion states that $p(x)$ is irreducible over $\mathbb{Q}$ if there exists a prime number $p\in \mathbb{Z}$ such that:
1. $p$ divides each $a_i$ for $0 \leq i < n$,
2. $p$ does not divide $a_n$
3. $p^2$ does not divide $a_0$

* Note that some polynomials do not directly satisfy Eisenstein's criterion, but can be transformed into a new polynomail that "indirectly" satisfies Eisenstein's criterion, which is enough to claim the original polynomial irreducible over $\mathbb{Q}$. This program only focuses on the direct case.

Check out https://en.wikipedia.org/wiki/Eisenstein's_criterion for more details.

## Running
~~~
git clone https://github.com/a-rust/eisensteins-criterion.git
cd eisensteins-criterion
cargo run
~~~

## Setup
* The program takes a randomly generated polynomial $p(x)$, and determines if it directly satisfies Eisenstein's criterion.
  * As stated earlier, $p(x)$ may fail the direct test, but upon a transformation, may satisfy the "indirest test".
    * Thus, if $p(x)$ fails the direct test in this program, it is not enough to conclude that $p(x)$ is reducible over $\mathbb{Q}$. 
* The user is given a choice to set the degree $n$ of $p(x)$, or default to a random $2\leq n\leq 10$. 
* The user is also given a choice to set the upper bound on the random number generator for the coefficients of $p(x)$, or default to $100$. 
* As the user, try to decide whether setting a higher or lower degree affects the chances that $p(x)$ satisfies all three conditions. 
  * Likewise, think about whether the range of possible coefficients of $p(x)$ (i.e., the choice of the upper bound) affects the chances that $p(x)$ satisfies all three conditions.
