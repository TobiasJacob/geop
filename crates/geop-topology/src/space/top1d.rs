// struct Top1dCircularValue {
//     pub value: f64,
// }

// struct Top1dCircularInterval {
//     pub start: f64,
//     pub end: f64,
// }

// struct Top1dCircular {
//     pub period: f64,
// }

// pub impl Top1dCircular {
//     pub fn order(start: f64, end: f64) -> (f64, f64) {
//         if start <= end {
//             (start, end)
//         } else {
//             (start, end + self.period)
//         }
//     }

//     pub fn within(&self, start: f64, end: f64, point: f64) -> bool {
//         let (start, end) = self.order(start, end);
//         start <= point && point <= end
//     }

//     pub fn intersect(&self, start1: f64, end1: f64, start2: f64, end2: f64) -> (f64, f64) {
//         let (start1, end1) = self.order(start1, end1);
//         let (start2, end2) = self.order(start2, end2);

//         if end1 < start2 || end2 < start1 {
//             return (0.0, 0.0);
//         }

//         if start1 <= start2 && end2 <= end1 {
//             return (start2, end2);
//         }
    
//         if start1 <= start2 && start2 <= end1 {
//             return (start2, end1);
//         }

//         if start2 <= start1 && end1 <= end2 {
//             return (start1, end1);
//         }

//         if start2 <= start1 && start1 <= end2 {
//             return (start1, end2);
//         }

//         (0.0, 0.0)
//     }
// }
