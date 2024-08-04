# Rasterization

Most stuff is pretty easy to rasterize. Just creating watertight faces is a bit thougher.

The first step is to rasterize the edges. This will ensure that all faces are bounded by the exact same edge vertices.

Then we generate a point grid on the surface and filter out all points that are not inside the face.

Then we use delauny triangulation.

![Rasterization](./generated_images/topology/face1wire.png)
