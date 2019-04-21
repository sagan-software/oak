use futures::{
    sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    Future, Stream,
};
use std::any::Any;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Element, Event, Node, Text};

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    {
        let counter_template = self::runtime::compile_template(self::counter::TEMPLATE);
        self::runtime::add_template::<self::counter::Counter>(counter_template);
        let timer_template = self::runtime::compile_template(self::timer::TEMPLATE);
        self::runtime::add_template::<self::timer::Timer>(timer_template);
    }

    let (counter_1_root, counter_1) = self::counter::create();
    let (counter_2_root, counter_2) = self::counter::create();
    let (counter_3_root, counter_3) = self::counter::create();

    let document = &self::runtime::DOCUMENT.0;

    let (timer_1_root, timer_1) = self::timer::create();

    let fragment = document.create_document_fragment();
    fragment.append_child(&counter_1_root)?;
    fragment.append_child(&counter_2_root)?;
    fragment.append_child(&counter_3_root)?;
    fragment.append_child(&timer_1_root)?;

    let mount_point = document.body().unwrap();
    mount_point.append_child(&fragment)?;

    let (msg_sender, msg_receiver): (
        UnboundedSender<(self::runtime::CowStr, Event)>,
        UnboundedReceiver<(self::runtime::CowStr, Event)>,
    ) = unbounded();
    let callback = {
        let msg_sender = msg_sender.clone();
        Closure::wrap(Box::new(move |event: Event| {
            msg_sender.unbounded_send(("click".into(), event)).unwrap();
        }) as Box<Fn(Event)>)
    };

    document
        .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
        .unwrap();
    callback.forget();

    let components = [counter_1, counter_2, counter_3];

    future_to_promise(
        msg_receiver
            .fold(components, move |mut components, (name, event)| {
                log::info!("event: {} {:#?}", name, event);
                let target = event.target().unwrap().unchecked_into::<Node>();
                for c in &mut components {
                    if target.is_same_node(Some(&c.increment_event.el)) {
                        if let Some(msg) = (c.increment_event.func)(&event) {
                            self::counter::reduce(c, msg);
                            self::counter::update(c);
                        }
                        break;
                        // TODO bubble up
                    }
                    if target.is_same_node(Some(&c.decrement_event.el)) {
                        if let Some(msg) = (c.decrement_event.func)(&event) {
                            self::counter::reduce(c, msg);
                            self::counter::update(c);
                        }
                        break;
                        // TODO bubble up
                    }
                }
                // for (msg) in msgs.into_iter() {
                //     msg_sender.unbounded_send(msg).unwrap();
                // }
                Ok(components)
            })
            .map(|_| JsValue::NULL)
            .map_err(|_| JsValue::NULL),
    );

    Ok(())
}

mod runtime {
    use lazy_static::lazy_static;
    use std::any::TypeId;
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use wasm_bindgen::JsCast;

    pub struct Window(web_sys::Window);

    unsafe impl Sync for Window {}

    lazy_static! {
        pub static ref WINDOW: Window = {
            let window = web_sys::window().expect("no window");
            Window(window)
        };
    }

    pub struct Document(pub web_sys::Document);

    unsafe impl Sync for Document {}

    lazy_static! {
        pub static ref DOCUMENT: Document = {
            let document = WINDOW.0.document().expect("no document");
            Document(document)
        };
    }

    struct CompilerTemplate(web_sys::HtmlTemplateElement);

    unsafe impl Sync for CompilerTemplate {}

    lazy_static! {
        static ref COMPILER_TEMPLATE: CompilerTemplate = {
            let compiler_template = DOCUMENT
                .0
                .create_element("template")
                .unwrap()
                .unchecked_into::<web_sys::HtmlTemplateElement>();
            CompilerTemplate(compiler_template)
        };
    }

    pub type CowStr = Cow<'static, str>;

