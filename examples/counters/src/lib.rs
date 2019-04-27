use oak::prelude::*;

#[oak_start]
pub fn main() {
    oak::run("body", |world| {
        world
        .create_entity()
        .with(state(()))
        .with(reducer(|_, _|))
    })
}

fn counter(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(reducer(|world, _, other| count + other))
        .with(view(|count| {
            (
                button.on(click(1)).push("+"),
                count,
                button.on(click(-1)).push("-"),
            )
        }))
}

type Model = Vec<i32>;

enum Msg {
    CreateCounter
    RemoveCounter(usize)
    CounterMsg(usize, i32)
}

fn update(model: &mut Model, msg: &Msg) {
    match msg {
        Msg::CreateCounter => model.push(0),
        Msg::RemoveCounter(index) => {model.remove(index);},
        Msg::CounterMsg(index, msg) => {
            self::counter::update(&mut model[index], msg);
        }
    }
}

fn view(model: &Model) -> impl Html<Msg> {

    div.inner((
        button.onclick(Msg::CreateCounter).inner("New Counter"),
        model.iter().map(|c| )
    ))
}

mod counter {
    fn update(count: &mut i32, other: i32) -> i32 {
        count + other
    }

    fn view(count: i32) -> impl Html<i32> {

    }
}
