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
        <>
            <h1 class={classes!("text-4xl", "bg-red-400")}>{ "SRP Sounds" }</h1>
            <button {onclick}>{ "Generate new sound" }</button>
            <audio controls=true src={format!("sounds/{}.wav", *audio_file)} format="audio/wav"></audio>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
