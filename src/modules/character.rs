use ratatui::style::Color;

use crate::modules::docker::DockerContainer;
use crate::modules::office::waypoints;

#[derive(Clone, PartialEq)]
pub enum ContainerState {
    Running,
    Paused,
    Exited,
}

#[derive(Clone)]
pub struct Character {
    pub x: f32,
    pub y: f32,
    pub target_wp: usize,
    pub color: Color,
    pub frame: usize,
    pub name: String,
    pub state: ContainerState,
    pub thought: Option<(String, usize)>,
}

pub const RUNNING_THOUGHTS: &[&str] = &[
    "deploying...",
    "npm install",
    "shipping it!",
    "LGTM!",
    "☕ coffee time",
    "fixing bugs",
    "in the zone",
    "git push!",
    "compiling...",
    "reviewing PR",
    "on a call...",
    "standup time",
    "refactoring",
    "writing tests",
];

pub const PAUSED_THOUGHTS: &[&str] = &[
    "zzZ...",
    "💤 snoring",
    "5 more mins",
    "zzZzz...",
    "*yawn*",
];

pub const EXITED_THOUGHTS: &[&str] = &[
    "segfault :(",
    "OOM killed",
    "exit code 1",
    "RIP",
    "need restart",
    "panic!()",
];

pub const WATERING_THOUGHTS: &[&str] = &[
    "watering 🌿",
    "💧 here ya go",
    "plant saved!",
    "hydrating!",
];

pub const THIRSTY_PLANT_THOUGHTS: &[&str] = &[
    "🪴 water me!",
    "save the plant!",
    "it's thirsty!",
    "water! now!",
];

impl Character {
    pub fn from_container(c: &DockerContainer, index: usize) -> Self {
        let wps = waypoints();
        let wp_idx = index % wps.len();
        let (sx, sy) = wps[wp_idx];
        let state = if c.state.contains("running") {
            ContainerState::Running
        } else if c.state.contains("paused") {
            ContainerState::Paused
        } else {
            ContainerState::Exited
        };
        let color = match state {
            ContainerState::Running => Color::Green,
            ContainerState::Paused => Color::Yellow,
            ContainerState::Exited => Color::Red,
        };
        let name = if c.names.len() > 10 {
            c.names[..10].to_string()
        } else {
            c.names.clone()
        };
        Character {
            x: sx,
            y: sy,
            target_wp: wp_idx,
            color,
            frame: 0,
            name,
            state,
            thought: None,
        }
    }

    pub fn sprite_lines(&self) -> Vec<(&str, &str)> {
        match self.state {
            ContainerState::Running => {
                if self.frame % 2 == 0 {
                    vec![
                        (" o ", " o "),
                        ("/|\\", "/|\\"),
                        ("/ \\", " |\\"),
                    ]
                } else {
                    vec![
                        (" o ", " o "),
                        ("/|\\", "/|\\"),
                        ("/| ", "/ \\"),
                    ]
                }
            }
            ContainerState::Paused => {
                vec![
                    (" _ ", " _ "),
                    ("(-.)", "(-.)")  ,
                    ("/|\\", "/|\\"),
                ]
            }
            ContainerState::Exited => {
                vec![
                    ("    ", "    "),
                    ("____", "____"),
                    ("x_x)", "x_x)"),
                ]
            }
        }
    }
}
