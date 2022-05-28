pub mod settings {
    use crate::engines;

    pub struct Config {
        beautify: bool,
        time_limit: u16,
        engine: engines::engines::Engine,
        start_as_white: bool,
    }

    impl Config {
        pub fn new() -> Config {
            Config { beautify: true, time_limit: 0, start_as_white: true, engine: engines::engines::Engine::Player }
        }

        pub fn get_user_config() -> Config{
            Config { beautify: Config::read_beautify_value(), time_limit: Config::read_time_limit(), engine: Config::read_engine(), start_as_white: Config::read_start_as_white() }
        }

        fn read_beautify_value() -> bool {
            println!("Beautify? (true/false)");
            let mut beautify = String::new();
            std::io::stdin()
                .read_line(&mut beautify)
                .expect("Could not read line from user!");
            beautify.trim().parse().expect("Not a valid value!")
        }

        fn read_time_limit() -> u16 {
            println!("Time in seconds (0 for no time limit)");
            let mut time = String::new();
            std::io::stdin()
                .read_line(&mut time)
                .expect("Could not read line from user!");
            time.trim().parse().expect("Not a valid value")
        }

        fn read_engine() -> engines::engines::Engine {
            engines::engines::Engine::Player
        }

        fn read_start_as_white() -> bool {
            println!("Would you like to start as white (true/false)");
            let mut start_as_white = String::new();
            std::io::stdin()
                .read_line(&mut start_as_white)
                .expect("Could not read line from user!");
            start_as_white.trim().parse().expect("Not a valid value!")
        }

        pub fn get_beautify(&self) -> bool {
            self.beautify
        }

        pub fn get_time_limit(&self) -> u16 {
            self.time_limit
        }

        pub fn get_engine(&self) -> engines::engines::Engine {
            self.engine
        }

        pub fn get_start_as_white(&self) -> bool {
            self.start_as_white
        }
    }
}
