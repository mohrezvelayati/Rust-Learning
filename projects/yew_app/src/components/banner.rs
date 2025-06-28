use yew::prelude::*;

#[function_component(Banner)]
pub fn banner() -> Html {
    html! {
        <section class="bg-gray-50 py-20">
            <div class="max-w-screen-xl mx-auto px-4 flex flex-col-reverse lg:flex-row items-center justify-between gap-10">
                
                
                <div class="text-center lg:text-left max-w-xl">
                    <h1 class="text-4xl sm:text-5xl font-bold text-gray-900 leading-tight">
                        {"I'm Mohrez"}<br />
                        <span class="text-teal-600">{"Developer & Computer Engineering Student"}</span>
                    </h1>

                    <p class="mt-4 text-gray-600 text-lg">
                        {"I build web apps with Rust and Django, and I'm passionate about tech and entrepreneurship."}
                    </p>

                    <div class="mt-6 flex flex-wrap justify-center lg:justify-start gap-4">
                        <a href="#contact" class="px-6 py-3 bg-teal-600 text-white rounded-md text-sm hover:bg-teal-700 transition">
                            {"Contact Me"}
                        </a>
                        <a href="#projects" class="px-6 py-3 border border-teal-600 text-teal-600 rounded-md text-sm hover:bg-teal-50 transition">
                            {"See Projects"}
                        </a>
                    </div>
                </div>

                
                <div class="w-full lg:w-1/2">
                    <img src="https://placehold.co/400x300?text=Your+Photo" alt="Profile Banner" class="mx-auto rounded-lg shadow-md"/>
                </div>
            </div>
        </section>
    }
}
