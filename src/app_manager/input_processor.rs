use crate::directory::path_map::PathMap;
use crate::view::view::View;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
pub trait InputProcessor {
    fn process_input(&self, 
    input: String,
    pwd: Arc<RwLock<String>>,
    view: &Rc<RefCell<Box<dyn View>>>,
    is_threading: &mut bool,
    ) -> bool;   
}
