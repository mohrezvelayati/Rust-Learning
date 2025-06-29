use yew::prelude::*;

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <main class="flex-1 p-6 space-y-10">


            
            <section>
                <h2 class="text-xl font-bold border-b pb-1 mb-4">{"Experience"}</h2>
                <div class="space-y-4 text-sm">
                    <div>
                        <h3 class="font-semibold">{"Horizon Value Expert — UI Director"}</h3>
                        <span class="text-xs text-gray-500">{"2021 – Present"}</span>
                        <p>{"Creating wireframe projects, improving customer relations and recruitment campaigns."}</p>
                    </div>
                    <div>
                        <h3 class="font-semibold">{"Puente – Visual Concept — Senior Product Designer"}</h3>
                        <span class="text-xs text-gray-500">{"2016 – 2021"}</span>
                        <p>{"Creating products for events with a remote team."}</p>
                    </div>
                </div>
            </section>

            
            <section>
                <h2 class="text-xl font-bold border-b pb-1 mb-4">{"Education"}</h2>
                <ul class="space-y-4 text-sm">
                    <li>
                        <h3 class="font-semibold">{"Glory Charter High School"}</h3>
                        <span class="text-xs text-gray-500">{"2014 – 2017"}</span>
                        <p>{"Interface design studies completed with honors."}</p>
                    </li>
                    <li>
                        <h3 class="font-semibold">{"Guli CE Academy"}</h3>
                        <span class="text-xs text-gray-500">{"2012 – 2014"}</span>
                        <p>{"Marketing and promotion products on the internet."}</p>
                    </li>
                    <li>
                        <h3 class="font-semibold">{"Ambers Community College"}</h3>
                        <span class="text-xs text-gray-500">{"2009 – 2012"}</span>
                        <p>{"Implementing new technologies on computer science."}</p>
                    </li>
                </ul>
            </section>

            
            <section>
                <h2 class="text-xl font-bold border-b pb-1 mb-4">{"Awards"}</h2>
                <ul class="space-y-4 text-sm">
                    <li>
                        <div class="flex justify-between">
                            <span>{"Interface Designs – VEROO | Website"}</span>
                            <span class="text-xs">{"17 May 2022"}</span>
                        </div>
                    </li>
                    <li>
                        <div class="flex justify-between">
                            <span>{"Graphics – FIRCON GROUP"}</span>
                            <span class="text-xs">{"5 Jan 2020"}</span>
                        </div>
                    </li>
                </ul>
            </section>


        
            <section>
                <h2 class="text-xl font-bold border-b pb-1 mb-4">{"Skills"}</h2>
                <ul class="space-y-4 text-sm">
                    <li>
                        <span class="block font-medium">{"Microsoft Office"}</span>
                        <div class="w-full bg-gray-200 h-2 rounded">
                            <div class="bg-black h-2 rounded" style="width: 50%"></div>
                        </div>
                    </li>
                    <li>
                        <span class="block font-medium">{"STUDIO Editor"}</span>
                        <div class="w-full bg-gray-200 h-2 rounded">
                            <div class="bg-black h-2 rounded" style="width: 80%"></div>
                        </div>
                    </li>
                </ul>
            </section>

            
            <section>
                <h2 class="text-xl font-bold border-b pb-1 mb-4">{"Hobby"}</h2>
                <p class="text-sm mb-2">
                    {"I am a person who likes challenges and undertakes the most advanced projects for learning and commitment."}
                </p>
                <div class="flex flex-wrap gap-2 text-xs">
                    <span class="border px-3 py-1 rounded-full">{"Football"}</span>
                    <span class="border px-3 py-1 rounded-full">{"Music"}</span>
                    <span class="border px-3 py-1 rounded-full">{"Development"}</span>
                    <span class="border px-3 py-1 rounded-full">{"Technology"}</span>
                    <span class="border px-3 py-1 rounded-full">{"Investments"}</span>
                </div>
            </section>


        </main>
    }
}
