# Bernstein Polynomials

A polynom in Bernstein representation is of the form $$p(x) = \sum_{i=0}^n a_i B_{i,n}(x)$$, with the basis functions being defined as $$B_{i,n}(x) = \binom{n}{i} x^i (1-x)^{n-i}$$. The Bernstein representation is a list of coefficients, where the coefficients are multiplied with the Bernstein basis functions. The following code snippet shows the `BernsteinPolynom` struct:

```rust
pub struct BernsteinPolynomial<T> {
    coefficients: Vec<T>,
}
```

Every polynomial can be converted from monomial to Bernstein representation and back. Bernstein Polynomials have better numerical properties than monomial polynomials, and are therefore used in many geometric algorithms.

Here is a picture of the Basis functions from \\(B_{0, 0}\\) (top) to \\(B_{0, 5}\\)...\\(B_{5, 5}\\) (bottom):

![Bernstein Basis Functions](./generated_images/algebra/bernstein_basis.png)

This table lists the polynomial functions \\( B_{i,j}(x) \\):

| \\( B_{i,j}(x) \\)  | Expression |
|-------------------|------------|
| \\( B_{0,0}(x) \\)  | \\( 1.00 \cdot x^0 \\) |
| \\( B_{0,1}(x) \\)  | \\( -1.00 \cdot x^1 + 1.00 \cdot x^0 \\) |
| \\( B_{1,1}(x) \\)  | \\( 1.00 \cdot x^1 \\) |
| \\( B_{0,2}(x) \\)  | \\( 1.00 \cdot x^2 - 2.00 \cdot x^1 + 1.00 \cdot x^0 \\) |
| \\( B_{1,2}(x) \\)  | \\( -2.00 \cdot x^2 + 2.00 \cdot x^1 \\) |
| \\( B_{2,2}(x) \\)  | \\( 1.00 \cdot x^2 \\) |
| \\( B_{0,3}(x) \\)  | \\( -1.00 \cdot x^3 + 3.00 \cdot x^2 - 3.00 \cdot x^1 + 1.00 \cdot x^0 \\) |
| \\( B_{1,3}(x) \\)  | \\( 3.00 \cdot x^3 - 6.00 \cdot x^2 + 3.00 \cdot x^1 \\) |
| \\( B_{2,3}(x) \\)  | \\( -3.00 \cdot x^3 + 3.00 \cdot x^2 \\) |
| \\( B_{3,3}(x) \\)  | \\( 1.00 \cdot x^3 \\) |
| \\( B_{0,4}(x) \\)  | \\( 1.00 \cdot x^4 - 4.00 \cdot x^3 + 6.00 \cdot x^2 - 4.00 \cdot x^1 + 1.00 \cdot x^0 \\) |
| \\( B_{1,4}(x) \\)  | \\( -4.00 \cdot x^4 + 12.00 \cdot x^3 - 12.00 \cdot x^2 + 4.00 \cdot x^1 \\) |
| \\( B_{2,4}(x) \\)  | \\( 6.00 \cdot x^4 - 12.00 \cdot x^3 + 6.00 \cdot x^2 \\) |
| \\( B_{3,4}(x) \\)  | \\( -4.00 \cdot x^4 + 4.00 \cdot x^3 \\) |
| \\( B_{4,4}(x) \\)  | \\( 1.00 \cdot x^4 \\) |
| ...               | ...        |

Many people forget that Bernstein Polynomials are defined on the entire real number range, not just on the interval \\([0, 1]\\). This means we could extrapolate far beyond the control points, but they spiral out of control quite quickly. However, monomial polynomials are also defined on the entire real number range, so it is nice that we can approximate them entirely with a couple control points.

And this is the Bernstein polynomial with coefficients `0.0, 0.6, 0.1, 0.8, 0.3`:

![Bernstein Function](./generated_images/algebra/bernstein.png)

## Algorithms for polynomials in Bernstein form

There is a great research paper called [Algorithms for polynomials in Bernstein form by R.T. Farouki and V.T. Rajan](https://www.sciencedirect.com/science/article/pii/0167839688900167). Most of the following algorithms are based on this paper, which we will quote as `Farouki 1988`.

## Conversion between Monomial and Bernstein

If a polynomial of degree \\(n\\) is given in monomial form and Bernstein form as

$$ P(x) = \sum_{i=0}^n c_i x^i = \sum_{i=0}^n a_{i} B_{i,n}(x) $$

then the coefficients are related by

$$ c_i = \sum_{k=0}^i (-1)^{i - k} \binom{n}{i} \binom{i}{k} a_{k} $$

and

$$ a_i = \sum_{k=0}^i \frac{\binom{i}{k}}{\binom{n}{k}} c_k $$

For example, for \\(n=4\\), the factors for converting to monomial are

```
1.00e0
-4.00e0 4.00e0
6.00e0  -1.20e1 6.00e0
-4.00e0 1.20e1  -1.20e1 4.00e0
1.00e0  -4.00e0 6.00e0  -4.00e0 1.00e0
```

and for converting to Bernstein are

```
1.00e0
1.00e0  2.50e-1
1.00e0  5.00e-1 1.67e-1
1.00e0  7.50e-1 5.00e-1 2.50e-1
1.00e0  1.00e0  1.00e0  1.00e0  1.00e0
```

## Adding Bernstein Polynomials




