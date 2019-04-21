## TODO

-   [ ] Event handling
-   [ ] HTTP requests
-   [ ] Timers
-   [ ] Sanitize HTML text
-   [ ] Allow trailing spaces in HTML text

## Benchmarks

### DBMon

https://mathieuancelin.github.io/js-repaint-perfs/

https://github.com/mathieuancelin/js-repaint-perfs

### JS Framework Benchmarks

https://krausest.github.io/js-framework-benchmark/current.html

https://github.com/krausest/js-framework-benchmark

### Marko Benchmarks

https://markojs.com/isomorphic-ui-benchmarks/

https://markojs.com/isomorphic-ui-benchmarks/benchmark/search-results/

https://markojs.com/isomorphic-ui-benchmarks/benchmark/color-picker/

https://github.com/marko-js/isomorphic-ui-benchmarks

### UI Bench

https://localvoid.github.io/uibench/

https://github.com/localvoid/uibench-react/tree/master/js

https://github.com/Freak613/stage0/blob/master/examples/uibench/app.js

## Wishlist

-   GraphQL integration
-   html!/css! macros
-   Built-in router
-   Compile-time HTML/CSS validation
-   Render CSS inline, in style block, or in static file
-   Threading with web workers
-   CLI to create/build/test/optimize projects
-   Optionally generate custom elements
-   Optional Shadow DOM
-   Document fragments <></>
-   Static site rendering
-   Server-side rendering
-   Server-side rendering w/ client hydration
-   Client-side app

## Inspiration

-   https://github.com/DenisKolodin/yew
-   https://github.com/csharad/ruukh
-   https://github.com/rbalicki2/smithy
-   https://github.com/David-OConnor/seed
-   https://github.com/utkarshkukreti/draco
-   https://github.com/bodil/typed-html
-   https://github.com/sindreij/willow
-   https://github.com/chinedufn/percy
-   https://elm-lang.org/
-   https://github.com/dpc/stpl

## Crates

oak
oak_core
oak_dom
oak_events
oak_html
oak_http
oak_i18n
oak_router
oak_sse
oak_svg
oak_time
oak_ws

## Example imports

oak::core::shrev
oak::core::shred
oak::core::specs
oak::prelude::\*
oak::dom::VirtualNode
oak::dom::VirtualNodeSystem
oak::dom::BrowserNode
oak::dom::BrowserNodeSystem
oak::dom::ParentNode
oak::time::Time
oak::time::Duration
oak::http::Request
oak::http::Response
oak::http::Client
oak::events::Event
oak::events::BrowserEventSystem

## Fake code

### Hello

```rs
use oak::prelude::*;

fn view(name: &str) -> Html {
    html! {
        <h1>Hello, { name }!</h1>
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    App::with_state("World")
        .with_view(view)
        .start("body")
}
```

### Counter

```rs
use oak::prelude::*;

enum Msg {
    Increment,
    Decrement
}

fn update(count: i16, msg: Msg) -> ( i16, Cmd ) {
    match msg {
        Msg::Increment => ( count + 1, Cmd::NONE ),
        Msg::Decrement => ( count - 1, Cmd::NONE ),
    }
}

fn view(count: i16) -> Html {
    html! {
        <button onclick=Msg::Increment>+</button>
        <div>{ count }</div>
        <button onclick=Msg::Decrement>-</button>
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    App::with_state(0i16)
        .with_view(view)
        .with_update(update)
        .start("body")
}
```

### Current time

```rs
use oak::prelude::*;

fn update(old: Time, new: Time) -> ( Time, Cmd ) {
    ( new, Cmd::NONE )
}

fn view(time: Time) -> Html {
    html! {
        The current time is: { time }
    }
}

fn subs(_: Time) -> Sub {
    Time::every(Duration::from_secs(1))
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    App::with_state(Time::new())
        .with_view(view)
        .with_update(update)
        .with_subs(subs)
        .start("body")
}
```

### Fetch

https://guide.elm-lang.org/effects/http.html

```rs
use oak::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    App::with_init(init)
        .with_view(view)
        .with_update(update)
        .start("body")
}

enum Model {
    Failure,
    Loading,
    Success(String),
}

fn init() -> ( Model, Cmd ) {
    ( Model::Loading, http::get("https://elm-lang.org/assets/public-opinion.txt") )
}

fn update(model: Model, msg: Result<String, http::Error>) -> ( Model, Cmd ) {
    ( msg
        .map(Model::Success)
        .map_err(|_| Model::Failure), Cmd::NONE )
}

fn view(model: Model) -> Html {
    match model {
        Model::Failure => html! { I was unable to load your book },
        Model::Loading => html! { Loading... },
        Model::Success(text) => html! { <pre>{ text }</pre> }
    }
}
```
