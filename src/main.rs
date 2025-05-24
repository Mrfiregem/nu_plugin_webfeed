#![deny(unused_crate_dependencies)]

mod commands;
mod util;

pub struct WebfeedPlugin;

impl nu_plugin::Plugin for WebfeedPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(commands::FeedFetch)]
    }
}

fn main() {
    nu_plugin::serve_plugin(&WebfeedPlugin, nu_plugin::MsgPackSerializer);
}
