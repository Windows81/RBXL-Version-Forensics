use clap::Parser;
use constants::ERAS;
use deserializer::ClassMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::u32;
mod constants;
mod deserializer;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    file_paths: Option<Vec<String>>,
}

fn analyse_class_map(map: ClassMap) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let mut likely_version_min = constants::VERSION_MIN;
    let mut likely_version_max = constants::VERSION_MAX;
    for (class, props) in map.class_to_props.iter() {
        let Some(h) = constants::TRAITS.get(class) else {
            continue;
        };

        for (&name, &v_min) in h.entries() {
            let check = match name {
                "" => true,
                _ => props.contains(name),
            };
            match check {
                true => {
                    if v_min > likely_version_min {
                        likely_version_min = v_min;
                    }
                }
                false => {
                    if v_min < likely_version_max {
                        likely_version_max = v_min;
                    }
                }
            }
        }
    }

    match likely_version_min >= likely_version_max {
        true => Err(format!(
            "Err: this program needs help figuring this data out [{}, {}].",
            likely_version_min,
            likely_version_max - 1
        )
        .into()),
        false => Ok((likely_version_min, likely_version_max - 1)),
    }
}

fn analyse_file(file_name: String) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open(file_name)?);
    let dom = deserializer::deserialize(input)?;
    return analyse_class_map(dom);
}

fn get_version_era(version: u32) -> String {
    return match ERAS.iter().find(|(_, min)| version > *min) {
        Some(&(era, min)) => era,
        None => "N/A",
    }
    .into();
}

fn main() {
    let paths = match Args::parse().file_paths {
        Some(x) => x,
        None => io::stdin().lock().lines().into_iter().flatten().collect(),
    };
    for path in paths {
        match analyse_file(path.clone()) {
            Ok((v_min, v_max)) => {
                println!(
                    "{},{},{},{},{}",
                    path,
                    v_min,
                    v_max,
                    get_version_era(v_min),
                    get_version_era(v_max),
                );
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
