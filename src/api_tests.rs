#[cfg(test)]
mod api_tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use crate::load_env_vars;

    fn create_env_file(content: &str) {
        let path = Path::new(".env");
        if path.exists() {
            remove_env_file();
        }
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    // Helper function to remove the .env file
    fn remove_env_file() {
        let path = Path::new(".env");
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn test_load_env_vars_success() {
        create_env_file(
            "IP=192.168.1.1\nPORT=8000\nGOOGLE_API_KEY=your_api_key\nGOOGLE_SEARCH_ENGINE_ID=your_search_engine_id",
        );

        let result = load_env_vars();

        assert!(result.is_ok());
        let env_values = result.unwrap();
        assert_eq!(env_values.ip, "192.168.1.1");
        assert_eq!(env_values.port, "8000");
        assert_eq!(env_values.api_key, "your_api_key");
        assert_eq!(env_values.search_engine_id, "your_search_engine_id");

        remove_env_file();
    }
    #[test]
    fn test_load_env_vars_missing_api_key() {
        create_env_file("IP=192.168.1.1\nPORT=8000");

        let result = load_env_vars();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "GOOGLE_API_KEY must be set".to_string()
        );

        remove_env_file();
    }
    #[test]
    fn test_load_env_vars_missing_search_engine_id() {
        create_env_file("IP=192.168.1.1\nPORT=8000\nGOOGLE_API_KEY=your_api_key");

        let result = load_env_vars();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "GOOGLE_SEARCH_ENGINE_ID must be set".to_string()
        );

        remove_env_file();
    }

    #[test]
    fn test_load_env_vars_defaults() {
        create_env_file("");

        let result = load_env_vars();
        assert!(result.is_err());

        remove_env_file();
    }

    #[test]
    fn test_load_env_vars_file_not_found() {
        let result = load_env_vars();
        assert!(result.is_err());
    }
}
