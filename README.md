# Eisenstein's criterion in Rust
This is an implementation of Eisenstein's criterion from scratch in Rust. 

# Irreducibility tests
Irreducibile polynomials are important for constructing finite fields. (Read [these notes](https://github.com/a-rust/constructing-finite-fields) I wrote about constructing finite fields from qoutient rings). There are many tests to determine whether a given polynomial is irreducible or not. Some of the tests depend on the field of coefficients; for example, Eisenstein's criterion (E.C).

## Theorem:
E.C. states the following:

Consider the polynomial $p(x) = a_0 + a_1(x) + a_2(x^2) + ... + a_n(x^n)$ where $a_i \in Q$.

If there exists a prime number $p$ such that:
1. for $0 \leq i < n$, $p$ divides $a_i$
2. $p$ does not divide $a_n$
3. $p^2$ does not divide $a_0$,

then $p(x)$ is irreducible over $Q$.

Note that this does not state that if a polynomial $p(x)$ over $Q$ does not satisfy E.C., then it is not irreducible over $Q$; consider $x^2 - 1 = (x + 1)(x - 1)$.

For more information, check out the [wikipedia page](https://en.wikipedia.org/wiki/Eisenstein's_criterion)

# Running the code:
The program takes a randomly generated polynomial, and determines if it satisfies E.C. 

The user is given a choice to set the degree of the polynomial, or let it be chosen randomly from 2-10. The lower bound is 2 because irreducible polynomials, by definition, must be of at least degree 2.

Generally speaking, higher degree randomly generated polynomials will be less likely to satisfy E.C., due to the first condition of E.C. being more difficult to satisfy as the degree (randomly) increases.
So setting the degree anywhere from 2-10 is best to get a feel for how E.C works.

The user is also given a choice to set the upper bound on the rng of all coefficients, from 1 - $n$, or let it default to $100$. Unlike the degree, setting this upper bound for all coefficients to be higher grants more opportunity for that a potential $p$ satsfying E.C. does in fact exist, generally speaking.