# Table of Contents
- [Table of Contents](#table-of-contents)
- [Project Description](#project-description)
  - [Overview of Theorem](#overview-of-theorem)
  - [How to run the program yourself](#how-to-run-the-program-yourself)
  - [Comments about the program](#comments-about-the-program)

# Project Description
This is a command line tool that generates random polynomials in $\mathbb{Z}[x]$, and determines if they are [irreducible over $\mathbb{Q}$](https://en.wikipedia.org/wiki/Irreducible_polynomial) via [Eisenstein's criterion](https://en.wikipedia.org/wiki/Eisenstein's_criterion)

## Overview of Theorem
Consider the polynomial $p(x) = a_0 + a_1(x) + a_2(x^2) + ... + a_n(x^n)$, where $a_i \in \mathbb{Z}$.

Eisenstein's criterion states that $p(x)$ is irreducible over $\mathbb{Q}$ if there exists a prime number $p\in \mathbb{Z}$ such that:
1. $p$ divides each $a_i$ for $0 \leq i < n$,
2. $p$ does not divide $a_n$
3. $p^2$ does not divide $a_0$

* Note that some polynomials do not directly satisfy Eisenstein's criterion, but can be transformed into a new polynomail that "indirectly" satisfies Eisenstein's criterion, which is enough to claim the original polynomial irreducible over $\mathbb{Q}$. This tool only focuses on the direct case.

## How to run the program yourself
~~~
git clone https://github.com/a-rust/eisensteins-criterion.git
cd eisensteins-criterion
cargo run
~~~

## Comments about the program
* The user is given a choice to set the degree $n$ of $p(x)$, or default to a random $2\leq n\leq 10$. 
* The user is also given a choice to set the upper bound on the randomly chosen coefficients of $p(x)$, or default to $100$. 
* As the user, try to decide whether setting a higher or lower degree affects the chances that $p(x)$ satisfies all three conditions. 
  * Likewise, think about whether the range of possible coefficients affects the chances that $p(x)$ satisfies all three conditions.
