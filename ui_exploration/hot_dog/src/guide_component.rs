#[derive(Props, PartialEq, Clone)]
struct DogAppProps {
    breed: String,
}

fn DogApp(props: DogAppProps) -> Element {
    todo!()
}

#[component]
fn DogApp(breed: String) -> Element {
    todo!()
}

#[component]
fn DogApp(breed: String) -> Element {
    tracing::info!("Rendered with breed: {breed}");

    todo!()
}

#[component]
fn DogApp(breed: String) -> Element {
    rsx! { "Breed: {breed}" }
}

#[component]
fn App() -> Element {
    rsx! {
        Header {}
        DogApp { breed: "corgi" }
        Footer {}
    }
}