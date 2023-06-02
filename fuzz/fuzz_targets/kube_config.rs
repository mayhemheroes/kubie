use honggfuzz::fuzz;
use kubie::kubeconfig::{KubeConfig, NamedCluster, NamedUser, NamedContext};
use std::path::Path;

fn main() {
    // Define the fuzzing loop
    loop {
        // Fuzzing input will be provided by Honggfuzz
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(path) = std::str::from_utf8(data) {
                // Create a KubeConfig instance
                let kube_config = KubeConfig {
                    clusters: vec![NamedCluster {
                        name: "cluster1".to_string(),
                        cluster: serde_yaml::Value::String("cluster1.example.com".to_string()),
                    }],
                    users: vec![NamedUser {
                        name: "user1".to_string(),
                        user: serde_yaml::Value::String("user1@example.com".to_string()),
                    }],
                    contexts: vec![NamedContext {
                        name: "context1".to_string(),
                        context: crate::Context {
                            cluster: "cluster1".to_string(),
                            namespace: Some("namespace1".to_string()),
                            user: "user1".to_string(),
                        },
                    }],
                    current_context: Some("context1".to_string()),
                    others: Default::default(),
                };
                
                // Call the write_to_file function with the fuzzed input
                let _ = kube_config.write_to_file(Path::new(path));

            }
        });
    }
}
