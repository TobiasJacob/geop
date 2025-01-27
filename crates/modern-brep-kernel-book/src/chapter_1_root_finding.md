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

This is a system of equations which is polynomial in nature. We can solve multiple systems of polynomial equations with IPP.

#### Curve curve intersection

This is a table to show which roots to find depending on if the curve is explicit or implicit:

| Curve 1           | Curve 2     | Roots to find |
|-------------------|-------------|---------------|
| Implicit \\(u_1, u_2\\) | Implicit \\(v_1, v_2\\) | \\(u_1(x, y, z) = 0, u_2(x, y, z) = 0, v_1(x, y, z) = 0, v_2(x, y, z) = 0\\) |
| Implicit \\(u_1, u_2\\) | Explicit \\(p(t_2)\\) | \\(u_1(p(t_2)) = 0, u_2(p(t_2)) = 0\\) |
| Explicit \\(p(t_1)\\) | Explicit \\(p(t_2)\\) | \\(p(t_1) - p(t_2) = 0\\) |

Since we can add, multiply and compose algebraic expressions, we can easily find the intersection of two curves. We just setup the "Roots to find" equation and let the IPP algorithm do the rest.

We expect the solution to be either a curve (if the curves are equal), or a set of points.

#### Curve surface intersection

| Curve 1           | Surface 2     | Roots to find |
|-------------------|-------------|---------------|
| Implicit \\(u_1, u_2\\) | Implicit \\(v_1\\) | \\(u_1(x, y, z) = 0, u_2(x, y, z) = 0, v_1(x, y, z) = 0\\) |
| Implicit \\(u_1, u_2\\) | Explicit \\(p(t_2, t_3)\\) | \\(u_1(p(t_3, t_4)) = 0, u_2(p(t_3, t_4)) = 0\\) |
| Explicit \\(p(t_1)\\) | Implicit \\(v_1\\) | \\(v_1(p(t_1)) = 0\\) |
| Explicit \\(p(t_1)\\) | Explicit \\(p(t_2, t_3)\\) | \\(p(t_1) - p(t_2, t_3)  = 0\\) |

We expect the solution to be either a curve (if the curve is embedded in the surface), or a set of points.

#### Surface surface intersection

| Surface 1           | Surface 2     | Roots to find |
|-------------------|-------------|---------------|
| Implicit \\(u_1\\) | Implicit \\(v_1\\) | \\(u_1(x, y, z) = 0, v_1(x, y, z) = 0\\) |
| Implicit \\(u_1\\) | Explicit \\(p(t_3, t_4)\\) | \\(u_1(p(t_3, t_4)) = 0\\) |
| Explicit \\(p(t_1, t_2)\\) | Explicit \\(p(t_3, t_4)\\) | \\(p(t_1, t_2) - p(t_3, t_4) = 0\\) |

We expect the solution to be either a surface (if the two surfaces are equal), or a set of points and curves.

## Root finding

Now that we have a "roots to find" equation for each intersection type, we can use our favorite root finding algorithm.

- For the curve-curve intersection, we can first check for equality, and if we ruled out equality, we can use IPP to find all intersection points.
- For the curve-surface intersection, we can first check for containment, and if we ruled out containment, IPP will tell us the points.
- For the surface-surface intersection, things get more complicated. If the surfaces are equal, we can simply return the surface. But if they differ, we have to find all numerical approximations of the intersection curves, or tell the solver, which curve should be fittet to the solution (e.g. with two spheres, we know that we have the solution should look like a circle). 

## Root finding algorithms for points

### Newton-Raphson

For benchmarking, we implement the Newton-Raphson method. It is very easy to implement. However, it is not guaranteed to find all roots, since we have to use an initial guess.

### IPP Algorithm

The standard algorithm used for root finding is the IPP algorithm. How the IPP algorithm works is explained in the [Interval Projected Polyhedron Algorithm section](https://web.mit.edu/hyperbook/Patrikalakis-Maekawa-Cho/node52.html) of the MIT course. It will always find all roots.

## Root finding algorithms for curves

There is an idea on how to modify IPP to to deal with curves, and return Bernstein Polynomials / Nurbs with bounded errors to the true solution, but it will be tough to implement. 

The polygon used to represent the boundary of the root will start to shrink towards a line, if the solution is a curve. We have to recognize this, and limit the shrinking along the main axis, but shrink along the perpendicular intersection axis as far as possible. 

A second step could be the fitting of a closed form solution (e.g. a circle) to the approximation.
