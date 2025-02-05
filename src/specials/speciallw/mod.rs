use super::*;

unsafe extern "C" fn marth_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_weiss(fighter.module_accessor) {
		return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
	}
    let boma = fighter.module_accessor;
    if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(boma, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(boma, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
	fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }

    let is_end = MotionModule::is_end(boma);
    if is_end {
        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }

    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
        CancelModule::enable_cancel(boma);
    }
    
    let is_changing = StatusModule::is_changing(boma);
    if is_changing || fighter.global_table[0x17].get_i32() != fighter.global_table[0x16].get_i32() {
        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
            if !WorkModule::is_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                MotionModule::change_motion(boma, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
				WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT)
            } else {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
            }
        } else {
            GroundModule::correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
            if !WorkModule::is_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                MotionModule::change_motion(boma, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
				WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT)
			} else {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }

    //if is_changing {
        if fighter.global_table[0x17] == *SITUATION_KIND_AIR
        && fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
        let status_frame = fighter.global_table[0xe].get_f32();
        if fighter.global_table[0x17] == *SITUATION_KIND_GROUND
        && fighter.global_table[0x16] == *SITUATION_KIND_AIR
        && status_frame > 2.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    //}

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if WorkModule::is_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_GLYPH_TELEPORT_END) {
        GroundModule::set_passable_check(boma, true);
        GroundModule::set_collidable(boma, true);
        JostleModule::set_status(boma, true);
        WorkModule::off_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_GLYPH_TELEPORT_END);
    }
    if WorkModule::is_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_GLYPH_TELEPORT) {
        if helper::is_exist_glyph(boma) {
            GroundModule::set_passable_check(boma, false);
            GroundModule::set_collidable(boma, false);
            JostleModule::set_status(boma, false);
            let pos = helper::get_glyph_position(boma);
            let new_pos = &Vector3f{ x: (*pos).x, y: (*pos).y - 10.0, z: (*pos).z };
            PostureModule::set_pos(boma, pos);
            helper::kill_glyph(boma);
        }
        WorkModule::off_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_GLYPH_TELEPORT);
        WorkModule::on_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_GLYPH_TELEPORT_END);
    }

    if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_frame = MotionModule::frame(boma);
        let half_height = WorkModule::get_param_float(boma, hash40("height"), 0) / 2.0;
        let facing = PostureModule::lr(boma);

        if motion_frame >= 10.0 && motion_frame <= 44.0 {
            let fire_usable = WorkModule::is_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_DUST_FIRE_USABLE);
            let lightning_usable = WorkModule::is_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_DUST_LIGHTNING_USABLE);
            let ice_usable = WorkModule::is_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_DUST_ICE_USABLE);
            // Summon effects
            if fire_usable {
                macros::EFFECT(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0, half_height + 14.0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 4.0, 0.0, 0.0);
            }
            if lightning_usable {
                macros::EFFECT(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, half_height, 12.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 4.0, 4.0, 0.0);
            }
            if ice_usable {
                macros::EFFECT(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, half_height, -12.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 4.0, 4.0);
            }

            // Handle switching
            if PostureModule::lr(boma) == 1.0 {
                if ControlModule::get_stick_y(boma) > 0.707 && fire_usable {
                    WorkModule::set_int(boma, 1, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
                } else if ControlModule::get_stick_x(boma) < -0.707 && ice_usable {
                    WorkModule::set_int(boma, 3, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
                } else if ControlModule::get_stick_x(boma) > 0.707 && lightning_usable {
                    WorkModule::set_int(boma, 2, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
                } else if ControlModule::get_stick_y(boma) < -0.707 {
                    WorkModule::set_int(boma, 0, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
                }
            } else {
                if ControlModule::get_stick_y(boma) > 0.707 && fire_usable {
                    WorkModule::set_int(boma, 1, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
                } else if ControlModule::get_stick_x(boma) > 0.707 && ice_usable {
                    WorkModule::set_int(boma, 3, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
                } else if ControlModule::get_stick_x(boma) < -0.707 && lightning_usable {
                    WorkModule::set_int(boma, 2, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
                } else if ControlModule::get_stick_y(boma) < -0.707 {
                    WorkModule::set_int(boma, 0, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), true.into());
                }
            }

            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                CancelModule::enable_cancel(boma);
            }
        }
    }
    
    0.into()
}

unsafe extern "C" fn marth_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_weiss(fighter.module_accessor) {
		return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
	}
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
		(*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 
		0, 
		0,
		0
	);
	return 0.into();
}

unsafe extern "C" fn marth_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_weiss(fighter.module_accessor) {
		return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
	}
    let boma = fighter.module_accessor;
	GroundModule::set_passable_check(boma, true);
    GroundModule::set_collidable(boma, true);
    JostleModule::set_status(boma, true);
    WorkModule::off_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_GLYPH_TELEPORT);
	0.into()
}

