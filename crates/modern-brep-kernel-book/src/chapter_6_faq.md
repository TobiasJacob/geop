# FAQ

## Why not expose the parameter space of curves and surfaces?

Usually, the nasty details of the exact representation of a curve or surface should remain in geop-geometry. The other crates only interface to it via methods, where instead of a parameter, a 3d point is used.

E.g. for a curve, the function signature is not `tangent(f64) -> Vec3`, but `tangent(Vec3) -> Vec3`.

This may have performance implications for functions, where projection is expensive. In the future, we might use the plane, cylinder and sphere as prototypical parameter spaces. So far, however, those functions are only called in places where a projection would be necessary anyway, so there is no performance loss yet. And for simple cases, like a sphere, the `normal(Vec3) -> Vec3` function is even faster than `normal(f64, f64) -> Vec3`.

## Why not use the half-edge data structure?

So far, this datastructure has not been necessary. It also contradicts the "single source of truth" principle.

The problem is, that each operation now has to make sure that the next edge, the previous edge is pointing to, points back to the current edge. If just one operation in all of geop-geometry forgets to update this, the whole system breaks down.

Instead, edges are bound by points, which are checked every time an edge is created. For face boundaries, it is also checked that the edges are connected in the right order.

If you have an algorithm that is much faster with the half-edge data structure, it would be possible to implement it in a separate crate, and treat it as a "cache" layer for the actual data. This is similar for example to how the internet works. The actual data is stored in a database, and a cache layer is used to speed up access.

## Why are faces constrained by 3d edges, not by 2d edges in parameter space?

Two touching faces have to be bound by the same edge in 3d space. This is useful for watertight meshes. Geop generates watertight meshes, because first the edges are rasterized, then the inner parts of the faces, and then the gap is bridged.

It becomes more interesting in cases, where the face boundaries can only be approximated. Doing this in parameter space might seem easier at first glance, but there will still be a point in time where the 3d edge has to be created, and projected back to the parameter space of the second face. So there is not really a performance gain.

For surfaces like a sphere, the 3d edge is always well defined. It can be easily bound by circles for example. A 2d boundary in the spheres parameter space however degenerates at the poles, which would make intersection algorithms hard to implement.

## Is the border of a face still part of the face?

Good question. In general, no. Even though this might be subject to change.

So for example, the intersection of two touching faces is empty.

The intersection of a line touching a face (from outside) is 2 lines, which end and start at the touching point. This is useful for the boolean operations, as it creates a split point.

However, when checking if a point is inside a face or an edge, the function will tell you if it is actually inside, on the boundary, or outside.

