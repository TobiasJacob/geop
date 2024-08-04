# Faces

Faces are defined by a surface, an optional boundary and a possible set of holes.

```rust
pub struct Face {
    pub boundary: Option<Contour>, // Coutner-clockwise
    pub holes: Vec<Contour>,       // Clockwise
    pub surface: Rc<Surface>,
}
```

A face can look something like this:

![Face](./generated_images/topology/face1.png)

In wireframe mode, you see how the face is triangulated for rendering

![Face Wireframe](./generated_images/topology/face1wire.png)

Faces can also be non-planar, like this half sphere:

![Face Half Sphere](./generated_images/topology/face2.png)

In wireframe mode, you see how the face is triangulated for rendering

![Face Half Sphere Wireframe](./generated_images/topology/face2wire.png)

