# Geometry

Geometry refers to structures that represent unbounded / inifinite objects, like planes, spheres, circles, lines, etc. These objects are used to define the shape of the objects in the scene. The simples case are curves.

> **Note**: For rendering, we have to somehow bound the geometry, as we cannot render infinite objects. Just ignore the points and the boundaries of the surfaces.

## Curves

This is a line, defined by a basis and a direction.

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

