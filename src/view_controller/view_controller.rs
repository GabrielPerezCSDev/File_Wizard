use crate::directory::path_map::PathMap;
use crate::view::view::View;
pub trait ViewController {
    fn get_input(&self) -> String;
    fn display_output(&self, output: &str);
    fn show_view(&self, path_map: &PathMap, url: &str);
    fn change_view(&self);
    fn init_view(&self);
}

