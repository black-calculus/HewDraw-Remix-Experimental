use super::*;
use utils::ext::*;


// Doubles camera speed
#[skyline::hook(offset = 0x4fdbf0)]
unsafe fn normal_camera(ptr: u64, float: f32) {
    call_original!(ptr, float);
    call_original!(ptr, float);
}

// Standardizes normal_camera_min_distance for all stages
#[skyline::hook(offset = 0x26209bc, inline)]
unsafe fn parse_stprm_normal_camera_min_distance(ctx: &mut skyline::hooks::InlineCtx) {
    let normal_camera_min_distance: f32;
    asm!("fmov w20, s0", out("w20") normal_camera_min_distance);
    if normal_camera_min_distance < 125.0 {
        let normal_camera_min_distance: f32 = 125.0;
        asm!("fmov s0, w8", in("w8") normal_camera_min_distance);
    }
}

// Standardizes normal_camera_vertical_angle and normal_camera_look_down_vertical_angle for all stages
#[skyline::hook(offset = 0x2620e50, inline)]
unsafe fn parse_stprm_normal_camera_angles(ctx: &mut skyline::hooks::InlineCtx) {
    let hash = *ctx.registers[18].x.as_ref();
    let mut angle: f32 = 0.0;
    if hash == hash40("normal_camera_vertical_angle") {
        angle = -5.5;
    }
    else if hash == hash40("normal_camera_look_down_vertical_angle") {
        angle = -25.0;
    }
    asm!("fmov s0, w8", in("w8") angle)
}

// Standardizes target_interpolation_rate for all stages
#[skyline::hook(offset = 0x2620fec, inline)]
unsafe fn parse_stprm_target_interpolation_rate(ctx: &mut skyline::hooks::InlineCtx) {
    let target_interpolation_rate: f32 = 1.0;
    asm!("fmov s1, w8", in("w8") target_interpolation_rate)
}

pub fn install() {
    unsafe {
        skyline::patching::Patch::in_text(0x2620e50).nop();
        skyline::patching::Patch::in_text(0x2620fec).nop();
    }
    skyline::install_hooks!(
        normal_camera,
        parse_stprm_normal_camera_min_distance,
        parse_stprm_normal_camera_angles,
        parse_stprm_target_interpolation_rate
    );
}