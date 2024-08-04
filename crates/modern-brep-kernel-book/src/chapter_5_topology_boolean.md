# Boolean Operations

Boolean operations are pretty complex. This crate implements 2d and 3d booleans based on the datastructure.

In general, they work by splitting the faces at the intersection points and then filtering out the parts that are inside or outside the other face. Then the parts are reassembled.
