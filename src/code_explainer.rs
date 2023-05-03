use llm_chain::{executor, parameters, prompt, step::Step};

#[tokio::main(flavor = "current_thread")]
pub async fn explainer(lang: &str, user_request: &str) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: check if OPENAI_API_KEY key is set
    // std::env::var("OPENAI_API_KEY").is_ok();

    // Create a new ChatGPT executor
    let exec = executor!()?;
    // Create our step containing our prompt template
    let step = Step::for_prompt_template(prompt!(
        "You are an expert in writing code documentation in the {{lang}} programming language.
        The user will provide you with a piece of code.
        Your task is to describe each function and explain how it should be used.
        You will only answer by using and expanding the following YAML template.
        Populate each `abstraction` (e.g. classes with functions)
        with the `functions` attached to it and provide a short `description` and a short `usage` case.
        You will only answer by using and expanding the following YAML template:
        ```yaml
        abstraction-name-1:
            function-name-1:
                - description: some description text
                - usage: describe a potential use-case
                - inputs:
                    input-1: describe the usage of the input
                    input-2: describe the usage of the input
            function-name-2:
                - description: some description text
                - usage: describe a potential use-case
                    input-1: describe the usage of the input
                    input-2: describe the usage of the input
        abstraction-name-2:
            function-name-1:
                - description: some description text
                - usage: describe a potential use-case
                    input-1: describe the usage of the input
                    input-2: describe the usage of the input
        ```",
        "{{user_request}}"
    ));

    let params = parameters!(
        "lang" => lang.trim(),
        "user_request" => user_request.trim()
    );
    println!("{}", step.format(&params).unwrap());

    let res = step.run(&params, &exec).await?;
    println!("{:?}", res);
    println!("{}", res);

    Ok(())
}
