# FAQ

## Why not expose the parameter space of curves and surfaces?

Usually, the nasty details of the exact representation of a curve or surface should remain in geop-geometry. The other crates only interface to it via methods, where instead of a parameter, a 3d point is used.

E.g. for a curve, the function signature is not `tangent(f64) -> Vec3`, but `tangent(Vec3) -> Vec3`.

This may have performance implications for functions, where projection is expensive. In the future, we might use the plane, cylinder and sphere as prototypical parameter spaces. So far, however, those functions are only called in places where a projection would be necessary anyway, so there is no performance loss yet. And for simple cases, like a sphere, the `normal(Vec3) -> Vec3` function is even faster than `normal(f64, f64) -> Vec3`.

## Is the border of a face still part of the face?

Good question. In general, no. Even though this might be subject to change.

So for example, the intersection of two touching faces is empty.

The intersection of a line touching a face is 2 lines, which end and start at the touching point. This is useful for the boolean operations, as it creates a split point.

However, when checking if a point is inside a face or an edge, the function will tell you if it is actually inside, on the boundary, or outside.

