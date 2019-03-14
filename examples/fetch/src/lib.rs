use oak::prelude::*;
use oak::http::{fetch, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
struct State {
    response: Option<Response<Branch>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Branch {
    name: String,
    commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
struct Commit {
    sha: String,
    commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommitDetails {
    author: Signature,
    committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
struct Signature {
    name: String,
    email: String,
}

impl Handler<Response<Branch>> for State {
    fn handle(&mut self, msg: &Response<Branch>) {
        self.response = Some(response.clone());
    }
}

fn init() -> State {

}

fn view(model: &Model) -> Html {
    div()
        .with_child(button().on(click(Msg::Increment)).with_child("+"))
        .with_child(model.count)
        .with_child(button().on(click(Msg::Decrement)).with_child("-"))
        .into()
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    oak::with_state(Model::default())
        .with_view(view)
        .mount("body")
}