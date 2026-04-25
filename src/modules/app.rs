use rand::Rng;
use ratatui::style::Color;
use std::time::Instant;

use crate::modules::character::{
    Character, ContainerState, EXITED_THOUGHTS, PAUSED_THOUGHTS, RUNNING_THOUGHTS,
    THIRSTY_PLANT_THOUGHTS, WATERING_THOUGHTS,
};
use crate::modules::docker::{fetch_containers, stop_container, DockerContainer};
use crate::modules::office::{waypoints, PLANTS};

pub const PLANT_MAX_THIRST: u32 = 600; // ~48s at 80ms/tick
const PLANT_WATER_DIST: f32 = 8.0;

pub struct App {
    pub containers: Vec<DockerContainer>,
    pub characters: Vec<Character>,
    pub tick: usize,
    pub selected: usize,
    pub last_refresh: Instant,
    pub office_width: u16,
    pub office_height: u16,
    pub plant_thirsts: Vec<u32>,
}

impl App {
    pub fn new() -> Self {
        let containers = fetch_containers();
        let characters: Vec<Character> = containers
            .iter()
            .enumerate()
            .map(|(i, c)| Character::from_container(c, i))
            .collect();
        Self {
            containers,
            characters,
            tick: 0,
            selected: 0,
            last_refresh: Instant::now(),
            office_width: 80,
            office_height: 30,
            plant_thirsts: vec![0; PLANTS.len()],
        }
    }

    pub fn refresh(&mut self) {
        let new_containers = fetch_containers();
        let mut new_chars: Vec<Character> = Vec::new();
        for (i, nc) in new_containers.iter().enumerate() {
            if let Some(existing) = self
                .characters
                .iter()
                .find(|ch| ch.name == nc.names || nc.names.starts_with(&ch.name))
            {
                let mut ch = existing.clone();
                ch.state = if nc.state.contains("running") {
                    ContainerState::Running
                } else if nc.state.contains("paused") {
                    ContainerState::Paused
                } else {
                    ContainerState::Exited
                };
                ch.color = match ch.state {
                    ContainerState::Running => Color::Green,
                    ContainerState::Paused => Color::Yellow,
                    ContainerState::Exited => Color::Red,
                };
                new_chars.push(ch);
            } else {
                new_chars.push(Character::from_container(nc, i));
            }
        }
        self.containers = new_containers;
        self.characters = new_chars;
        if !self.containers.is_empty() && self.selected >= self.containers.len() {
            self.selected = self.containers.len() - 1;
        }
        self.last_refresh = Instant::now();
    }

    pub fn update(&mut self) {
        self.tick += 1;
        let mut rng = rand::thread_rng();
        let wps = waypoints();

        // --- Plant game mechanic ---
        let has_running = self.characters.iter().any(|c| c.state == ContainerState::Running);
        if has_running {
            let thirst_ratio_max = self.plant_thirsts.iter()
                .map(|&t| t as f32 / PLANT_MAX_THIRST as f32)
                .fold(0.0_f32, f32::max);

            for thirst in self.plant_thirsts.iter_mut() {
                *thirst += 1;
            }

            if thirst_ratio_max > 0.6 {
                for ch in self.characters.iter_mut() {
                    if ch.state != ContainerState::Running {
                        continue;
                    }
                    if rng.gen_range(0..100) == 0 {
                        let thought =
                            THIRSTY_PLANT_THOUGHTS[rng.gen_range(0..THIRSTY_PLANT_THOUGHTS.len())];
                        ch.thought = Some((thought.to_string(), 50));
                    }
                }
            }

            // Reset dead plants and count how many died this tick
            let dead_count = self.plant_thirsts.iter().filter(|&&t| t >= PLANT_MAX_THIRST).count();
            for thirst in self.plant_thirsts.iter_mut() {
                if *thirst >= PLANT_MAX_THIRST {
                    *thirst = 0;
                }
            }
            for _ in 0..dead_count {
                let running_indices: Vec<usize> = self
                    .characters
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.state == ContainerState::Running)
                    .map(|(i, _)| i)
                    .collect();
                if !running_indices.is_empty() {
                    let idx = running_indices[rng.gen_range(0..running_indices.len())];
                    let container_id = self.containers.get(idx).map(|c| c.id.clone());
                    if let Some(id) = container_id {
                        stop_container(&id);
                    }
                    if let Some(ch) = self.characters.get_mut(idx) {
                        ch.state = ContainerState::Exited;
                        ch.color = Color::Red;
                        ch.thought = Some(("killed by plant 🪴".to_string(), 100));
                    }
                }
            }
        }
        // --- End plant mechanic ---

