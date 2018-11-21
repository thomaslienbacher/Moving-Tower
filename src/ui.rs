use sfml::window::Event;

pub trait UiWidget {
    fn new() -> Self;

    fn event(&mut self, evt: Event);
}

pub struct UiButton {
    
}