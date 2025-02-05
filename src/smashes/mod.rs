use super::*;

// Attacks4
unsafe extern "C" fn marth_game_attacks4(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let selected_dust = WorkModule::get_int(agent.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_SELECTED_DUST_TYPE);
	let mut dirY = 0.0;
	let mut dirZ = 10.0;
	frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
		if selected_dust == 1 { // Fire
			macros::ATTACK(agent, 4, 1, Hash40::new("top"), 7.0, 361, 40, 40, 0, 13.0, 0.0, 3.0, 13.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
			macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 3.0, 13.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
		}
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
		let mut collision_attr = "collision_attr_cutup";
		if selected_dust == 3 { // Ice
			collision_attr = "collision_attr_ice";
		}
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 13.0, 361, 75, 0, 48, 3.5, 1.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(collision_attr), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 13.0, 361, 75, 0, 48, 3.0, 0.0, 1.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(collision_attr), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("bust"), 13.0, 361, 75, 0, 48, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(collision_attr), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 18.0, 361, 80, 0, 80, 3.5, 1.0, 0.0, 7.3, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(collision_attr), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
		if selected_dust == 1 { // Fire
			AttackModule::clear(agent.module_accessor, 4, false);
        }
    }
    frame(agent.lua_state_agent, 14.0);
	if selected_dust == 2 { // Lightning
		for _ in 0..8 {
			if macros::is_excute(agent) {
				dirY = dirY + 6.0 * ControlModule::get_stick_y(agent.module_accessor);
				dirZ = dirZ + 8.0; 
				macros::ATTACK(agent, 4, 0, Hash40::new("top"), 2.1, 361, 25, 0, 35, 10.0, 0.0, dirY, dirZ, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_electric"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
				macros::EFFECT(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0.0, dirY, dirZ, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
			}
			wait(agent.lua_state_agent, 3.0);
			if macros::is_excute(agent) {
				AttackModule::clear_all(agent.module_accessor);
			}
		}
	} else {
		if macros::is_excute(agent) {
			AttackModule::clear_all(agent.module_accessor);
		}
	}
}

// Attackhi4
static mut icePosZ: [f32; 8] = [50.0; 8];
static mut iceSize: [f32; 8] = [1.2; 8];
static mut icePosYmult: [f32; 8] = [0.5; 8];
static mut iceHitboxSizeMult: [f32; 8] = [0.8; 8];

unsafe extern "C" fn marth_game_attackhi4(agent: &mut L2CAgentBase) {
	let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) {
		icePosZ[entry_id] = 50.0;
		iceSize[entry_id] = 1.2;
		icePosYmult[entry_id] = 0.5;
		iceHitboxSizeMult[entry_id] = 0.8;
    }
	frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        let facing = PostureModule::lr(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 98, 85, 0, 30, 7.5 * iceHitboxSizeMult[entry_id], 5.0, -2.0 * icePosYmult[entry_id], icePosZ[entry_id], Some(-5.0), Some(-2.0 * icePosYmult[entry_id]), Some(icePosZ[entry_id]), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 82, 85, 0, 30, 7.5 * iceHitboxSizeMult[entry_id], 5.0, -2.0 * icePosYmult[entry_id], -icePosZ[entry_id], Some(-5.0), Some(-2.0 * icePosYmult[entry_id]), Some(-icePosZ[entry_id]), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
		macros::EFFECT(agent, Hash40::new("weiss_ice_sword_1_l"), Hash40::new("top"), 0, 6.0, -icePosZ[entry_id] * facing, 0.0, 0.0, 0.0, iceSize[entry_id], 0, 0, 0, 0, 0, 0, true);
		macros::EFFECT(agent, Hash40::new("weiss_ice_sword_1_r"), Hash40::new("top"), 0, 6.0, icePosZ[entry_id] * facing, 0.0, 0.0, 0.0, iceSize[entry_id], 0, 0, 0, 0, 0, 0, true);
    }
	frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 98, 85, 0, 30, 7.5 * iceHitboxSizeMult[entry_id], 5.0, 1.0 * icePosYmult[entry_id], icePosZ[entry_id], Some(-5.0), Some(1.0 * icePosYmult[entry_id]), Some(icePosZ[entry_id]), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 82, 85, 0, 30, 7.5 * iceHitboxSizeMult[entry_id], 5.0, 1.0 * icePosYmult[entry_id], -icePosZ[entry_id], Some(-5.0), Some(1.0 * icePosYmult[entry_id]), Some(-icePosZ[entry_id]), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
	}
	frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 98, 85, 0, 30, 7.5 * iceHitboxSizeMult[entry_id], 5.0, 4.0 * icePosYmult[entry_id], icePosZ[entry_id], Some(-5.0), Some(4.0 * icePosYmult[entry_id]), Some(icePosZ[entry_id]), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 82, 85, 0, 30, 7.5 * iceHitboxSizeMult[entry_id], 5.0, 4.0 * icePosYmult[entry_id], -icePosZ[entry_id], Some(-5.0), Some(4.0 * icePosYmult[entry_id]), Some(-icePosZ[entry_id]), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
	}
	frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 98, 85, 0, 30, 7.5 * iceHitboxSizeMult[entry_id], 5.0, 6.0 * icePosYmult[entry_id], icePosZ[entry_id], Some(-5.0), Some(6.0 * icePosYmult[entry_id]), Some(icePosZ[entry_id]), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 82, 85, 0, 30, 7.5 * iceHitboxSizeMult[entry_id], 5.0, 6.0 * icePosYmult[entry_id], -icePosZ[entry_id], Some(-5.0), Some(6.0 * icePosYmult[entry_id]), Some(-icePosZ[entry_id]), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
	}
	frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 98, 85, 0, 30, 7.5 * iceHitboxSizeMult[entry_id], 5.0, 10.0 * icePosYmult[entry_id], icePosZ[entry_id], Some(-5.0), Some(10.0 * icePosYmult[entry_id]), Some(icePosZ[entry_id]), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 82, 85, 0, 30, 7.5 * iceHitboxSizeMult[entry_id], 5.0, 10.0 * icePosYmult[entry_id], -icePosZ[entry_id], Some(-5.0), Some(10.0 * icePosYmult[entry_id]), Some(-icePosZ[entry_id]), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_KICK);
	}
	frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn marth_game_attackhi4charge(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	frame(agent.lua_state_agent, 1.0);
	for _ in 0..59 {
		if macros::is_excute(agent) {
			icePosZ[entry_id] = icePosZ[entry_id] - 0.5;
			iceSize[entry_id] = iceSize[entry_id] + 0.01;
			icePosYmult[entry_id] = icePosYmult[entry_id] + 1.0 / 120.0;
			iceHitboxSizeMult[entry_id] = iceHitboxSizeMult[entry_id] + 0.02;
		}
		wait(agent.lua_state_agent, 1.0);
	}
}

