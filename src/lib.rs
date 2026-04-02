mod channel_with_scope;
mod channel_with_spawn;
mod constants;
mod thread_scope;

use crate::{
    channel_with_scope::use_channel_with_scope, channel_with_spawn::use_channel_with_spawn,
    thread_scope::use_thread_scope,
};

pub fn run() {
    use_thread_scope();
    use_channel_with_scope();
    use_channel_with_spawn();
}
