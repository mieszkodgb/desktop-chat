use ollama_rs::{
    generation::completion::{
        request::GenerationRequest, GenerationContext, GenerationResponseStream
    },
    Ollama,
};

pub async fn llama_server(input: String) -> String{
    let ollama = Ollama::default();

    let mut context: Option<GenerationContext> = None;

    // let input = "Give me 10 programming languages and what they are good for".to_string();

    let mut request = GenerationRequest::new("deepseek-r1:1.5b".into(), input.to_string());
        if let Some(context) = context.clone() {
            request = request.context(context);
        }
    let response = ollama.generate(request).await;

    if let Ok(response) = response {
        println!("{}", response.response);
        return response.response;
    }
    return "Error".to_string();
}

pub async fn llama_stream(input: String) -> GenerationResponseStream{
    let ollama = Ollama::default();

    let mut context: Option<GenerationContext> = None;

    let mut request = GenerationRequest::new("deepseek-r1:1.5b".into(), input.to_string());
    if let Some(context) = context.clone() {
        request = request.context(context);
    }
    let stream: GenerationResponseStream = ollama.generate_stream(request).await.unwrap();
    return  stream;
}