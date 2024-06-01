# Chapter 3 Topology

## Introduction

Infinite lines and planes are great, but also fairly limiting. We want to manufacture more than infinite planes and lines. We want to manufacture parts with holes, fillets, and other features. We want to manufacture parts with bounded surfaces and volumes. This is where topology comes in.

## Topology

Topology is the study of properties of space that are preserved under continuous deformations, such as stretching, crumpling, and bending, but not tearing or gluing. In the context of computer-aided design, topology is the study of bounded sets, such as edges, faces, and volumes. Take a look at the following figure:

![Topology](./images/topology.png)

## Checking if something is inside or outside a contour


Take a look at the following picture.

![Inside or Outside](./images/indside_outside.png)

Imagine we are at a corner of a contour. We want to check if a point is inside or outside the contour. We can do this by drawing a line from the point to the corner (ray). If the line crosses the contour an odd number of times, the point is inside the contour. If the line crosses the contour an even number of times, the point is outside the contour.

Alternatifly, we can also take a look at the direction of the closest intersection. If we normalize -ray, -tangent_1 and tangent_2, if -ray is on the top side of the green dashed line, we know it comes from inside. We can also create a coordinate system (red lines and normal) and check its handedness. It switches handed ness when -ray is crossing the contour.
