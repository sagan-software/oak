use crate::markup::Attribute;

pub fn on_click<Msg>(msg: Msg) -> Attribute<Msg> {
    Attribute::SimpleEvent("click".to_owned(), msg)
}

pub fn on_double_click<Msg>(msg: Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

pub fn on_mouse_down<Msg>(msg: Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

pub fn on_mouse_up<Msg>(msg: Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

pub fn on_mouse_enter<Msg>(msg: Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

pub fn on_mouse_leave<Msg>(msg: Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

pub fn on_mouse_over<Msg>(msg: Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

pub fn on_mouse_out<Msg>(msg: Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

pub fn on_input<Msg>(to_msg: fn(String) -> Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

pub fn on_check<Msg>(to_msg: fn(bool) -> Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

pub fn on_submit<Msg>(msg: Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

pub fn on_blur<Msg>(msg: Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

pub fn on_focus<Msg>(msg: Msg) -> Attribute<Msg> {
    Attribute::Bool("tmp".to_owned())
}

// on_mouse_wheel
// on_scroll
// on_drag_start
// on_drag
// on_drag_end
// on_drag_enter
// on_drag_leave
// on_drag_over
// on_drag_exit
// on_drop
// on_context_menu
// on_change
// on_pointer_up
