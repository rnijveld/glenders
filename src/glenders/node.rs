// use std::rc::Rc;
// use super::vec::Vec3;
//
// trait Node {
//     fn parent(&self) -> Option<Rc<Node>>;
//     fn children(&self) -> &Vec<Rc<Node>>;
//     fn position(&mut self) -> &mut Vec3;
// }
//
// struct Object {
//
// }
//
// struct Scene {
//     children: Vec<Rc<Node>>,
// }
//
// impl Node for Scene {
//     fn parent(&self) -> Option<Rc<Node>> {
//         None
//     }
//
//     fn children(&self) -> &Vec<Rc<Node>> {
//         &self.children
//     }
//
//     fn position(&mut self) -> &mut Vec3 {
//         Vec3::new(0.0, 0.0, 0.0)
//     }
// }
