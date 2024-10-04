use crate::directory::path_map::PathMap;

pub trait ViewController {
    fn get_input(&self) -> String;
    fn display_output(&self, output: &str);
    fn show_view(&self, path_map: &PathMap, url: &str);
}

