use std::any::Any; //allow downcasting

pub trait View{
    fn as_any(&mut self) -> &mut dyn Any; //used for downcasting (but keeping object safe)
    fn change_view(&mut self, new_view: Box<dyn std::any::Any>);
    fn print_view(&self);
}