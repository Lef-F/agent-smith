use llm_chain::{executor, parameters, prompt, step::Step};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    let exec = executor!()?;
    // Create our step containing our prompt template
    let step = Step::for_prompt_template(prompt!(
        "You are a specialist agent in the {{cli_tool}} commandline tool.
        You are asked to solve an issue that the user is facing.
        Your response has to be only the following YAML template:

        ```yaml
        explanation: Your brief explanation goes here.
        command: {{cli_tool}} shell command solution goes here.
        ```

        Only replace the information as required.
        Do not include any text outside of the template.",
        "{{user_request}}"
    ));

    let params = parameters!(
        "cli_tool" => "kubectl",
        "user_request" => "I need to list all the namespaces."
    );
    println!("{}", step.format(&params).unwrap());

    let res = step.run(&params, &exec).await?;
    println!("{:?}", res);
    println!("{}", res);

    Ok(())
}
