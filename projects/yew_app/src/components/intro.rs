use yew::prelude::*;

#[function_component(Intro)]
pub fn intro() -> Html {
    html! {
        <main class="w-full flex flex-row justify-center mt-10">
            <div class="w-7/12">
            
            <section>
                <h1 class="text-5xl font-bold uppercase leading-tight">
                    {"Software Engineer"}<br/>
                    <span class="font-light">{"And "}</span>
                    <span class="font-bold italic">{"Back-End Developer."}</span>
                </h1>
            </section>
            </div>

            <section>
                <div class="p-20 w-4/12 flex flex-row justify-between items-end gap-5">
                    <a
                        class="w-10 h-10 aspect-square flex items-center justify-center
                        border border-gray-300 rounded-full
                        hover:bg-gray-800 hover:text-white transition"
                        href="https://instagram.com/mohrez.velayati"
                        target="_blank"
                    >
                        <i class="fa-brands fa-instagram text-lg"></i>
                    </a>
                    <a
                        class="w-10 h-10 aspect-square flex items-center justify-center
                        border border-gray-300 rounded-full
                        hover:bg-gray-800 hover:text-white transition"
                        href="https://x.com/MohrezVelayati"
                        target="_blank"
                    >
                        <i class="fa-brands fa-x-twitter text-lg"></i>
                    </a>
                    <a
                        class="w-10 h-10 aspect-square flex items-center justify-center
                        border border-gray-300 rounded-full
                        hover:bg-gray-800 hover:text-white transition"
                        href="https://www.linkedin.com/in/mohammadrezavelayati/"
                        target="_blank"
                    >
                        <i class="fa-brands fa-linkedin-in text-lg"></i>
                    </a>
                    <a
                        class="w-10 h-10 aspect-square flex items-center justify-center
                        border border-gray-300 rounded-full
                        hover:bg-gray-800 hover:text-white transition"
                        href="https://t.me/mohrezvelayati"
                        target="_blank"
                    >
                        <i class="fa-brands fa-telegram text-lg"></i>
                    </a>
                </div>
            </section>


        </main>
    }
}