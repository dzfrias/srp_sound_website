use rand::seq::SliceRandom;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let audio_file = use_state(|| {
        let mut notes = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let mut rng = rand::thread_rng();
        notes.shuffle(&mut rng);
        notes.into_iter().take(4).collect::<String>()
    });
    let onclick = {
        let audio_file = audio_file.clone();
        Callback::from(move |_| {
            let mut notes = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
            let mut rng = rand::thread_rng();
            notes.shuffle(&mut rng);
            audio_file.set(notes.into_iter().take(4).collect::<String>())
        })
    };
    html! {
        // TODO: Keybinds
        <div class={classes!("m-auto", "2xl:w-3/4", "h-screen", "selection:bg-slate-400", "font-['Nunito']")}>
            <h1 class={classes!("text-6xl", "text-sky-300", "italic", "text-center", "font-extrabold", "text-transparent", "text-7xl", "bg-clip-text", "bg-gradient-to-r", "from-sky-400", "to-green-300", "mb-10")}>{ "SRP Sounds" }</h1>
            <div class={classes!("md:w-1/2", "m-auto", "p-2", "rounded", "flex", "flex-col", "items-center", "border", "border-white", "shadow-md", "shadow-white", "bg-gradient-to-b", "from-blue-300", "to-green-200")}>
                <p class={classes!("text-center", "pb-16", "text-slate-800", "mt-5", "text-lg")}>{ "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat." }</p>
                <button {onclick} class={classes!("text-sky-800", "bg-green-300", "p-2", "rounded", "mb-2", "shadow-sm", "hover:shadow-white", "hover:bg-green-200", "border-black", "border", "active:bg-green-400")}>{ "New Sound" }</button>
                <audio controls=true src={format!("sounds/{}.wav", *audio_file)} format="audio/wav" class={classes!("bg-teal-600", "mb-3")}></audio>
                <p class={classes!("mb-4", "text-blue-900")}>{ format!("Playing: {}", audio_file.to_uppercase()) }</p>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
