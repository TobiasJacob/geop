# Kernel Architecture

The overall kernel architecture is shown in the following figure:

- `geop-geometry`: Implements "unbounded" sets, curves, surfaces, etc. This can be a plane, a sphere, a circle, or a line.
- `geop-topology`: Implements "bounded" sets, like edges, faces, volumes.
- `geop-rasterize`: Implements rasterization algorithms that convert topological objects into triangle list, that can be rendered by a GPU.
- `geop-wgpu`: Uses the rasterizaed data and renders it using the `wgpu` crate.

