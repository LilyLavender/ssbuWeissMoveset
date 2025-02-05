use super::*;

unsafe extern "C" fn marth_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);

	if is_weiss(boma) {
		// Mode switching
		let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
		let selected_dust = WorkModule::get_int(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
		if selected_dust == 1 { // Fire
			WorkModule::set_float(boma, 4.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_R);
			WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_G);
			WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_B);
			AttackModule::set_power_up(boma, 1.1);
			AttackModule::set_reaction_mul(boma, 1.1);
			DamageModule::set_damage_mul(boma, 1.3);
			DamageModule::set_reaction_mul(boma, 1.5);
		}
		if selected_dust == 0 { // Normal
			WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_R);
			WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_G);
			WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_B);
			AttackModule::set_power_up(boma, 1.0);
			AttackModule::set_reaction_mul(boma, 1.0);
			DamageModule::set_damage_mul(boma, 1.0);
			DamageModule::set_reaction_mul(boma, 1.0);
		}
		if selected_dust == 3 { // Ice
			WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_R);
			WorkModule::set_float(boma, 4.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_G);
			WorkModule::set_float(boma, 4.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_B);
			smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 0.9);
    	    fighter.clear_lua_stack();
    	    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.0);
    	    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
			AttackModule::set_power_up(boma, 0.8);
			AttackModule::set_reaction_mul(boma, 0.8);
			DamageModule::set_damage_mul(boma, 0.7);
			DamageModule::set_reaction_mul(boma, 0.7);
		}
		if selected_dust == 2 { // Lightning
			WorkModule::set_float(boma, 4.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_R);
			WorkModule::set_float(boma, 4.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_G);
			WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_B);
			smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 1.4);
    	    fighter.clear_lua_stack();
    	    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 3.2);
    	    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    	    fighter.clear_lua_stack();
    	    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.4);
    	    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
			AttackModule::set_power_up(boma, 0.8);
			AttackModule::set_reaction_mul(boma, 0.8);
			DamageModule::set_damage_mul(boma, 1.0);
			DamageModule::set_reaction_mul(boma, 1.0);
		}
		// If percent when dust was selected is 30 less than percent now, remove dust
		let perc_select = WorkModule::get_float(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_PERCENTAGE_AT_SELECTION);
		if perc_select + 30.0 <= DamageModule::damage(boma, 0) {
			WorkModule::set_int(boma, 0, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
			WorkModule::set_float(boma, 999.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_PERCENTAGE_AT_SELECTION);
			macros::EFFECT(fighter, Hash40::new("marth_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		// If player dies or training mode reset, remove dust without playing effect
		if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DEAD || !sv_information::is_ready_go() {
    	    WorkModule::on_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_DUST_FIRE_USABLE);
    	    WorkModule::on_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_DUST_LIGHTNING_USABLE);
    	    WorkModule::on_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_DUST_ICE_USABLE);
			WorkModule::set_int(boma, 0, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
			WorkModule::set_float(boma, 999.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_PERCENTAGE_AT_SELECTION);
		}
		// Summon glow effect
		WorkModule::inc_int(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_EFFECT_COUNT);
		let effect_count = WorkModule::get_int(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_EFFECT_COUNT);
		let r = WorkModule::get_float(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_R);
		let g = WorkModule::get_float(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_G);
		let b = WorkModule::get_float(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_B);
		if effect_count == 5 || effect_count == 10 {
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
			macros::LAST_EFFECT_SET_COLOR(fighter, r, g, b);
		}
		if effect_count >= 20 {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
			macros::LAST_EFFECT_SET_COLOR(fighter, r, g, b);
			WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_EFFECT_COUNT);
		}
	}
}

pub fn install() {
    Agent::new("marth")
        .on_line(Main, marth_frame)
        .install();
}