        let selected = self.selected;
        for (i, ch) in self.characters.iter_mut().enumerate() {
            match ch.state {
                ContainerState::Running => {
                    // Player-controlled character: skip auto-wander
                    if i != selected {
                        let (tx, ty) = wps[ch.target_wp];
                        let dx = tx - ch.x;
                        let dy = ty - ch.y;
                        let dist = (dx * dx + dy * dy).sqrt();

                        if dist < 1.0 {
                            ch.target_wp = rng.gen_range(0..wps.len());
                            if rng.gen_range(0..4) == 0 {
                                let thought =
                                    RUNNING_THOUGHTS[rng.gen_range(0..RUNNING_THOUGHTS.len())];
                                ch.thought = Some((thought.to_string(), 30));
                            }
                        } else {
                            let speed = 0.4;
                            ch.x += (dx / dist) * speed;
                            ch.y += (dy / dist) * speed;
                        }
                    }
                    ch.frame = self.tick / 3;
                }
                ContainerState::Paused => {
                    if rng.gen_range(0..80) == 0 {
                        let thought = PAUSED_THOUGHTS[rng.gen_range(0..PAUSED_THOUGHTS.len())];
                        ch.thought = Some((thought.to_string(), 40));
                    }
                }
                ContainerState::Exited => {
                    if rng.gen_range(0..120) == 0 {
                        let thought = EXITED_THOUGHTS[rng.gen_range(0..EXITED_THOUGHTS.len())];
                        ch.thought = Some((thought.to_string(), 50));
                    }
                }
            }

            if let Some((_, ref mut ttl)) = ch.thought {
                if *ttl == 0 {
                    ch.thought = None;
                } else {
                    *ttl -= 1;
                }
            }
        }
    }

    pub fn move_selected(&mut self, dx: f32, dy: f32) {
        if let Some(ch) = self.characters.get_mut(self.selected) {
            if ch.state == ContainerState::Running {
                ch.x = (ch.x + dx).clamp(2.0, 76.0);
                ch.y = (ch.y + dy).clamp(2.0, 26.0);
                ch.frame = self.tick / 3;
            }
        }
    }

    pub fn try_water_plant(&mut self) {
        let selected_pos = self.characters.get(self.selected).and_then(|ch| {
            if ch.state == ContainerState::Running {
                Some((ch.x, ch.y))
            } else {
                None
            }
        });

        if let Some((cx, cy)) = selected_pos {
            let mut watered = false;
            for (i, plant) in PLANTS.iter().enumerate() {
                let dx = cx - plant.game_x;
                let dy = cy - plant.game_y;
                if (dx * dx + dy * dy).sqrt() < PLANT_WATER_DIST {
                    self.plant_thirsts[i] = 0;
                    watered = true;
                }
            }
            if watered {
                let thought_idx = self.tick % WATERING_THOUGHTS.len();
                self.characters[self.selected].thought =
                    Some((WATERING_THOUGHTS[thought_idx].to_string(), 40));
            }
        }
    }

    pub fn is_selected_near_plant(&self) -> bool {
        self.characters.get(self.selected).map(|ch| {
            if ch.state != ContainerState::Running {
                return false;
            }
            PLANTS.iter().any(|plant| {
                let dx = ch.x - plant.game_x;
                let dy = ch.y - plant.game_y;
                (dx * dx + dy * dy).sqrt() < PLANT_WATER_DIST
            })
        }).unwrap_or(false)
    }
}
