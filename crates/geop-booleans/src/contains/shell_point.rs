use crate::point::Point;

use crate::shell::Shell;

use super::face_point::{face_point_contains, FacePointContains};

pub fn shell_point_contains(shell: &Shell, point: Point) -> FacePointContains {
    for face in shell.faces.iter() {
        let contains: FacePointContains = face_point_contains(face, point);
        match contains {
            FacePointContains::OnEdge(edge) => {
                return FacePointContains::OnEdge(edge);
            }
            FacePointContains::OnPoint(point) => {
                return FacePointContains::OnPoint(point);
            }
            FacePointContains::Inside => {
                return FacePointContains::Inside;
            }
            FacePointContains::Outside => {}
            FacePointContains::NotOnSurface => {}
        }
    }
    return FacePointContains::Outside;
}
