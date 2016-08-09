#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
extern crate syntax;
extern crate rustc_plugin;

mod expand;

use rustc_plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("concat_bytes", expand::expand_syntax_ext);
}
