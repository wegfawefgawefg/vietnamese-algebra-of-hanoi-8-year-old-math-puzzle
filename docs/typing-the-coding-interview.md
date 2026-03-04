# Typing the Coding Interview (Funny Experimental Architecture)

This document describes the intentionally overengineered version of this puzzle
solver, inspired by the "typing the coding interview" meme.

The idea is to solve a simple digit puzzle through layers of foundational
abstractions:

1. Set-theoretic naturals (von Neumann/Peano style)
2. A number tower (`Nat -> Int -> Rational`)
3. Cons cells and list-based expression trees
4. A puzzle solver that runs on top

## Why Do This?

Because it is funny, educational, and kind of beautiful.

It is not the fastest route to a solution. The direct integer brute-force in
`src/main.rs` is much more practical.

## Layer 1: Sets of Sets as Naturals

Use the classic encoding:

- `0 = {}`
- `1 = {0}`
- `2 = {0,1}`
- `3 = {0,1,2}`

`succ(n) = n U {n}`

At this layer, addition and multiplication are defined recursively in Peano
style. In Rust, this usually means a symbolic structure plus helper operations.

## Layer 2: Number Tower

Build increasingly expressive numeric types:

- `Nat` for counting
- `Int` for signed arithmetic
- `Rational` for exact division terms like `(13*B)/C`

This makes equation evaluation exact and avoids floating-point edge cases.

## Layer 3: Cons Cells and AST

Represent expressions as Lisp-like trees with cons cells, for example:

`(+ A (/ (* 13 B) C) D (* 12 E) (- F) (- 11) (/ (* G H) I) (- 10))`

This lets the same evaluator run different puzzle variants by changing only the
expression data.

## Layer 4: Constraint Solver

On top of the expression model:

- Variables: `A..I`
- Domain: digits `1..9`
- All-different constraint
- Evaluate expression under each assignment
- Keep assignments equal to target (`66`)

Possible pruning:

- Early partial evaluation
- Divisibility checks before full evaluation
- Symmetry breaking on variable ordering

## Practical Hybrid

A good compromise is:

- Keep this towered/symbolic version for clarity and experimentation
- Keep the current integer brute-force backend for speed

Same puzzle interface, two execution engines.

## Meme Status

Yes, this is absolutely "typing the coding interview."

That is the point.
