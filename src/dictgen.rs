mod random;
pub mod settings;
pub mod generators;

use settings::Settings;
use generators::Value;
use std::collections::HashMap;
use anyhow::Result;
use rand::Rng;
use self::generators::Generator;

pub struct DictGen {}

impl DictGen {
    pub fn generate(
        settings: &Settings
    ) -> Result<HashMap<String, Value>, String> {
        let max_height = settings.max_height;
        let max_depth = settings.max_depth;

        if max_height < 1 || max_depth < 1 {
            return Err("max_height and max_depth must be greater than 0".into());
        }

        let mut all_generators: Vec<Generator> = settings.value_generators.to_vec();

        if max_depth > 1 {
            all_generators.extend(settings.nested_generators);
        }

        let mut rng = rand::thread_rng();
        let mut generator_tuples = Vec::new();
        for _ in 0..rng.gen_range(0..max_height + 1) {
            generator_tuples.push((
                settings.key_generators[rng.gen_range(0..settings.key_generators.len())],
                all_generators[rng.gen_range(0..all_generators.len())],
            ));
        }

        let mut result = HashMap::new();
        for (key_generator, val_generator) in generator_tuples {
            let key = key_generator(&settings);
            let value = val_generator(&settings);
        
            result.insert(key.to_string(), value);
        }
        
        Ok(result)
    }
}