use tracing::info;

macro_rules! animal_behaviour_expr {
    ($animal:expr, $behaviour:expr) => {
        Box::pin(async {
            info!("The {} is {}", $animal, $behaviour);
        })
    };
}

macro_rules! animal_behaviour {
    ($animal:ident, $behaviour:expr) => {
        fn $animal() {
            info!("The {} is {}", stringify!($animal), $behaviour);
        }
    };
}

macro_rules! define_config {
    ($(
        $(#[doc = $doc:literal])?
        ($name:ident: $ty:ty = $default:expr),
    )*) => {
        // Struct definition
        #[derive(Debug)]
        pub struct Config {
            $(
                $(#[doc = $doc])?
                pub $name: $ty,
            )*
        }

        // Default values module
        mod defaults {
            use super::*;
            $(
                pub fn $name() -> $ty { $default }
            )*
        }

        // Implement Default trait
        impl Default for Config {
            fn default() -> Self {
                Self {
                    $(
                        $name: defaults::$name(),
                    )*
                }
            }
        }
    };
}

macro_rules! define_config_w_validation {
    ($(
        $(#[doc = $doc:literal])?
        $(#[deprecated($dep:literal, $new_field:ident)])?
        $(#[validate = $validate:expr])?
        ($name:ident: $ty:ty = $default:expr),
    )*) => {
        // Struct definition
        #[derive(Debug)]
        pub struct Config {
            $(
                $(#[doc = $doc])?
                pub $name: $ty,
            )*
        }

        // Default values module
        mod defaults {
            use super::*;
            $(
                pub fn $name() -> $ty { $default }
            )*
        }

        // Implement Default trait
        impl Default for Config {
            fn default() -> Self {
                Self {
                    $(
                        $name: defaults::$name(),
                    )*
                }
            }
        }

        // Validation implementation
        impl Config {
            pub fn validate(&self) -> Result<(), String> {
                let mut errors = vec![];
                $(
                    if let Some(validation_fn) = $($validate)? {
                        if let Err(e) = validation_fn(&self.$name) {
                            errors.push(format!("Field `{}`: {}", stringify!($name), e));
                        }
                    }
                )*
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors.join("\n"))
                }
            }

            pub fn check_deprecated(&self) {
                $(
                    if let Some(deprecated_msg) = $($dep)? {
                        println!("Warning: Field `{}` is deprecated. {}", stringify!($name), deprecated_msg);
                        println!("Use `{}` instead.", stringify!($($new_field)?));
                    }
                )*
            }
        }
    };
}

pub async fn run_animal_behavior_macro() {
    animal_behaviour_expr!("dog", "barking").await;
    animal_behaviour!(cat, "meowing");
    cat();
    // define_config! {
    //     /// The number of threads to use.
    //     (num_threads: usize = 4),
    //
    //     /// The timeout duration in seconds.
    //     (timeout_seconds: u64 = 30),
    //
    //     /// The path to the configuration file.
    //     (config_path: String = String::from("/etc/app/config.toml")),
    // }
    // let config = Config::default();
    // info!("Config: {:?}", config);

    define_config_w_validation! {
        /// The number of threads to use.
        (num_threads: usize = 4),

        /// The timeout duration in seconds.
        (timeout_seconds: u64 = 30),

        /// The path to the configuration file.
        (config_path: String = String::from("/etc/app/config.toml")),

        /// A deprecated configuration field.
        #[deprecated("Use `new_field` instead", new_field)]
        (old_field: String = String::from("deprecated")),

        /// A new configuration field.
        (new_field: String = String::from("new value")),

        /// A field with custom validation.
        #[validate = |value: &usize| if *value > 100 { Err("must be 100 or less") } else { Ok(()) }]
        (max_connections: usize = 50),
    }
    let config = Config::default();
    info!("config: {:?}", config);

    // Check for deprecated fields
    // config.check_deprecated();
    //
    // // Validate configuration
    // match config.validate() {
    //     Ok(_) => println!("Configuration is valid."),
    //     Err(e) => println!("Configuration errors:\n{}", e),
    // }
}
