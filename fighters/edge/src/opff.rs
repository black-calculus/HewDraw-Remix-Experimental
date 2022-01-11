use ::common::opff_import::*;
use super::*;
use globals::*;
use ::common::opff::*;
 
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    let long_sword_scale = Vector3f{x: 0.9, y: 1.0, z: 1.0};
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordl1"), &long_sword_scale);
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordr1"), &long_sword_scale);
    //println!("Sephiroth Success! Sephiroth's Fighter Kind Number: {}", *FIGHTER_KIND_EDGE);
}

// Limit Blade Rush jump cancel on hit
unsafe fn limit_blade_rush_jc(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32) {
    //println!("Sephiroth status kind: {}", status_kind);
    if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH && WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) && situation_kind == *SITUATION_KIND_GROUND {
        //println!("Limit Blade Rush");
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            //println!("========== Limit Blade Rush hit!");
            if moveset_utils::jump_checker_buffer(boma, cat1) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT,true);
            }
        }
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //sword_length(boma);
    limit_blade_rush_jc(boma, cat[0], status_kind, situation_kind);
    nspecial_cancels(boma, status_kind, situation_kind, cat[1]);
}

#[utils::opff(FIGHTER_KIND_EDGE )]
pub fn edge_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		edge_frame(fighter)
    }
}

pub unsafe fn edge_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
