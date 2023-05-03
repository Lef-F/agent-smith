use llm_chain::{executor, parameters, prompt, step::Step};

#[tokio::main(flavor = "current_thread")]
pub async fn explainer(interests: &str, email: &str) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: check if OPENAI_API_KEY key is set
    // std::env::var("OPENAI_API_KEY").is_ok();

    // Create a new ChatGPT executor
    let exec = executor!()?;
    // Create our step containing our prompt template
    let step = Step::for_prompt_template(prompt!(
        "You are an email summariser. 
        The user will provide you with the body of an email and they expect you to
        answer with a bullet list of at most 5 actions that the user might be interested in considering.
        The user is looking for {{interests}}.
        Do not provide any bullet points that are not relevant to what the user is looking for.
        Prefer to respond with fewer than 5 bullet points if necessary.",
        "{{email}}"
    ));

    let params = parameters!(
        "interests" => interests.trim(),
        "email" => email.trim()
    );
    println!("{}", step.format(&params).unwrap());

    let res = step.run(&params, &exec).await?;
    println!("{:?}", res);
    println!("{}", res);

    Ok(())
}
