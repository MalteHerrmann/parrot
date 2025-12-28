use parrot::llm::get_available_models;

fn main() {
    let available_models = get_available_models().expect("failed to get models");
    available_models.iter().for_each(|m| println!("{}", m))
}
