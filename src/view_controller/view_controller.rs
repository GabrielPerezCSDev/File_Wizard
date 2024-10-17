use crate::AppManager;
pub trait ViewController {
    fn get_input(&self) -> String;
    fn display_output(&self, output: &str);
    fn show_view(&self, url: &str, app_manager: &AppManager);
    fn change_view(&self);
    fn init_view(&self);
}

