

#[derive(Default)]
struct Task {
    description: String,
    completed: bool,
    edits: Option<String>,
    id: usize,
}

enum Msg {
    Focus(String),
    Edit(String),
    Cancel,
    Commit,
    Completed(bool),
    Delete,
}
