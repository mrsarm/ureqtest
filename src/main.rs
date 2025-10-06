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

    let res = agent.get(url).call().unwrap();
    println!(
        "Response: {:?} {} {}",
        res.version(),
        res.status().as_u16(),
        res.status().canonical_reason().unwrap_or("")
    );
    let body = res.into_body().read_to_string().unwrap();
    println!("Body:\n{} ...", &body[0..40]);
}
