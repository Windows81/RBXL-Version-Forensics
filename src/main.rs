use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::u32;
use ustr::Ustr;
mod traits;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    file_paths: Vec<String>,
}

fn analyse_dom(dom: rbx_dom_weak::WeakDom) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let mut used_classes = HashSet::new();

    let mut likely_version_min = traits::VERSION_MIN;
    let mut likely_version_max = traits::VERSION_MAX;
    for descendant in dom.descendants() {
        let class = descendant.class.as_str();
        if used_classes.contains(class) {
            continue;
        }
        let Some(h) = traits::TRAITS.get(class) else {
            continue;
        };

        used_classes.insert(class);
        for (&name, &(v_min, v_max)) in h.entries() {
            match descendant.properties.contains_key(&Ustr::from(name)) {
                true => {
                    if v_min > likely_version_min {
                        likely_version_min = v_min;
                    }
                    if v_max < likely_version_max {
                        likely_version_max = v_max;
                    }
                }
                false => {
                    if v_max >= likely_version_max && v_min < likely_version_max {
                        likely_version_max = v_min;
                    }
                }
            }
        }
    }

    return Ok((likely_version_min, likely_version_max - 1));
}

fn analyse_file(file_name: String) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open(file_name)?);
    let dom = rbx_binary::from_reader(input)?;
    return analyse_dom(dom);
}

fn main() {
    for path in Args::parse().file_paths {
        let (v_min, v_max) = analyse_file(path.clone()).unwrap();
        println!("{},{},{}", path, v_min, v_max);
    }
}
