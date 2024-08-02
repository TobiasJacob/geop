# Geometry

Geometry refers to structures that represent unbounded / inifinite objects, like planes, spheres, circles, lines, etc. These objects are used to define the shape of the objects in the scene. The simples case are curves.

> **Note**: For rendering, we have to somehow bound the geometry, as we cannot render infinite objects. Just ignore the points and the boundaries of the surfaces.

## Curves

A curve is something that could be thought of as a function that maps a number to a point in space. Right now, geop implements only two primitive curves. This is a line, defined by a basis and a direction.

```rust
let line = Line::new(
    Point3::new(0.0, 0.0, 0.0),
    Vector3::new(1.0, 1.0, 1.0),
);
```

![Line](./generated_images/geometry/primitive_line.png)

Next is a circle, defined by a center, a normal and a radius. The radius is a point, indicating where the circle starts or is at 0.

```rust
let circle = Circle::new(
    Point3::new(0.0, 0.0, 0.0),
    Vector3::new(0.0, 0.0, 1.0),
    1.0,
);
```

![Circle](./generated_images/geometry/primitive_circle.png)

### Do not expose the parameters

So technically, all the curves are something like $$c(u): \mathbb{R} \rightarrow \mathbb{R}^3, u \rightarrow p$$. But we don't want to use the parameters, as they are not very dangerous and misleading in general. For example, imagine writing a function that checks if a point is in an interval. You would write something like this:

```rust
// Checks if p is between a and b
fn is_in_interval(p: Point3, a: Point3, b: Point3) -> bool {
    let u_p = unproject(p);
    let u_a = unproject(a);
    let u_b = unproject(b);
    u_p >= u_a && u_p <= u_b
}
```

However, this code does not work for a circle. A circle has no unique mapping from $p$ to $u$. So, we have to avoid using the parameters. This is why the curves do not expose the parameters. 

### How to interact with curves

How do we interact with curves? We can do this by using the `Curve` trait. It implements the following methods:

```rust
// Transform
pub fn transform(&self, transform: Transform) -> Curve;

// Change the direction of the curve
pub fn neg(&self) -> Curve;

// Normalized Tangent / Direction of the curve at the given point.
pub fn tangent(&self, p: Point) -> Point;

// Checks if point is on the curve.
pub fn on_curve(&self, p: Point) -> bool;

// Interpolate between start and end at t. t is between 0 and 1.
pub fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point;

// Checks if m is between x and y. m==x and m==y are true.
pub fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> bool;

// Get the midpoint between start and end.
// This will guarantee that between(start, midpoint, end) is true and midpoint != start and midpoint != end.
// If start or end is None, the midpoint is a point that is a unit distance away from the other point.
pub fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Point;

// Finds the closest point on the curve to the given point.
pub fn project(&self, p: Point) -> Point;
```

These are all the methods that have to be used to interact with curves. You might notice, that interpolate, between and get_midpoint functions accept Optionals. This is for cases where we work with infinite edges. For example, a line that starts somewhere and goes to infinity would work with `start = Some(Point3::new(0.0, 0.0, 0.0))` and `end = None`. Circles also frequently don't have a start and end point, so we have to use Optionals.


### Intersections

Intersections are fully supported between all curves. Keep in mind, that intersections are not always a single point. For example, two lines can intersect in one point, but they can also intersect in infinitely many points if they are the same line, or in no points if they are parallel. This is represented by the enum data type that is returned by the intersection function.

```rust
pub enum LineLineIntersection {
    Line(Line),
    Point(Point),
    None,
}

pub fn line_line_intersection(a: &Line, b: &Line) -> LineLineIntersection;
```

Take a look at the different cases:

