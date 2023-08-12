// use std::rc::Rc;

// use geop_geometry::geometry::points::point::Point;

// use super::FaceGroup::FaceGroup;


// pub struct Volume {
//     pub outer_face: Rc<FaceGroup>,
//     pub inner_faces: Vec<Rc<FaceGroup>>
// }

// impl Volume {
//     pub fn new(outer_face: Rc<FaceGroup>, inner_faces: Vec<Rc<FaceGroup>>) -> Volume {
//         Volume {
//             outer_face,
//             inner_faces
//         }
//     }

//     pub fn rasterize(&self) -> Vec<Vec<Vec<Point>>> {
//         todo!("Rasterize the volume")
//     }
// }