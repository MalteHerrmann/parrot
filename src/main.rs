use parrot::llm::get_available_models;

fn main() {
    let available_models = get_available_models().expect("failed to get models");

    available_models.iter().for_each(|m| {
        let out = m
            .prompt("say hello to my friends")
            .expect("failed to prompt");

        println!("{} - {}", m.get_name(), out);
    })
}
