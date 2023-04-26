use llm_chain::{executor, parameters, prompt, step::Step};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    let exec = executor!()?;
    // Create our step containing our prompt template
    // TODO: replace kubectl with {{cli-tool}}
    // TODO: replace {{text}} with {{user-request}}
    let step = Step::for_prompt(prompt!(
        "You are a specialist agent in the kubectl commandline tool.
        You are asked to solve an issue that the user is facing.
        You will respond with a brief paragraph explaining the solution.
        In the last line of your output you will provide the full command that the user shall execute.",
        "{{text}}"
    ));

    // TODO: figure out why the multi-parameter setup did not work
    // let params = parameters!("cli-tool" => "kubectl", "user-request" => "I need to list all the namespaces.");
    let params = parameters!("I need to list all the namespaces.");
    println!("{}", step.prompt());

    let res = step.run(&params, &exec).await?;
    println!("{}", res);

    Ok(())
}
