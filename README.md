# Geop: Geometric Operations CAD Kernel
**A modern CAD kernel using Riemannian Manifolds**

Geop is a modern CAD Kernel. It uses Riemannian Manifolds to be numerically stable and accurate. It is designed to be fast and efficient. It is designed to be used in a variety of applications, including CAD, CAM, and CAE.

Documentation: [Modern Brep Kernel Book](https://tobiasjacob.github.io/geop/)

- :white_check_mark: **Testing and documenting as a book:** CAD Kernels are hard to debug and to explain, as the algorithms become very theoretical. This is what the modern-brep-kernel-book is for. The graphics do not only serve as a visual representation of the algorithms, but also as a way to test them. The book is written in markdown and is meant to be an introduction for new developers and users.
- :speech_balloon: **Accurate results:** Makes use of Rusts expressive type system to ensure correctness. For example, the intersection result of two lines is
    ```rust
    pub enum LineLineIntersection {
        Line(Line),
        Point(Point),
        None,
    }

    pub fn line_line_intersection(a: &Line, b: &Line) -> LineLineIntersection;
    ```
- :rock: **Solid mathematical foundation:** We use Riemannian Manifolds to ensure numerical stability and accuracy.
- :heart: **Simplicity is key:** We avoid the half edge datastructure. Topological structures are contained by simple structs. We also avoid the use of abstract data types, instead curves and surfaces are represented as enum. This makes the code easier to understand and to use. Last but not least, we make sure to never expose the parameters of the geometric objects, but always work with reference points in 3D.
- :100: **Code coverage:** We aim for 100% code coverage.

## Geop-Geometry

1. Point: A simple point in 3D space.
1. Curves: 1D objects
    1. Line: An infinite line.
    1. Circle: A circle in 3D space.
1. Surfaces: 2D objects
    1. Plane: An infinite plane.
    1. Sphere: A sphere in 3D space.
    1. Cylinder (WIP): A cylinder in 3D space.

Dis crate also defines all intersections between any two combination of these objects.

## Geop-Topology

1. Edge: A curve bounded by two optional points.
1. Contour: A connected set of edges.
1. Face: A surface optionally bounded by a contour and with a set of holes.
1. Shell: A set of faces that close up a defined area in space.
1. Volume: A area in space bounded by a shell, with optional holes bounded by a shell.

This crate also defines the contains operation, and has functions to create simple primitives.

## Geop-Boolean

A crate that defines the boolean operations (intersection, union, difference) between two volumes or faces.

![Boolean](./crates/modern-brep-kernel-book/src/generated_images/booleans/face_difference.png)

## Geop-Rasterize

This is used to convert the topological structures into triangles.

## Geop-Wgpu

A crate that uses wgpu to render the topological structures.

## Modern-Brep-Kernel-Book

A crate used to render the graphics for the book, and contains the book.


