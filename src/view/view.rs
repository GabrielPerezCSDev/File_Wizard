use std::any::Any; //allow downcasting
use crate::PathMap;
use crate::AppManager;
pub trait View{
    fn as_any(&mut self) -> &mut dyn Any; //used for downcasting (but keeping object safe)
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn change_view(&mut self, new_view: Box<dyn std::any::Any>);
    fn print_view(&self, path_map: &PathMap, url: &str, app_manager: &AppManager);
}