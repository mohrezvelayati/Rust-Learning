use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <aside class="bg-white w-full flex flex-row justify-between px-6 py-10">
            <div class="flex flex-col items-center text-center">

                
                <img
                    src="https://placehold.co/150x150"
                    alt="Profile"
                    class="rounded-full mb-4"
                />

                
                <h2 class="text-xl font-semibold">{"Mike Davids"}</h2>
                <p class="text-gray-600 text-sm mb-4">{"User interface designer with a wealth of experience..."}</p>

                
                <div class="text-sm w-full text-left space-y-3">
                    <div class="flex justify-between border-t pt-2">
                        <span class="font-semibold">{"Profession"}</span>
                        <span>{"UI Designer"}</span>
                    </div>
                    <div class="flex justify-between border-t pt-2">
                        <span class="font-semibold">{"Date of Birth"}</span>
                        <span>{"11 November 1985"}</span>
                    </div>
                    <div class="flex justify-between border-t pt-2">
                        <span class="font-semibold">{"Education"}</span>
                        <span>{"Higher Education"}</span>
                    </div>
                </div>

                
                <button class="mt-6 px-4 py-2 border text-sm border-black hover:bg-black hover:text-white transition">
                    {"Send Message"}
                </button>
            </div>
        </aside>
    }
}
