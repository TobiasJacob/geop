# Chapter 3 Topology


Infinite lines and planes are great, but also fairly limiting. We want to manufacture more than infinite planes and lines. We want to manufacture parts with holes, fillets, and other features. We want to manufacture parts with bounded surfaces and volumes. This is where topology comes in.

This crate has everything to define even complex geometries. It includes vertices, edges, faces and volumes. It also includes containment checks.

This is the numpy of CAD. Numpy is a Python library that defines a datastructure to store and manipulate n-dimensional arrays. It is the foundation of many scientific libraries in Python that use these arrays as a common language. This crate is the foundation of the modern-brep-kernel. It is like a STEP file and can represent any geometry. It itself however is not very useful, but other crates provide functions to create and manipulate these structures.

This is why boolean operations and rasterization are in separate crates. You can also create your own extension crates that define new operations on these structures like fillets, or part libraries that define standard parts like screws or nuts.
