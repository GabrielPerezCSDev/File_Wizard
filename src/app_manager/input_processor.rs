use crate::directory::path_map::PathMap;

pub trait InputProcessor {
    fn process_input(&self, input: String, path_map: &mut PathMap, url: &mut String) -> bool;   
}
