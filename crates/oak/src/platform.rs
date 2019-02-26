use wasm_bindgen::JsValue;

pub trait Cmd<Msg> {
    fn run(&self) -> Result<(), JsValue>;

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub trait Sub<Msg> {
    fn init(&self) -> Result<(), JsValue>;
}


pub struct None;

impl<Msg> Cmd<Msg> for None {
    fn run(&self) -> Result<(), JsValue> {
        Ok(())
    }
}

impl<Msg> Sub<Msg> for None {
    fn init(&self) -> Result<(), JsValue> {
        Ok(())
    }
}