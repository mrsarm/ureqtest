use std::time::Duration;
use ureq::Agent;

fn main() {
    let url = "https://google.com";
    println!("GET {url} ...");
    let agent: Agent = Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(5)))
        .timeout_global(Option::from(Duration::from_secs(10)))
        .build()
        .into();

    agent.get(url).call().unwrap();
}
