# Contours

A contour is a connected vector of edges. It is used to define the boundary of a face. A contour has to be closed, meaning that the first and last edge have to be the same. 

```rust
pub struct Contour {
    pub edges: Vec<Edge>,
}
```

![Contour](./generated_images/topology/contours.png)
