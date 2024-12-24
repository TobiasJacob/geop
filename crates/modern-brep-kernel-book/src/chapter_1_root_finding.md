# Root finding

## Root finding with algebraic expressions

Why do we care so much about algebraic expressions like Bernstein Polynomials or Nurbs? Because using the IPP (Interval Projected Polyhedro Algorithm) allows us to find arbitrary roots of them. Let's consider the following example:

Imagine there is a line

$$p(t) = \vec{a} + t \vec{b}$$

And a circle in its implicit form (intersection of a sphere and a plane):

$$u_1(x, y, z) = x ^ 2 + y ^ 2 + z ^ 2 - r = 0$$
$$u_2(x, y, z) = n_1 x + n_2 y + n_3 z = 0$$

Now to find the intersection, we have to solve

$$u_1(p(t)) = 0$$
$$u_2(p(t)) = 0$$

This is a system of equations which is polynomial in nature. We can convert this system of equations into a single polynomial equation using Bernstein Polynomials. The IPP algorithm can then find the roots of this polynomial equation. This is how we can find the intersection of a line and a circle.

#### Curve curve intersection

This is a table to show which roots to find depending on if the curve is explicit or implicit:

| Curve 1           | Curve 2     | Roots to find |
|-------------------|-------------|---------------|
| Implicit \\(u_1, u_2\\) | Implicit \\(v_1, v_2\\) | \\(u_1(x, y, z) \cdot u_2(x, y, z) \cdot v_1(x, y, z) \cdot v_2(x, y, z)\\) |
| Implicit \\(u_1, u_2\\) | Explicit \\(p(t_2)\\) | \\(u_1(p(t_2)) \cdot u_2(p(t_2))\\) |
| Explicit \\(p(t_1)\\) | Explicit \\(p(t_2)\\) | \\((p(t_1) - p(t_2))^2\\) |

Since we can add, multiply and compose algebraic expressions, we can easily find the intersection of two curves. We just setup the "Roots to find" equation and let the IPP algorithm do the rest.

#### Curve surface intersection

| Curve 1           | Surface 2     | Roots to find |
|-------------------|-------------|---------------|
| Implicit \\(u_1, u_2\\) | Implicit \\(v_1\\) | \\(u_1(x, y, z) \cdot u_2(x, y, z) \cdot v_1(x, y, z)\\) |
| Implicit \\(u_1, u_2\\) | Explicit \\(p(t_3, t_4)\\) | \\(u_1(p(t_3, t_4)) \cdot u_2(p(t_3, t_4))\\) |
