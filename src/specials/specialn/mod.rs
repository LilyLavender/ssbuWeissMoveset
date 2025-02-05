use super::*;

unsafe extern "C" fn marth_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	if !is_weiss(fighter.module_accessor) {
		return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
	}
	PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
	fighter.sub_shift_status_main(L2CValue::Ptr(marth_specialn_main_loop as *const () as _))
}

unsafe extern "C" fn marth_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }

    let is_end = MotionModule::is_end(fighter.module_accessor);
    if is_end {
        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    
    let is_changing = StatusModule::is_changing(fighter.module_accessor);
    if is_changing || fighter.global_table[0x17].get_i32() != fighter.global_table[0x16].get_i32() {
        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
				WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST)
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
            }
        } else {
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
				WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST)
			} else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn marth_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
	if !is_weiss(fighter.module_accessor) {
		return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
	}
	fighter.sub_status_pre_SpecialNCommon();
	StatusModule::init_settings(
		fighter.module_accessor, 
		smash::app::SituationKind(*SITUATION_KIND_NONE),  
		*FIGHTER_KINETIC_TYPE_UNIQ, 
		GROUND_CORRECT_KIND_KEEP.into(), 
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
		true, 
		FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG.into(), 
		*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
		FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT.into(), 
		0
	);
	FighterStatusModuleImpl::set_fighter_status_data(
		fighter.module_accessor, 
		false, 
		*FIGHTER_TREADED_KIND_NO_REAC, 
		false, 
		false, 
		false, 
		(*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 
		FIGHTER_STATUS_ATTR_START_TURN.into(), 
		FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N.into(), 
		0
	);
	return 0.into();
}

unsafe extern "C" fn marth_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
	if !is_weiss(fighter.module_accessor) {
		return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
	}
	let selected_dust = WorkModule::get_int(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
	if selected_dust != 0 {
		macros::EFFECT(fighter, Hash40::new("marth_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
	WorkModule::set_float(fighter.module_accessor, 999.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_PERCENTAGE_AT_SELECTION);
	0.into()
}

unsafe extern "C" fn marth_game_specialn(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let selected_dust = WorkModule::get_int(agent.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
	if selected_dust == 0 { // Normal
		frame(agent.lua_state_agent, 9.0);
		for i in 0..7 {
			if macros::is_excute(agent) {
				macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.8, 367, 25, 0, 35, 13.0, 0.0, 12.0, 4.0, Some(0.0), Some(12.0), Some(-4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
				macros::EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
				macros::EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
				macros::PLAY_SE(agent, Hash40::new(&format!("se_common_swing_0{}", i + 1)));
			}
			wait(agent.lua_state_agent, 3.0);
			if macros::is_excute(agent) {
				AttackModule::clear_all(agent.module_accessor);
			}
		}
	} else if selected_dust == 1 { // Fire
		macros::FT_MOTION_RATE(agent, 8.0);
		frame(agent.lua_state_agent, 5.0);
		macros::FT_MOTION_RATE(agent, 1.0);
		frame(agent.lua_state_agent, 9.0);
		if macros::is_excute(agent) {
			macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 16.0, 0.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
			macros::ATTACK(agent, 0, 0, Hash40::new("top"), 24.0, 361, 85, 0, 58, 26.0, 0.0, 16.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
			macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
		}
		frame(agent.lua_state_agent, 12.0);
		if macros::is_excute(agent) {
			AttackModule::clear_all(agent.module_accessor);
			DamageModule::add_damage(agent.module_accessor, 12.0, 0);
			StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_STATUS_KIND_FURAFURA.into(), false.into());
		}
	} else if selected_dust == 2 { // Lighting
		frame(agent.lua_state_agent, 9.0);
		if macros::is_excute(agent) {
			macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 361, 77, 0, 70, 13.0, 0.0, 10.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("weiss_lightning"), Hash40::new("top"), 0, 8, 0, 0, 90, 0, 1, true);
			macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("weiss_lightning2"), Hash40::new("top"), 0, 8, 0, 0, 90, 0, 1.3, true);
			macros::PLAY_SE(agent, Hash40::new("se_common_fire_l"));
		}
		frame(agent.lua_state_agent, 12.0);
		if macros::is_excute(agent) {
			AttackModule::clear_all(agent.module_accessor);
		}
	} else { // Ice
		frame(agent.lua_state_agent, 9.0);
		if macros::is_excute(agent) {
			let facing = PostureModule::lr(agent.module_accessor);
			macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.2, 98, 85, 0, 30, 10.0, 5.0, 0.0, 16.0, Some(-5.0), Some(-20.0), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
			macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.2, 82, 85, 0, 30, 10.0, 5.0, 0.0, -16.0, Some(-5.0), Some(-20.0), Some(-16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
			macros::EFFECT(agent, Hash40::new("weiss_ice_sword_2_l"), Hash40::new("top"), 0, 10.0, -16.0 * facing, 0.0, 0.0, 0.0, 1.8, 0, 0, 0, 0, 0, 0, true);
			macros::EFFECT(agent, Hash40::new("weiss_ice_sword_2_r"), Hash40::new("top"), 0, 10.0, 16.0 * facing, 0.0, 0.0, 0.0, 1.8, 0, 0, 0, 0, 0, 0, true);	
			macros::PLAY_SE(agent, Hash40::new("se_common_frieze_ll"));
		}
		frame(agent.lua_state_agent, 12.0);
		if macros::is_excute(agent) {
			AttackModule::clear_all(agent.module_accessor);
		}
	}
}

pub fn install() {
    Agent::new("marth")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, marth_specialn_main)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, marth_specialn_pre)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, marth_specialn_end)
        .game_acmd("game_specialn_weiss", marth_game_specialn, Default)
        .game_acmd("game_specialairn_weiss", marth_game_specialn, Default)
        .install();
}
