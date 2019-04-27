use oak::prelude::*;

#[oak_start]
pub fn main() {
    App::new()
        .with(Store::new(0, |count, other| count + other))
        .with(View::new(|world, count| {
            (
                button.on(click(1)).push("+"),
                count,
                button.on(click(-1)).push("-"),
            )
        }))
        .mount("body")
}
