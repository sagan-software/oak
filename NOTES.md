ECS brainstorming

- Components

  - RouteComponent
  - PointerEvent
  - Mouse

- Systems
  - BrowserRenderer

# hello

```rust
use oak::World;
use oak::markup::MarkupComponent;
use oak::html::{div, text};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let mut world = World::new();
    world.register::<MarkupComponent>();

    world.create_entity().with(MarkupComponent::new(
        div()
    ))
}

```

```json
{
  "id": "hello-example",
  "components": [
    {
      "type": "Markup",
      "value": {
        "type": "text",
        "value": "Hello"
      }
    },
    {}
  ]
}
```
