use automator::*;
// use reqwest::Client;
use itertools::izip;
use lazy_static::lazy_static;
use regex::Regex;
use std::process::{Command, Output};
use std::time::Duration;

lazy_static! {
    static ref NAMES: Regex = { Regex::new(r"name:\s<(.+)>\n").unwrap() };
    static ref DESCRIPTIONS: Regex = { Regex::new(r#"device\.description\s=\s"(.+)""#).unwrap() };
    static ref INDICIES: Regex = { Regex::new(r"index:\s([0-9]+)").unwrap() };
}

fn main() {
    let mut con = Connector::new();
    // con.add_connection(Interval::new(Duration::from_secs(5)), DebugAction {});
    con.add_connection(
        Interval::new(Duration::from_secs(1)),
        SwitchAudio::new(
            "Built-in Audio Digital Stereo (IEC958)",
            "Alienware Wireless Gaming Headset AW988 Analog Mono",
        ),
    );
    con.run();
}

struct SwitchAudio {
    first: &'static str,
    second: &'static str,
    is_first: bool,
}

impl SwitchAudio {
    pub fn new(first: &'static str, second: &'static str) -> Self {
        Self {
            first,
            second,
            is_first: true,
        }
    }

    pub fn get_pairs() -> Vec<(String, String, usize)> {
        let pacmd = Command::new("pacmd")
            .arg("list-sinks")
            .output()
            .expect("pacmd not found");

        let data = String::from_utf8(pacmd.stdout).expect("Couldn't parse pacmd output.");

        let name_matches = dbg!(NAMES
            .captures_iter(&data)
            .map(|s| s.iter().nth(1).unwrap().unwrap().as_str().to_string())
            .collect::<Vec<String>>());
        let desc_matches = DESCRIPTIONS
            .captures_iter(&data)
            .map(|s| s.iter().nth(1).unwrap().unwrap().as_str().to_string())
            .collect::<Vec<String>>();
        let idx_matches = INDICIES
            .captures_iter(&data)
            .filter_map(|s| {
                s.iter()
                    .nth(1)
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .ok()
            })
            .collect::<Vec<usize>>();

        dbg!(izip!(name_matches, desc_matches, idx_matches).collect())
    }

    pub fn inputs_to_move() -> Vec<usize> {
        let cmd = Command::new("pacmd")
            .arg("list-sink-inputs")
            .output()
            .expect("Couldn't call pacmd.");

        let data = String::from_utf8(cmd.stdout).expect("Couldn't parse pacmd output.");
        INDICIES
            .captures_iter(&data)
            .filter_map(|s| {
                s.iter()
                    .nth(1)
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .ok()
            })
            .collect()
    }
}

impl Action for SwitchAudio {
    fn act(&mut self) -> bool {
        println!("Changing");
        let dat = SwitchAudio::get_pairs();
        let mut cmd = Command::new("pacmd");
        let mut cmd = cmd.arg("set-default-sink");
        let mut snum = 0;

        for (name, desc, num) in dat {
            if (desc == self.first && self.is_first) || (desc == self.second && !self.is_first) {
                cmd = cmd.arg(name);
                snum = num;
            }
        }

        cmd.spawn().expect("Couldn't change audio device.");

        let inps = dbg!(SwitchAudio::inputs_to_move());
        for inp in inps {
            Command::new("pacmd")
                .arg("move-sink-input")
                .arg(format!("{}", inp))
                .arg(format!("{}", dbg!(snum)))
                .spawn()
                .expect("Couldn't move input to new output.");
        }
        self.is_first = !self.is_first;

        true
    }
}
