use crate::key_mapping::KeyMapping;

#[derive(Debug)]
pub(crate) enum KeyMappings {
    Linear,
    Custom(Vec<KeyMapping>),
}
