# Geop: Geometric Operations CAD Kernel
## A modern CAD kernel using Riemannian Manifolds

Geop is a modern CAD Kernel. It uses Riemannian Manifolds to be numerically stable and accurate. It is designed to be fast and efficient. It is designed to be used in a variety of applications, including CAD, CAM, and CAE.

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

## Geometric data structures


## Topological data structures

1. Point
2. Edge
3. Contour
4. Face
5. Shell
6. Volume

## Contains

1.  Edge_Point
2.  Face_Point
    Face_Edge
    Face_Contour
3.  Volume_Point
    Volume_Edge
    Volume_Face
    Volume_Shell


## Intersections


