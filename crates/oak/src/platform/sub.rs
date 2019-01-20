use wasm_bindgen::JsValue;

pub trait Sub<Msg> {
    fn run(&self) -> Result<(), JsValue>;
}

pub struct None;

impl<Msg> Sub<Msg> for None {
    fn run(&self) -> Result<(), JsValue> {
        Ok(())
    }
}
