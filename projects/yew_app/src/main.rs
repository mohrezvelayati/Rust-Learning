use yew::prelude::*;
mod components;

use crate::components::header::Header;
use crate::components::banner::Banner;

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
            <Banner />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}