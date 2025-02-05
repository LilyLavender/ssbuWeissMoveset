use super::*;

unsafe extern "C" fn marth_iceshard_glyph_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let mut pos = *PostureModule::pos(weapon.module_accessor);
    pos.x = pos.x + 24.0 * PostureModule::lr(weapon.module_accessor);
    pos.y = pos.y + 26.0;
    PostureModule::set_pos(weapon.module_accessor, &pos);
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT, 
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 
        0
    );
    return 0.into();
}

unsafe extern "C" fn marth_iceshard_glyph_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let glyph_id = (*(weapon.module_accessor)).battle_object_id as i64;
    WorkModule::set_int64(owner_boma, glyph_id, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_ICESHARD_ID);
	MotionModule::change_motion(weapon.module_accessor, Hash40::new("glyph"), 0.0, 1.0, false, 0.0, false, false);
	weapon.fastshift(L2CValue::Ptr(marth_iceshard_glyph_main_loop as *const () as _))
}

unsafe extern "C" fn marth_iceshard_glyph_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, WEAPON_MARTH_ICESHARD_INSTANCE_WORK_ID_FLAG_KILL) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
	return 0.into();
}

unsafe extern "C" fn marth_iceshard_glyph_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("weiss_glyph"), true, true);
	return 0.into();
}

unsafe extern "C" fn marth_iceshard_effect_glyph(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
        let facing = PostureModule::lr(agent.module_accessor);
		macros::EFFECT(agent, Hash40::new("weiss_glyph"), Hash40::new("top"), 0, 0, 0, 90, 30.0 * facing, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
	}
}

pub fn install() {
    Agent::new("marth_iceshard")
        .status(Pre, WEAPON_MARTH_ICESHARD_STATUS_KIND_GLYPH, marth_iceshard_glyph_pre)
        .status(Main, WEAPON_MARTH_ICESHARD_STATUS_KIND_GLYPH, marth_iceshard_glyph_main)
        .status(End, WEAPON_MARTH_ICESHARD_STATUS_KIND_GLYPH, marth_iceshard_glyph_end)
        .effect_acmd("effect_glyph", marth_iceshard_effect_glyph, Default)
        .install();
}
