use std::collections::HashMap;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use super::{settings::Settings, DictGen, generators::Value};

pub struct Random {}

impl Random {
    pub fn int() -> i32 {
        let mut rng = thread_rng();
        
        rng.gen::<i32>()
    }

    pub fn string() -> String {
        let num_chars = rand::thread_rng().gen_range(1..=10);
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(num_chars)
            .map(char::from)
            .collect()
    }

    pub fn float() -> f32 {
        let mut rng = thread_rng();
        
        rng.gen::<f32>()
    }

    pub fn bool() -> bool {
        let mut rng = thread_rng();

        rng.gen::<bool>()
    }

    pub fn dict(settings: &Settings) -> HashMap<String, Value> {
        let new_settings = settings.decrease_max_depth();
        DictGen::generate(&new_settings).unwrap()
    }

    pub fn array(settings: &Settings) -> Vec<Value> {
        let mut rng = rand::thread_rng();
        let rand_height = rng.gen_range(0..settings.max_height);

        let generators = (0..rand_height).map(|_| {
            if settings.max_depth > 2 {
                let mut all_generators = settings.value_generators.to_vec();
                all_generators.extend(settings.nested_generators);
                all_generators[rng.gen_range(0..all_generators.len())]
            } else {
                settings.value_generators[rng.gen_range(0..settings.value_generators.len())]
            }
        }).collect::<Vec<_>>();

        let new_settings = settings.decrease_max_depth();
        generators
            .iter()
            .map(|generator| generator(&new_settings))
            .collect()
    }
}