use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize)]
struct Service {
    name: String,
    deps: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Config {
    services: Vec<Service>,
}

fn brute_force() {
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

fn progressive_lookup() {
    // Read the YAML file
    let yaml_str = fs::read_to_string("env-config.yml").expect("Failed to read env-config.yml");

    // Parse the YAML
    let config: Config = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");

    // Create a map over the services and their dependencies
    let mut dependency_map: HashMap<String, Vec<String>> = HashMap::new();

    for service in &config.services {
        let entry = dependency_map
            .entry(service.name.clone())
            .or_insert_with(Vec::new);

        if let Some(deps) = &service.deps {
            entry.extend(deps.clone());
        }
    }

    // Create a map over the dependencies and the services that depend on them
    let mut reverse_dependency_map: HashMap<String, Vec<String>> = HashMap::new();

    for (service, deps) in &dependency_map {
        for dep in deps {
            reverse_dependency_map
                .entry(dep.clone())
                .or_insert_with(Vec::new)
                .push(service.clone());
        }
    }

    println!("Dependency Map:");
    for (service, deps) in &dependency_map {
        println!("  {}: {:?}", service, deps);
    }

    println!("\nReverse Dependency Map:");
    for (dep, services) in &reverse_dependency_map {
        println!("  {}: {:?}", dep, services);
    }

    let mut not_installed: Vec<String> = dependency_map.keys().cloned().collect();

    let mut installing: Vec<String> = Vec::new();
    let mut installed: Vec<String> = Vec::new();

    // Initialize installing with services that have no dependencies
    for (name, deps) in &dependency_map {
        if deps.is_empty() {
            not_installed.retain(|n| n != name);
            installing.push(name.clone());
        }
    }

    let mut iteration = 0;

    while !not_installed.is_empty() || !installing.is_empty() {
        // Mark previously installing services as done
        let mut done = Vec::new();

        for name in &installing {
            done.push(name.clone());
        }

        // Move done services from installing to installed
        for name in &done {
            installing.retain(|n| n != name);
            installed.push(name.clone());
        }

        for name in &done {
            // Find the dependencies of the service that was just installed
            let next_jobs = reverse_dependency_map
                .get(name)
                .unwrap_or(Vec::new().as_ref())
                .iter()
                // Filter out the services that are already installed or installing
                .filter(|s| not_installed.contains(s))
                .cloned()
                .collect::<Vec<String>>();

            for service_name in next_jobs {
                // Find the dependencies of the next job
                let deps = dependency_map
                    .get(&service_name)
                    .unwrap_or(&Vec::new())
                    .clone();

                // Verify if all dependencies are satisfied before starting the next job
                let deps_satisfied = deps.iter().all(|dep| installed.contains(dep));

                // Queue the service if all dependencies are satisfied
                if deps_satisfied {
                    installing.push(service_name.clone());
                    not_installed.retain(|n| n != &service_name);
                }
            }
        }

        println!("\nIteration: {}", iteration);
        iteration += 1;

        for name in &done {
            println!("  - Installed: {}", name);
        }
    }
}

fn main() {
    progressive_lookup();
}
