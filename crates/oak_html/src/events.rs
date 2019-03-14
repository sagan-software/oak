use oak_vdom::{Event, EventHandler};

pub fn click<Msg: Clone + 'static>(msg: Msg) -> Event<Msg> {
    let func = move |_| -> Msg { msg.clone() };
    let handler = EventHandler::new(func);
    Event("click".to_owned(), handler)
}
