use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize, Clone)]
pub struct DockerContainer {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Names")]
    pub names: String,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Ports")]
    pub ports: String,
    #[serde(rename = "State")]
    pub state: String,
}

pub fn fetch_containers() -> Vec<DockerContainer> {
    let output = Command::new("docker")
        .args(["ps", "-a", "--format", "{{json .}}"])
        .output();
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout
                .lines()
                .filter_map(|l| serde_json::from_str::<DockerContainer>(l).ok())
                .collect()
        }
        Err(_) => vec![],
    }
}

pub fn stop_container(id: &str) {
    let _ = Command::new("docker")
        .args(["stop", id])
        .output();
}
