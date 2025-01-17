# Boolean Solid Operations

3D booleans work in a similar way to 2D booleans. The first step is to find all split edges. To find these, we compare each pair of Faces and find the intersection. If the intersection is a point, we ignore it. If the intersection is a line, we add it to the list of split edges. If the intersection is a face, all boundary edges are also added to the list of split edges.

In red, you can see the split edges. Due to clipping, only 2 of the 4 split edges are visible.

![Picture](./generated_images/booleans/volume_split_edges.png)

The next step is to split each face along all possible split edges. To do this, we iterate over all faces and edges, and split the face by the edge if necessary. This is the most complex part of the boolean operation. Some edge cases that have to be considered are:

- In general, we have to split the face boundaries at the start and end of the edge.
- Now the behavior depends on of the start and end of the edge actually lie on the face boundary.
    - If none lies on the boundary, a new hole is created.
    - If only one lies on the boundary, the boundary is extended with a new path back and forth.
    - If both lie on the boundary, it depends.
        - If its the same boundary, split it into two faces.
        - If its different boundaries, connect them back and forth, and make them one boundary.

This leads to the following result

![Picture](./generated_images/booleans/face_subdivions.png)

The next step is to use an inner point on the surface to determine if a face is inside, outside, equal with normals facing in the same direction, or equal with normals facing in the opposite direction. Determining if an point is inside, outside or on the face of a volume is a simple test which can be made using a ray casting algorithm.

![Picture](./generated_images/booleans/face_classification.png)

Now, depending on which class the faces fall into, we can determine the final result of the boolean operation. For example, for a union operation, we can use the following rules:

```rust
    VolumeSplit::AinB(_) => false,
    VolumeSplit::AonBSameSide(_) => true,
    VolumeSplit::AonBOpSide(_) => false,
    VolumeSplit::AoutB(_) => true,
    VolumeSplit::BinA(_) => false,
    VolumeSplit::BonASameSide(_) => false,
    VolumeSplit::BonAOpSide(_) => false,
    VolumeSplit::BoutA(_) => true,
```

This results in

![Picture](./generated_images/booleans/volume_union_splits.png)

Now, the last step is to stich the faces back together to form shells, and then connect them back to a volume.
