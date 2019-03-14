pub enum Event<Msg> {
    Mouse(Handler<MouseName, MouseEvent, Msg>),
}

pub struct Handler<Name, Event, Msg> {
    pub name: Name,
    pub prevent_default: bool,
    pub stop_propagation: bool,
    pub func: Box<Fn(Event) -> Msg>,
}

pub enum MouseName {
    Click,
}

pub struct MouseEvent;
