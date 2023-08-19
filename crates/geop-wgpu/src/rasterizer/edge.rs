
pub fn point_at(&self, u: f64) -> Point {
    let mut u = u % 1.0;
    u = u * self.edges.len() as f64;
    let i = u.floor() as usize;
    u = u - i as f64;
    let edge = self.edges[i].clone();
    edge.point_at(u)
}

pub fn project(&self, point: &Point) -> Option<f64> {
    let mut u = 0.0;
    for edge in self.edges.iter() {
        match edge.project(point) {
            Some(u_p) => {
                return Some((u + u_p) / self.edges.len() as f64);
            }
            None => {
                u += 1.0;
            }
        }
    }
    None
}

pub fn rasterize(&self) -> Vec<Point> {
    self.edges
        .iter()
        .map(|edge| edge.rasterize())
        .flatten()
        .collect()
}
