# Kernel Architecture

The overall kernel architecture is shown in the following figure:

- `geop-algebra` Implements the basic algebraic operations in interval arithmetic, like EFloat64 (extended Float, with upper and lower boundary), Polynomials, BSplines, Nurbs, or other algebraic expressions.
- `geop-geometry`: Implements "unbounded" sets, curves, surfaces, points, etc. This can be a plane, a sphere, a circle, or a line.
- `geop-topology`: Implements "bounded" sets, like edges, faces, volumes.
- `geop-rasterize`: Implements rasterization algorithms that convert topological objects into triangle list, that can be rendered by a GPU.
- `geop-wgpu`: Uses the rasterizaed data and renders it using the `wgpu` crate.
- `modern-brep-kernel-book`: This book.

## Things that I tried that did not work

Well, there are some ideas that sounded good but turned out badly. Use this as advice for developing your own geometric kernel:

- **Comparing floats to 1e-8 to check for equality**: Bad idea. Use the EFloat class instead, that uses interval arithmetic and tracks the upper and lower boundary.
- **Doing Algebra instead of numerical approxmations with subdivion**: In the first versions, a line-line intersection would return a line, or a point, or nothing. A circle-line intersectino would return two points, one point, or nothing. But after adding more and more geometric options, the N^2 complexity was too much implementation effort. The second idea was to use Algebra solvers, to solve each case algebraicly. But e.g. for nurbs-nurbs intersection, there is not even a closed formula, and if there were, it would have a degree of >300. So we discarded everything for a numerical approach with subdivision, which is very easy to understand and much more robust.
- **Doing algebra in rusts type system**: It is better that EFloat returns a result if you take a square root of a negative number, instead of panicking, or defining a type that is PositiveEfloat. The types tend to grow exponentially.
- **Thinking coincident surfaces or curves are an edge case**: Surface surface intersection is not always a curve. It can be a subpart of the surface as well. While in most 3D applications, coincident surfaces are an edge-case, they are actually the default case in CAD applications. Same applies for curve-curve intersection.
- **Inverting a matrix or calculating a derivative**: Geop does not use any algebra library in its core, since it is not needed. Inverting a matrix is to brittle. Derivatives tend to be zero in singular points, which is not helpful. Instead, geop uses a numerical approach with subdivision, which is works always, no matter how complex the geometry is.
- **Doing booleans with only checking local conditions**: I spend a lot of time coming up with schema a-la "If an incoming curve from A intersects B, then for the union it has to continue with the outgoing part of B". But it does not work for all cases. The simplest way to do booleans is to classify each curve segment into AoutB, AinB, AonBParallel, AOnBOpposite.
- **Having a single type for Edge, with an enum for the underlying curve**: It is easier to have the Edge as an interface, and implement each Edge type (e.g. Line with start and end point, Circle with center and radius, and none, 1, or 2 disjoint edge points) separately. This way, the Edge can be a trait object, and the underlying curve can be a concrete type.
- **Forcing disjoint start and end points for circular curves**: For booleans, we do subdivion by splitting each curve at the intersection points. So typically for a circle, it is split into one curve with anchor point, and as a second step into two disjoint curves. This way, we can handle all cases, no matter how complex the geometry is.
- **Having an outside and multiple inside boundaries for faces**: You cannot clearly define which one the outer boundary is, and it is not even useful to know this. For a face, all boundaries are a set of curves, with no one being special. 

### Ideas to be discussed
- **Using homogenous coordinates**: At first, we used EFloats and 3D points a lot. But then we realized that we can work much better with circles if switch to homogenous coordinates instead. This way, dividing by zero, etc. results in a homogenous point at infinity, e.g. 3/0 is fine. Equality checks can be division free as well. However, it is still not possible to normalize zero, as this would result in 0/0, which is every possible real number, and invalid. Thus, we switched to homogenous coordinates for all calculations.
A big problem is however, that now multiplication can fail. E.g. (3/0) * (0/3) is (0/0) which is invalid, so it does not help, it just shifts the problem.

## Dark vs light mode

The kernel can be used in both dark and light mode. The following figure shows the difference between the two modes:

![Dark mode](./generated_images/test_dark.png)
![Light mode](./generated_images/test_light.png)
