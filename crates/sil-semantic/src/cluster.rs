use std::collections::HashMap;

pub fn cluster_intents(items: Vec<&str>) -> HashMap<String, Vec<String>> {
    let mut clusters: HashMap<String, Vec<String>> = HashMap::new();

    for item in items {
        let key = generate_cluster_key(item);

        clusters.entry(key).or_insert(vec![]).push(item.to_string());
    }

    clusters
}

fn generate_cluster_key(input: &str) -> String {
    let normalized = input.to_lowercase();

    if normalized.contains("login") {
        return "AUTH".to_string();
    }

    if normalized.contains("user") {
        return "USER_MGMT".to_string();
    }

    if normalized.contains("payment") {
        return "PAYMENT".to_string();
    }

    "GENERAL".to_string()
}
