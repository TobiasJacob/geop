# Intersections

We do support all sorts of intersections between curves and surfaces.

## Curve-Curve Intersections

Curve Curve intersections result in one of three cases:
- Intersection is a curve. This can only happen with two equal curves.
- Intersection is a finite set of discrete points
- Intersection is a infinite set of discrete points (this can happen between a helix and a line). This is just models as a `PointArray`, a set of equidistant points.
- No intersection

```rust
pub enum CurveCurveIntersection {
    None,
    FinitePoints(Vec<Point>),
    InfiniteDiscretePoints(PointArray),
    Curve(Curve),
}
```

**One very imporant thing to understand is that curve-curve intersections cannot result in something like a short edge and a point. The intersection is EITHER the same curve, OR a set of points.** This follows from one of the fundamental properties of curves. The proof looks something along the lines of:

1. Let `C1` and `C2` be two curves.
1. Let `P` be an intersection point on `C1` and `C2`.
1. Now do the taylor expansion of `C1` and `C2` around `P`.
1. Either the taylor expansions are equal, or they are not.
    1. If they are equal, then `C1` and `C2` are the same curve.
    1. If they are not equal, then there is an infinitly small neighborhood around `P` where `C1` and `C2` are not equal, hence `P` is a discrete intersection point.

Similar proofs can be made for surfaces etc.

## Curve-Surface Intersections

...
