fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = parser::settings::Settings::from_env();
    let app = parser::App::new(settings);
    app.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        // I don't want to deal with mocking settings here.
        // So, it's just a very basic test to make sure nothing weird happens.
        let result = main();

        assert!(result.is_ok());
    }
}
