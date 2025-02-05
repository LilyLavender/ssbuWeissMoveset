use super::*;

pub unsafe extern "C" fn is_exist_glyph(o_boma: *mut BattleObjectModuleAccessor) -> bool {
    let a_id = WorkModule::get_int64(o_boma, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_ICESHARD_ID);
    let is_active = sv_battle_object::is_active(a_id as u32);
    let a_boma = smash::app::sv_battle_object::module_accessor(a_id as u32);
    let is_glyph = StatusModule::status_kind(a_boma) == WEAPON_MARTH_ICESHARD_STATUS_KIND_GLYPH;
    a_id != 0 && is_active && is_glyph
}

pub unsafe extern "C" fn change_status_glyph(o_boma: *mut BattleObjectModuleAccessor, status: i32) {
    let a_id = WorkModule::get_int64(o_boma, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_ICESHARD_ID);
    let a_boma = smash::app::sv_battle_object::module_accessor(a_id as u32);
    StatusModule::change_status_request_from_script(a_boma, status, true);
}

pub unsafe extern "C" fn get_glyph_position(o_boma: *mut BattleObjectModuleAccessor) -> *const Vector3f {
    let a_id = WorkModule::get_int64(o_boma, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_ICESHARD_ID);
    let a_boma = smash::app::sv_battle_object::module_accessor(a_id as u32);
    PostureModule::pos(a_boma)
}

pub unsafe extern "C" fn kill_glyph(o_boma: *mut BattleObjectModuleAccessor) {
    let a_id = WorkModule::get_int64(o_boma, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_ICESHARD_ID);
    let a_boma = smash::app::sv_battle_object::module_accessor(a_id as u32);
    WorkModule::on_flag(a_boma, WEAPON_MARTH_ICESHARD_INSTANCE_WORK_ID_FLAG_KILL);
}