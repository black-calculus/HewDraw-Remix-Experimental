#![feature(asm)]

pub mod acmd;

//pub mod status;
//pub mod opff;

use smash::{
    app::{
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    lib::lua_const::*,
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use utils::{
    *,
    ext::*,
    consts::*,
};
use smashline::*;

pub fn install(is_runtime: bool) {
    acmd::install();
    // status::install();
    //opff::install(is_runtime);
}