    pub fn compile_template<S: Into<CowStr>>(html: S) -> web_sys::Element {
        COMPILER_TEMPLATE.0.set_inner_html(html.into().trim());
        COMPILER_TEMPLATE
            .0
            .content()
            .first_element_child()
            .expect("first child")
    }

    struct Templates(Mutex<HashMap<TypeId, web_sys::Element>>);

    unsafe impl Sync for Templates {}

    lazy_static! {
        static ref TEMPLATES: Templates = { Templates(Mutex::new(HashMap::new())) };
    }

    pub fn add_template<T: 'static>(el: web_sys::Element) {
        let type_id = TypeId::of::<T>();
        TEMPLATES.0.lock().unwrap().insert(type_id, el);
    }

    pub fn get_template<T: 'static>() -> web_sys::Element {
        let type_id = TypeId::of::<T>();
        TEMPLATES
            .0
            .lock()
            .unwrap()
            .get(&type_id)
            .unwrap()
            .clone_node_with_deep(true)
            .unwrap()
            .unchecked_into()
    }

    pub fn has_template<T: 'static>() -> bool {
        let type_id = TypeId::of::<T>();
        TEMPLATES.0.lock().unwrap().contains_key(&type_id)
    }
}

pub struct EventListeners<Msg> {
    el: web_sys::Element,
    func: Box<dyn Fn(&Event) -> Option<Msg>>,
}

// pub struct Component<State, Msg> {
//     pub state: State,
//     pub root: Element,
//     pub elements: Vec<Element>,
//     pub listeners: Vec<EventListeners<Msg>>,
// }

mod counter {
    use super::EventListeners;
    use web_sys::Element;

    #[derive(Clone, Debug)]
    pub enum Action {
        Increment,
        Decrement,
    }

    pub struct Counter {
        pub count: i16,
        count_el: Element,
        pub increment_event: EventListeners<Action>,
        pub decrement_event: EventListeners<Action>,
    }

    pub fn reduce(counter: &mut Counter, msg: Action) {
        match msg {
            Action::Increment => counter.count += 1,
            Action::Decrement => counter.count -= 1,
        }
    }

    pub const TEMPLATE: &str = r"<div>
        <button>+</button>
        <div></div>
        <button>-</button>
    </div>";

    pub fn create() -> (Element, Counter) {
        let count = 0_i16;
        let el = super::runtime::get_template::<Counter>();
        let increment_el = el.first_element_child().unwrap();
        let count_el = increment_el.next_element_sibling().unwrap();
        let decrement_el = count_el.next_element_sibling().unwrap();

        count_el.set_text_content(Some(&count.to_string()));
        increment_el.set_text_content(Some("+"));

        (
            el,
            Counter {
                count,
                count_el,
                increment_event: EventListeners {
                    el: increment_el,
                    func: Box::new(|_| Some(Action::Increment)),
                },
                decrement_event: EventListeners {
                    el: decrement_el,
                    func: Box::new(|_| Some(Action::Decrement)),
                },
            },
        )
    }

    pub fn update(counter: &Counter) {
        counter
            .count_el
            .set_text_content(Some(&counter.count.to_string()));
    }

}

mod timer {
    use js_sys::Date;
    use web_sys::{Element, Text};

    #[derive(Clone, Debug)]
    pub enum Action {
        Tick,
    }

    pub const TEMPLATE: &str = r"<time>Alive for <!----> seconds</time>";

    pub struct Timer {
        pub start_time: f64,
        pub current_time: f64,
        pub text_node: Text,
    }

    pub fn create() -> (Element, Timer) {
        let start_time = Date::now();
        let current_time = start_time.clone();
        let el = super::runtime::get_template::<Timer>();
        let text_node = super::runtime::DOCUMENT.0.create_text_node("0");
        let comment_node = el.first_child().unwrap().next_sibling().unwrap();
        el.replace_child(&text_node, &comment_node).unwrap();
        (
            el,
            Timer {
                start_time,
                current_time,
                text_node,
            },
        )
    }
}
