use serde::Deserialize;
use serde_yaml;
use std::fs;

#[derive(Debug, Deserialize)]
struct Service {
    name: String,
    deps: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Config {
    services: Vec<Service>,
}

fn main() {
    // Read the YAML file
    let yaml_str = fs::read_to_string("env-config.yml").expect("Failed to read env-config.yml");

    // Parse the YAML
    let config: Config = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");

    let mut not_installed: Vec<String> = config
        .services
        .iter()
        .map(|service| service.name.clone())
        .collect();

    let mut installing: Vec<String> = Vec::new();
    let mut installed: Vec<String> = Vec::new();

    let mut iteration = 0;

    while !not_installed.is_empty() || !installing.is_empty() {
        // Mark previous services as done
        let mut done = Vec::new();

        for name in &installing {
            done.push(name.clone());
        }

        println!("Iteration: {}", iteration);
        iteration += 1;

        // Move done services from installing to installed
        for name in &done {
            installing.retain(|n| n != name);
            installed.push(name.clone());
            println!("  - Installed: {}", name);
        }

        // Find services that are ready to be installed because their dependencies are satisfied
        let mut ready = Vec::new();

        for service_name in &not_installed {
            let service = config
                .services
                .iter()
                .find(|s| &s.name == service_name)
                .unwrap();

            let deps_satisfied = match &service.deps {
                None => true,
                Some(deps) => deps.iter().all(|dep| installed.contains(dep)),
            };

            if deps_satisfied {
                ready.push(service_name.clone());
            }
        }

        // Move ready services from not_installed to installing
        for name in &ready {
            not_installed.retain(|n| n != name);
            installing.push(name.clone());
        }
    }
}
