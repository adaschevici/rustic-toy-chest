use crate::components::page_wrapper::PageWrapper;
use crate::components::todo_item::TodoItem;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    //    let todo_items: Vec<TodoItem> = vec![
    //        TodoItem {
    //            id: 0,
    //            task: String::from("Take out the trash"),
    //            status: false,
    //        },
    //        TodoItem {
    //            id: 1,
    //            task: String::from("Make the bed"),
    //            status: false,
    //        },
    //        TodoItem {
    //            id: 2,
    //            task: String::from("Mow the lawn"),
    //            status: true,
    //        },
    //        TodoItem {
    //            id: 3,
    //            task: String::from("Wash the dishes"),
    //            status: false,
    //        },
    //    ];
    let (todo_items, set_todo_items) = create_signal(Vec::<TodoItem>::new());
    let delete_todo_item = move |todo_id: u32| {
        set_todo_items.update(move |todo_items| {
            todo_items.retain(|todo_item| todo_item.id != todo_id);
        });
    };
    let todo_task_input_ref: NodeRef<Input> = create_node_ref();

    // 👇 New!
    // Helpers
    /// Helper function to grab the largest ID.
    /// Used to know what the next todo item ID should be
    let last_todo_id = move || todo_items().iter().map(|todo_item| todo_item.id).max();

    // Handlers
    // 👇 New!
    let on_submit = move |ev: SubmitEvent| {
        // Prevent the page from refreshing
        ev.prevent_default();

        // Clone the vector to mutate it
        let mut new_todo_items = todo_items();

        // Get the next node ID
        let todo_id = last_todo_id().unwrap_or_default() + 1;

        // Create a new TodoItem and push it to our new todo_items vector
        new_todo_items.push(TodoItem {
            id: todo_id,
            task: todo_task_input_ref().expect("<input> to exist").value(),
            status: false,
        });

        // Set our todo_items signal to have the new todo_items vector
        set_todo_items.set(new_todo_items);
    };
    view! {
        <PageWrapper>
            <div id="add-task" class="flex flex-col rounded mb-20 text-black">
                <h2 class="text-2xl font-medium mb-4">"Add Task"</h2>
                <form class="w-full flex flex-col" on:submit={on_submit}>
                    <div class="flex items-center justify-between">
                        <input
                            class="w-2/3 px-2 py-1 border-b-2 border-black focus:outline-none"
                            type="text"
                            placeholder="Add a new task"
                            node_ref={todo_task_input_ref}
                        />
                        <input class="hover:cursor-pointer" type="submit" value="Submit" />
                    </div>
                </form>
            </div>
            <div id="todo_items">
                <h2 class="text-2xl font-medium mb-4">"Tasks"</h2>
                <For
                    each={todo_items}
                    key=|task| task.id
                    children=move |task : TodoItem| {
                        view! {
                            <TodoItem todo_item={task} delete_callback={delete_todo_item}/>
                        }
                    }
                />
            </div>
        </PageWrapper>
    }
}
