use crate::{EmEnv, Exited};

// __exit
pub fn exit(_ctx: &EmEnv, value: i32) -> Result<(), Exited> {
    debug!("emscripten::exit {}", value);
    Err(Exited(value))
}
