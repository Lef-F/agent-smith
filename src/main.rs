use std::io;

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

        Only replace the information in the YAML template above
        and only return that YAML template as your answer.",
        "{{user_request}}"
    ));

    let mut cli_tool = String::new();
    println!("What CLI command do you need help with?");
    io::stdin()
        .read_line(&mut cli_tool)
        .expect("error: unable to read user input");

    let mut user_request = String::new();
    println!("Describe your issue:");
    io::stdin()
        .read_line(&mut user_request)
        .expect("error: unable to read user input");

    let params = parameters!(
        "cli_tool" => cli_tool.trim(),
        "user_request" => user_request.trim()
    );
    println!("{}", step.format(&params).unwrap());

    let res = step.run(&params, &exec).await?;
    println!("{:?}", res);
    println!("{}", res);

    Ok(())
}
