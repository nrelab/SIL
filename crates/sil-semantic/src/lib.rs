#![warn(clippy::pedantic)]

pub mod cluster;
pub mod similarity;

pub use cluster::cluster_intents;
pub use similarity::semantic_similarity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity_related() {
        let score = semantic_similarity("login_user", "authenticate_user");
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_similarity_identical() {
        let score = semantic_similarity("login_user", "login_user");
        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_similarity_unrelated() {
        let score = semantic_similarity("login_user", "payment_process");
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_cluster_intents() {
        let items = vec![
            "login_user",
            "authenticate_user",
            "user_login_handler",
            "payment_process",
        ];
        let clusters = cluster_intents(items);

        assert!(clusters.contains_key("AUTH"));
        assert!(clusters.contains_key("USER_MGMT"));
        assert!(clusters.contains_key("PAYMENT"));
    }

    #[test]
    fn test_cluster_auth_count() {
        let items = vec!["login_user", "authenticate_user", "user_login_handler"];
        let clusters = cluster_intents(items);

        let auth = clusters.get("AUTH").unwrap();
        assert_eq!(auth.len(), 2);

        let user_mgmt = clusters.get("USER_MGMT").unwrap();
        assert_eq!(user_mgmt.len(), 1);
    }

    #[test]
    fn test_empty_input_cluster() {
        let items: Vec<&str> = vec![];
        let clusters = cluster_intents(items);
        assert!(clusters.is_empty());
    }

    #[test]
    fn test_general_fallback() {
        let items = vec!["unknown_function"];
        let clusters = cluster_intents(items);
        assert!(clusters.contains_key("GENERAL"));
    }
}
