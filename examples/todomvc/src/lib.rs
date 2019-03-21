use oak::prelude::*;

#[wasm_bindgen]
pub fn main() -> AppResult {
    App::update(update).view(view).mount_to_body()
}

#[derive(Default)]
struct Model {
    tasks: Vec<Task>,
    field: String,
    id: usize,
    visibility: Visibility,
}

#[derive(Default, Debug)]
struct Task {
    description: String,
    completed: bool,
    edits: Option<String>,
    id: usize,
}

#[derive(PartialEq, Clone, Debug)]
enum Visibility {
    All,
    Active,
    Completed,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::All
    }
}

#[derive(Clone, Debug)]
enum Msg {
    UpdateField(String),
    ChangeVisibility(Visibility),
    CheckAll(bool),
    Keydown(String),
    DeleteTask(usize),
    ToggleCompleted(usize),
    ClearCompleted,
}

fn update(mut model: Model, msg: Msg) -> Model {
    match msg {
        Msg::UpdateField(field) => {
            log::info!("!!! {}", field);
            model.field = field;
        }
        Msg::ChangeVisibility(visibility) => {
            model.visibility = visibility;
        }
        Msg::CheckAll(completed) => {
            log::info!("COMPLETED???? {}", completed);
            model.tasks.iter_mut().for_each(|t| t.completed = completed);
        }
        Msg::Keydown(code) => {
            if code == "Enter" {
                model.tasks.push(Task {
                    description: model.field.clone(),
                    completed: false,
                    edits: None,
                    id: model.id,
                });
                model.id += 1;
                model.field = String::new();
            }
        }
        Msg::DeleteTask(id) => {
            log::info!("DELETING TASK: {:#?}", id);
            log::info!("ALL TASKS: {:#?}", model.tasks);
            model.tasks = model.tasks.into_iter().filter(|t| t.id != id).collect();
        }
        Msg::ToggleCompleted(id) => {
            log::info!("TOGGLE COMPLETED {}", id);
            model
                .tasks
                .iter_mut()
                .filter(|t| t.id == id)
                .for_each(|t| t.completed = !t.completed);
        }
        Msg::ClearCompleted => {
            model.tasks = model.tasks.into_iter().filter(|t| !t.completed).collect();
        }
    }
    model
}

fn view(model: &Model) -> HtmlElement<Msg> {
    div()
        .set(class("todomvc-wrapper"))
        .set(style("visibility:hidden;"))
        .push(
            section()
                .set(class("todoapp"))
                .push(view_task_entry(&model.field))
                .push(view_task_list(&model.visibility, &model.tasks))
                .push(view_controls(&model.visibility, &model.tasks)),
        )
        .push(view_info_footer())
}

fn view_task_entry(text: &str) -> HtmlElement<Msg> {
    header().set(class("header")).push(h1().push("todos")).push(
        input()
            .set(class("new-todo"))
            .set(placeholder("What needs to be done?"))
            .set(autofocus())
            .set(value(text))
            .set(on_input(Msg::UpdateField))
            .set(on_keydown(Msg::Keydown))
            .set(name("newTodo")),
    )
}

fn view_task_list(visibility: &Visibility, tasks: &[Task]) -> HtmlElement<Msg> {
    let all_completed = tasks.iter().all(|t| t.completed);
    log::info!("????????????????????????????????? {}", all_completed);
    section()
        .set(class("main"))
        .set(style(if tasks.is_empty() {
            "visibility:hidden"
        } else {
            "visibility:visible"
        }))
        .push(
            input()
                .set(class("toggle-all"))
                .set(id("toggle-all"))
                .set(type_("checkbox"))
                .set(name("toggle"))
                .set(on_click(Msg::CheckAll(!all_completed))),
        )
        .push(label().set(for_("toggle-all")).push("Mark all as complete"))
        .push(
            ul().set(class("todo-list")).push_iter(
                tasks
                    .iter()
                    .filter(|t| match visibility {
                        Visibility::Completed => t.completed,
                        Visibility::Active => !t.completed,
                        Visibility::All => true,
                    })
                    .map(view_task),
            ),
        )
}

fn view_task(task: &Task) -> HtmlElement<Msg> {
    let description = match &task.edits {
        Some(e) => e.as_str(),
        None => task.description.as_str(),
    };
    li().set(classes([
        if task.completed { "completed" } else { "" },
        if task.edits.is_some() { "editing" } else { "" },
    ]))
    .push(
        div()
            .set_key(Some(task.id.to_string()))
            .set(class("view"))
            .push(
                input()
                    .set(class("toggle"))
                    .set(type_("checkbox"))
                    .set_if(task.completed, checked())
                    .set(on_click(Msg::ToggleCompleted(task.id))),
            )
            .push(label().push(description))
            .push(
                button()
                    .set(class("destroy"))
                    .set(on_click(Msg::DeleteTask(task.id))),
            ),
    )
}

fn view_controls(visibility: &Visibility, tasks: &[Task]) -> HtmlElement<Msg> {
    let tasks_completed = tasks.iter().filter(|t| t.completed).count();
    let tasks_left = tasks.len() - tasks_completed;
    let item_label = if tasks_left == 1 { "item" } else { "items" };
    footer()
        .set(class("footer"))
        .push(
            span()
                .set(class("todo-count"))
                .push(strong().push(tasks_left))
                .push(" ")
                .push(item_label)
                .push(" left"),
        )
        .push(
            ul().set(class("filters"))
                .push(view_visibility_swap(Visibility::All, &visibility))
                .push(view_visibility_swap(Visibility::Active, &visibility))
                .push(view_visibility_swap(Visibility::Completed, &visibility)),
        )
        .push(
            button()
                .set(class("clear-completed"))
                .set(on_click(Msg::ClearCompleted))
                .push("Clear completed (")
                .push(tasks_completed)
                .push(")"),
        )
}

fn view_visibility_swap(target: Visibility, actual: &Visibility) -> HtmlElement<Msg> {
    let (uri, label) = match target {
        Visibility::All => ("#/", "All"),
        Visibility::Active => ("#/active", "Active"),
        Visibility::Completed => ("#/completed", "Completed"),
    };
    let class_name = if &target == actual { "selected" } else { "" };
    li().set(on_click(Msg::ChangeVisibility(target)))
        .push(a().set(class(class_name)).set(href(uri)).push(label))
}

fn view_info_footer<T>() -> HtmlElement<T> {
    footer()
        .set(class("info"))
        .push(p().push("Double-click to edit a todo"))
        .push(
            p().push("Written by ").push(
                a().set(href("https://www.sagan.software"))
                    .push("sagan.software"),
            ),
        )
        .push(
            p().push("Part of ")
                .push(a().set(href("http://todomvc.com/")).push("TodoMVC")),
        )
}