unsafe extern "C" fn marth_game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 3.0);
    frame(agent.lua_state_agent, 44.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn marth_game_speciallwhit(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 8.0, 361, 60, 0, 90, 5.5, 1.5, 0.0, 1.5, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 8.0, 361, 60, 0, 90, 5.5, 0.0, 1.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("claviclel"), 8.0, 361, 60, 0, 90, 5.5, 0.0, 1.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 8.0, 361, 60, 0, 90, 5.5, 1.5, 0.0, 6.5, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn marth_game_specialairlw(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_GLYPH_TELEPORT);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, 1, -5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::ATTACK(agent, 0, 0, Hash40::new("footl"), 10.0, 278, 75, 0, 25, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 10.0, 278, 75, 0, 25, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("footl"), 8.0, 80, 60, 0, 35, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("kneel"), 8.0, 80, 60, 0, 35, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

unsafe extern "C" fn marth_effect_specialairlw(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn marth_speciallw_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_weiss(fighter.module_accessor) {
		return smashline::original_status(Main, fighter, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT)(fighter);
	}
    let boma = fighter.module_accessor;
	if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(boma, Hash40::new("special_air_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(boma, Hash40::new("special_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
    }
    macros::EFFECT(fighter, Hash40::new("marth_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let motion_frame = MotionModule::frame(boma);
    let facing = PostureModule::lr(boma);
    let selected_dust = WorkModule::get_int(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
    let perc_select = WorkModule::get_float(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_PERCENTAGE_AT_SELECTION);
    if selected_dust == 1 { // Fire
        WorkModule::set_float(boma, 4.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_R);
		WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_G);
		WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_B);
        WorkModule::set_float(boma, DamageModule::damage(boma, 0), FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_PERCENTAGE_AT_SELECTION);
        WorkModule::off_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_DUST_FIRE_USABLE);
    } else if selected_dust == 3 { // Ice
        WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_R);
		WorkModule::set_float(boma, 4.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_G);
		WorkModule::set_float(boma, 4.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_B);
        WorkModule::set_float(boma, DamageModule::damage(boma, 0), FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_PERCENTAGE_AT_SELECTION);
        WorkModule::off_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_DUST_ICE_USABLE);
    } else if selected_dust == 2 { // Lightning
        WorkModule::set_float(boma, 4.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_R);
		WorkModule::set_float(boma, 4.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_G);
		WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_B);
        WorkModule::set_float(boma, DamageModule::damage(boma, 0), FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_PERCENTAGE_AT_SELECTION);
        WorkModule::off_flag(boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_DUST_LIGHTNING_USABLE);
    } else if selected_dust == 0 { // Normal
        WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_R);
		WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_G);
		WorkModule::set_float(boma, 0.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_DUST_EFFECT_B);
        WorkModule::set_float(boma, 999.0, FIGHTER_MARTH_INSTANCE_WORK_ID_FLOAT_PERCENTAGE_AT_SELECTION);
    }
    
    smashline::original_status(Main, fighter, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT)(fighter)
}

unsafe extern "C" fn marth_sound_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_wallhit"));
    }
}

unsafe extern "C" fn marth_sound_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_sword_swing_m"));
    }
}

pub fn install() {
    Agent::new("marth")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, marth_speciallw_main)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, marth_speciallw_pre)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, marth_speciallw_end)
        .game_acmd("game_speciallw_weiss", marth_game_speciallw, Default)
        .game_acmd("game_speciallwhit_weiss", marth_game_speciallwhit, Default)
        .game_acmd("game_specialairlw_weiss", marth_game_specialairlw, Default)
        .effect_acmd("effect_specialairlw_weiss", marth_effect_specialairlw, Default)
        .status(Main, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, marth_speciallw_hit_main)
        .sound_acmd("sound_speciallw_weiss", marth_sound_speciallw, Default)
        .sound_acmd("sound_specialairlw_weiss", marth_sound_specialairlw, Default)
        .install();
}
