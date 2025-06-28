use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-white shadow sticky top-0 z-50">
            <div class="max-w-screen-xl mx-auto flex items-center justify-between h-16 px-4 sm:px-6 lg:px-8">
                <a href="#" class="text-teal-600 font-bold text-xl tracking-tight">
                    {"Mohrez.dev"}
                </a>

                <nav class="hidden md:flex space-x-6 text-sm">
                    <a href="#about" class="text-gray-600 hover:text-teal-600 transition">{"About"}</a>
                    <a href="#skills" class="text-gray-600 hover:text-teal-600 transition">{"Skills"}</a>
                    <a href="#projects" class="text-gray-600 hover:text-teal-600 transition">{"Projects"}</a>
                    <a href="#contact" class="text-gray-600 hover:text-teal-600 transition">{"Contact"}</a>
                    
                    <a
                        href="https://github.com/mohrezvelayati"
                        target="_blank"
                        class="text-gray-600 hover:text-teal-600 transition"
                    >
                        {"GitHub"}
                    </a>
                </nav>

                <div class="hidden md:block">
                    <a
                        href="/resume.pdf"
                        target="_blank"
                        class="px-4 py-2 text-sm bg-teal-600 text-white rounded hover:bg-teal-700 transition"
                    >
                        {"Download CV"}
                    </a>
                </div>
            </div>
        </header>
    }
}