unsafe extern "C" fn marth_sound_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_marth_attack06"));
        macros::PLAY_SE(agent, Hash40::new("se_marth_smash_h01"));
    }
}

// Attacklw4
unsafe extern "C" fn marth_game_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 32, 93, 0, 30, 2.0, 0.0, 1.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 32, 93, 0, 30, 2.0, 1.0, 4.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 12.0, 32, 93, 0, 30, 2.5, -1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("armr"), 12.0, 32, 93, 0, 30, 2.5, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 30, 93, 0, 35, 2.0, 0.0, 1.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 30, 93, 0, 35, 2.0, 1.0, 4.5, 0.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 12.0, 30, 93, 0, 35, 2.5, -1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("armr"), 12.0, 30, 93, 0, 35, 2.5, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn marth_effect_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("knife"), 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.7);
    }
	frame(agent.lua_state_agent, 11.0);
	if macros::is_excute(agent) {
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_marth_sword1"), Hash40::new("tex_marth_sword2"), 10, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2)
	}
	frame(agent.lua_state_agent, 22.0);
	if macros::is_excute(agent) {
		macros::AFTER_IMAGE_OFF(agent, 0);
	}
}

unsafe extern "C" fn marth_effect_attacklw4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, true);
    }
    wait(agent.lua_state_agent, 5.0);
    macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("knife"), 0, 0, 3, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, false);
}

unsafe extern "C" fn marth_sound_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_marth_attack07"));
        macros::PLAY_SE(agent, Hash40::new("se_marth_smash_l01"));
    }
    wait(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_smash_l01"));
    }
}

pub fn install() {
    Agent::new("marth")
        .game_acmd("game_attacks4_weiss", marth_game_attacks4, Default)
        .game_acmd("game_attackhi4_weiss", marth_game_attackhi4, Default)
        .game_acmd("game_attackhi4charge_weiss", marth_game_attackhi4charge, Default)
        .sound_acmd("sound_attackhi4_weiss", marth_sound_attackhi4, Default)
        .game_acmd("game_attacklw4_weiss", marth_game_attacklw4, Default)
        .effect_acmd("effect_attacklw4_weiss", marth_effect_attacklw4, Default)
        .effect_acmd("effect_attacklw4charge_weiss", marth_effect_attacklw4charge, Default)
        .sound_acmd("sound_attacklw4_weiss", marth_sound_attacklw4, Default)
        .install();
}
