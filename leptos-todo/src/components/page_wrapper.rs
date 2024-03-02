use leptos::*;

#[component]
pub fn PageWrapper(children: Children) -> impl IntoView {
    view! {
        <div class="h-screen">
            <div class="pt-20 mb-10 mx-auto px-5 w-full lg:px-0 lg:max-w-[90ch]">
                {children()}
            </div>
        </div>
    }
}
