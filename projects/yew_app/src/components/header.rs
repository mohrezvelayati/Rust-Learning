use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-white sticky top-0 z-50">
            <div class="mx-auto flex w-11/12 items-center justify-between h-16 px-4 sm:px-6 lg:px-8 border-b border-slate-250">
                <a href="#" class="text-black font-bold text-xl tracking-tight">
                    {"MOHREZ DEV"}
                </a>

                <nav class="hidden md:flex space-x-6 text-sm">
                    <a href="https://github.com/mohrezvelayati" target="_blank" class="text-gray-600 hover:text-black transition">{"GitHub"}</a>
                </nav>
            </div>
        </header>
    }
}
