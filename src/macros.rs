macro_rules! try_yaml {
    ($result: expr, Yaml::Array, $error_msg: expr) => {{ match $result {
        Yaml::Array(ref res) => res,
        _ => return Err(KeyboardError::Config($error_msg.into())),
    }}};
    ($result: expr, Yaml::String, $error_msg: expr) => {{ match $result {
        Yaml::String(ref res) => res,
        _ => return Err(KeyboardError::Config($error_msg.into())),
    }}};
    ($result: expr, Yaml::Hash, $error_msg: expr) => {{ match $result {
        Yaml::Hash(ref res) => res,
        _ => return Err(KeyboardError::Config($error_msg.into())),
    }}};
}
