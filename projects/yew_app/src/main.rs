mod components;

use yew::prelude::*;

use crate::components::header::Header;
// use crate::components::banner::Banner;
use crate::components::sidebar::Sidebar;
use crate::components::main_content::MainContent;
use crate::components::intro::Intro;

// #[function_component]
// fn App() -> Html {
//     let counter = use_state(|| 0);
//     let onclick = {
//         let counter = counter.clone();
//         move |_| {
//             let value = *counter + 1;
//             counter.set(value);
//         }
//     };

//     html! {
//         <div>
//             <button {onclick}>{ "+1" }</button>
//             <p>{ *counter }</p>
//         </div>
//     }
// }


// #[function_component(HelloWorld)]
// fn hello_world() -> Html {
//     html! {
//         <h1 class="text-3xl font-bold underline">
//         {"Hello World!"}
//         </h1>
//     }
// }




#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Header />
            // <Banner />
            <Intro />
            <Sidebar />
            <MainContent />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}