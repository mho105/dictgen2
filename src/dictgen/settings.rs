use super::generators::Generators;

pub struct Settings<'a> {
    pub max_height: u32,
    pub max_depth: u32,
    pub key_generators: &'a Generators,
    pub value_generators: &'a Generators,
    pub nested_generators: &'a Generators,
}

impl<'a> Settings<'a> {
    pub fn decrease_max_depth(&self) -> Settings{
        Settings { 
            max_height: self.max_height,
            max_depth: self.max_depth - 1,
            key_generators: self.key_generators,
            value_generators: self.value_generators,
            nested_generators: self.nested_generators, 
        }
    }
}