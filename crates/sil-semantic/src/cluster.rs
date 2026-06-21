use std::collections::HashMap;

/// Clusters a list of intent strings into named groups using keyword matching.
///
/// # Categories
/// - `AUTH` — items containing "login"
/// - `USER_MGMT` — items containing "user"
/// - `PAYMENT` — items containing "payment"
/// - `GENERAL` — fallback for unrecognized intents
///
/// Returns a map from category name to the list of matching items.
/// Returns an empty map if the input list is empty.
#[must_use]
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
