use crate::directory::path_map::PathMap;
use crate::view::view::View;
use std::rc::Rc;
use std::cell::RefCell;

pub trait InputProcessor {
    fn process_input(&self, 
    input: String,
    path_map: &mut PathMap,
    pwd: &mut String,
    view: &Rc<RefCell<Box<dyn View>>>,
    is_threading: &mut bool,
    ) -> bool;   
}
