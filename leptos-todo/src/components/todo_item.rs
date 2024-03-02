use leptos::*;

#[derive(Clone, Debug)]
pub struct TodoItem {
    pub id: u32,
    pub task: String,
    pub status: bool,
}

#[component]
pub fn TodoItem<F>(todo_item: TodoItem, delete_callback: F) -> impl IntoView
where
    F: Fn(u32) + 'static,
{
    let (status, set_status) = create_signal(todo_item.status);
    let on_click = move |_| set_status.update(|status| *status = !*status);
    let task_title_style =
        move || format!("text-md {}", if status() { "line-through" } else { "" });
    let complete_button_style = move || {
        format!(
            "hover:cursor-pointer {}",
            if !status() {
                "opacity-100"
            } else {
                "opacity-50"
            }
        )
    };
    view! {
        <div class="flex justify-between items-center">
            <span class={task_title_style}>{todo_item.task}</span>
            <div class="flex justify-between w-fit sm:w-1/3">
                <button
                    on:click=on_click
                    class={complete_button_style}
                >
                    {move || if !status() { "Complete" } else { "Undo" }}
                </button>
                <button on:click={move |_| delete_callback(todo_item.id)} class="hover:cusor-pointer ml-4 sm:ml-0">"Delete"</button>
            </div>
        </div>
    }
}
