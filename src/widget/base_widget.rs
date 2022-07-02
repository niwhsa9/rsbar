
pub trait Widget {
    fn get_text(&mut self) -> String;
    fn get_icon(&self) -> String;
}