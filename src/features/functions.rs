use super::cell;
pub fn name_from_id<T: Into<String>>(id: T) -> String { 
    id.into()
}
// pub fn new(gx: u32, gy: u32, direction: cell::Direction, id: &str) -> cell::Cell{ 
//     return cell::Cell { gx: gx, gy: gy, direction: direction, id: id.to_string(),   name: name_from_id(id), category: vec!["mover", "default"].iter().map(|x|     x.to_string()).collect() };
// }