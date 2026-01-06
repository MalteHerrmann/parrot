use parrot::llm::get_available_models;

#[tokio::main]
async fn main() {
    let available_models = get_available_models().expect("failed to get models");

    for model in available_models.into_iter() {
        let out = model
            .prompt("reply with a check mark if this request is successful")
            .await
            .expect("failed to prompt");

        println!("{} - {}", model.get_name(), out.trim());
    }
}
