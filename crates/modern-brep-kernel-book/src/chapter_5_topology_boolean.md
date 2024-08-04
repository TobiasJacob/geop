# Boolean Operations

Boolean operations are pretty complex. This crate implements 2d and 3d booleans based on the datastructure.

What makes this so tricky is that there are a lot of cases where lines are parallel or overlapping and so on. In most software they are edge cases and occur rarely, but for a CAD kernel they occur quite frequently. Take for example a face that is extruded until it reaches another face, and then you want to compute the union. This is a very common operation in CAD software. Treating this correctly is very important.

In general, they work by splitting the faces at the intersection points and then filtering out the parts that are inside or outside the other face. Then the parts are reassembled.
