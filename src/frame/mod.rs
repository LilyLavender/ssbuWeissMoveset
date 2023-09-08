use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*,
	std::f32::consts::E
};
use smash::app::FighterManager;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;

static mut state: [i32; 8] = [0; 8];
static mut r: [f32; 8] = [0.0; 8];
static mut g: [f32; 8] = [0.0; 8];
static mut b: [f32; 8] = [0.0; 8];
static mut effect: [i32; 8] = [0; 8];
static mut percSelect: [f32; 8] = [999.0; 8];
static mut freezeTimer: [f32; 8] = [180.0; 8];
static mut timeDilationZoneX: [f32; 8] = [042105.0; 8];
static mut timeDilationZoneY: [f32; 8] = [042105.0; 8];
static mut timeDilationZoneTimer: [f32; 8] = [600.0; 8];

static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
const FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SEARCH_HIT : i32 = 0x200000eb;
const FIGHTER_INSTANCE_WORK_ID_FLAG_FROZEN : i32 = 496;
const FIGHTER_INSTANCE_WORK_ID_FLAG_UNFREEZE : i32 = 497;

#[fighter_frame(agent = FIGHTER_KIND_MARTH)]
fn marth_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		
		// Helper funct for Ice neutral B
		if MotionModule::motion_kind(fighter.module_accessor) != hash40("special_n") && MotionModule::motion_kind(fighter.module_accessor) != hash40("special_air_n") {
			WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
		}
		
		// Mode switching
		let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
		// This if checks for three different conditions: 1. frame 4 of a counter, 2. frames 3-4 of a parry, 3. frame 10 of down taunt in training mode
		if ((MotionModule::motion_kind(module_accessor) == hash40("special_lw_hit") || MotionModule::motion_kind(module_accessor) == hash40("special_air_lw_hit")) && MotionModule::frame(module_accessor) == 4.0) || (MotionModule::motion_kind(module_accessor) == hash40("just_shield_off") && (MotionModule::frame(module_accessor) >= 3.0 && MotionModule::frame(module_accessor) <= 4.0)) || (smashball::is_training_mode() && (MotionModule::motion_kind(module_accessor) == hash40("appeal_lw_l") || MotionModule::motion_kind(module_accessor) == hash40("appeal_lw_r")) && MotionModule::frame(module_accessor) == 10.0) {
			macros::EFFECT(fighter, Hash40::new("marth_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			if PostureModule::lr(fighter.module_accessor) == 1.0 {
				if ControlModule::get_stick_y(module_accessor) > 0.707 {
					state[entry_id] = 1; // Fire
					r[entry_id] = 4.0;
					g[entry_id] = 0.0;
					b[entry_id] = 0.0;
					percSelect[entry_id] = DamageModule::damage(fighter.module_accessor, 0);
				} else if ControlModule::get_stick_x(module_accessor) < -0.707 {
					state[entry_id] = 3; // Ice
					r[entry_id] = 0.0;
					g[entry_id] = 4.0;
					b[entry_id] = 4.0;
					percSelect[entry_id] = DamageModule::damage(fighter.module_accessor, 0);
				} else if ControlModule::get_stick_x(module_accessor) > 0.707 {
					state[entry_id] = 2; // Lightning
					r[entry_id] = 4.0;
					g[entry_id] = 4.0;
					b[entry_id] = 0.0;
					percSelect[entry_id] = DamageModule::damage(fighter.module_accessor, 0);
				} else if ControlModule::get_stick_y(module_accessor) < -0.707 {
					state[entry_id] = 0; // Normal
					r[entry_id] = 0.0;
					g[entry_id] = 0.0;
					b[entry_id] = 0.0;
					percSelect[entry_id] = 999.0;
				}
			} else {
				if ControlModule::get_stick_y(module_accessor) > 0.707 {
					state[entry_id] = 1; // Fire
					r[entry_id] = 4.0;
					g[entry_id] = 0.0;
					b[entry_id] = 0.0;
					percSelect[entry_id] = DamageModule::damage(fighter.module_accessor, 0);
				} else if ControlModule::get_stick_x(module_accessor) > 0.707 {
					state[entry_id] = 3; // Ice
					r[entry_id] = 0.0;
					g[entry_id] = 4.0;
					b[entry_id] = 4.0;
					percSelect[entry_id] = DamageModule::damage(fighter.module_accessor, 0);
				} else if ControlModule::get_stick_x(module_accessor) < -0.707 {
					state[entry_id] = 2; // Lightning
					r[entry_id] = 4.0;
					g[entry_id] = 4.0;
					b[entry_id] = 0.0;
					percSelect[entry_id] = DamageModule::damage(fighter.module_accessor, 0);
				} else if ControlModule::get_stick_y(module_accessor) < -0.707 {
					state[entry_id] = 0; // Normal
					r[entry_id] = 0.0;
					g[entry_id] = 0.0;
					b[entry_id] = 0.0;
					percSelect[entry_id] = 999.0;
				}
			}
		}
		if state[entry_id] == 1 { // Fire
			r[entry_id] = 4.0;
			g[entry_id] = 0.0;
			b[entry_id] = 0.0;
			AttackModule::set_power_up(module_accessor, 1.1);
			AttackModule::set_reaction_mul(module_accessor, 1.1);
			DamageModule::set_damage_mul(module_accessor, 1.3);
			DamageModule::set_reaction_mul(module_accessor, 1.5);
		}
		if state[entry_id] == 0 { // Normal
			r[entry_id] = 0.0;
			g[entry_id] = 0.0;
			b[entry_id] = 0.0;
			AttackModule::set_power_up(module_accessor, 1.0);
			AttackModule::set_reaction_mul(module_accessor, 1.0);
			DamageModule::set_damage_mul(module_accessor, 1.0);
			DamageModule::set_reaction_mul(module_accessor, 1.0);
		}
		if state[entry_id] == 3 { // Ice
			r[entry_id] = 0.0;
			g[entry_id] = 4.0;
			b[entry_id] = 4.0;
			smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 0.9);
			let lua_state = fighter.lua_state_agent;
			acmd!(lua_state, {
				sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.0)
			});
			AttackModule::set_power_up(module_accessor, 0.8);
			AttackModule::set_reaction_mul(module_accessor, 0.8);
			DamageModule::set_damage_mul(module_accessor, 0.7);
			DamageModule::set_reaction_mul(module_accessor, 0.7);
		}
		if state[entry_id] == 2 { // Lightning
			r[entry_id] = 4.0;
			g[entry_id] = 4.0;
			b[entry_id] = 0.0;
			smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 1.4);
			let lua_state = fighter.lua_state_agent;
			acmd!(lua_state, {
				sv_kinetic_energy::set_limit_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 3.2)
				sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.4)
			});
			AttackModule::set_power_up(module_accessor, 0.8);
			AttackModule::set_reaction_mul(module_accessor, 0.8);
			DamageModule::set_damage_mul(module_accessor, 1.0);
			DamageModule::set_reaction_mul(module_accessor, 1.0);
		}
		// If pecent when dust was selected is 30 less than percent now, remove dust
		if percSelect[entry_id] + 30.0 <= DamageModule::damage(fighter.module_accessor, 0) {
			state[entry_id] = 0;
			percSelect[entry_id] = 999.0;
			macros::EFFECT(fighter, Hash40::new("marth_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		// If player dies or training mode reset, remove dust without playing effect
		if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DEAD || sv_information::is_ready_go() == false {
			state[entry_id] = 0;
			percSelect[entry_id] = 999.0;
		}
		// Summon glow effect
		effect[entry_id] += 1;
		if effect[entry_id] == 5 || effect[entry_id] == 10 {
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
			macros::LAST_EFFECT_SET_COLOR(fighter, r[entry_id], g[entry_id], b[entry_id]);
		}
		if effect[entry_id] >= 20 {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
			macros::LAST_EFFECT_SET_COLOR(fighter, r[entry_id], g[entry_id], b[entry_id]);
			effect[entry_id] = 0;
		}
    }
}

#[acmd_script( agent = "marth", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn marth_game_attacks4(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let mut dirY = 0.0;
	let mut dirZ = 10.0;
	frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
		let mut collision_attr = "collision_attr_cutup";
		if state[entry_id] == 3 { // Ice
			collision_attr = "collision_attr_ice";
		}
		if state[entry_id] == 1 { // Fire
			macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 3.0, 13.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
			macros::ATTACK(agent, 4, 0, Hash40::new("top"), 18.0, 361, 85, 0, 58, 13.0, 0.0, 3.0, 13.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
		}
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 13.0, 361, 75, 0, 48, 3.5, 1.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(collision_attr), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 13.0, 361, 75, 0, 48, 3.0, 0.0, 1.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(collision_attr), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("bust"), 13.0, 361, 75, 0, 48, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(collision_attr), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 18.0, 361, 80, 0, 80, 3.5, 1.0, 0.0, 7.3, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(collision_attr), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 14.0);
	if state[entry_id] == 2 { // Lightning
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

#[acmd_script( agent = "marth", scripts = [ "game_specialn", "game_specialairn" ], category = ACMD_GAME, low_priority )]
unsafe fn marth_game_specialn(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if state[entry_id] == 0 { // Normal
		frame(agent.lua_state_agent, 9.0);
		for _ in 0..7 {
			if macros::is_excute(agent) {
				macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.8, 367, 25, 0, 35, 13.0, 0.0, 12.0, 4.0, Some(0.0), Some(12.0), Some(-4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
				macros::EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
				macros::EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
			}
			wait(agent.lua_state_agent, 3.0);
			if macros::is_excute(agent) {
				AttackModule::clear_all(agent.module_accessor);
			}
		}
	} else if state[entry_id] == 1 { // Fire
		macros::FT_MOTION_RATE(agent, 8.0);
		frame(agent.lua_state_agent, 5.0);
		macros::FT_MOTION_RATE(agent, 1.0);
		frame(agent.lua_state_agent, 9.0);
		if macros::is_excute(agent) {
			macros::EFFECT(agent, Hash40::new("sys_bomb_c"), Hash40::new("top"), 0.0, 16.0, 0.0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
			macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
			macros::ATTACK(agent, 0, 0, Hash40::new("top"), 24.0, 361, 85, 0, 58, 26.0, 0.0, 16.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
			state[entry_id] = 0;
			percSelect[entry_id] = 999.0;
			macros::EFFECT(agent, Hash40::new("marth_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		frame(agent.lua_state_agent, 12.0);
		if macros::is_excute(agent) {
			AttackModule::clear_all(agent.module_accessor);
			DamageModule::add_damage(agent.module_accessor, 6.0, 0);
			StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_STATUS_KIND_FURAFURA.into(), false.into());
		}
	} else if state[entry_id] == 2 { // Lighting
		frame(agent.lua_state_agent, 9.0);
		if macros::is_excute(agent) {
			timeDilationZoneX[entry_id] = PostureModule::pos_x(agent.module_accessor);
			timeDilationZoneY[entry_id] = PostureModule::pos_y(agent.module_accessor);
			macros::EFFECT(agent, Hash40::new("sys_shield_damage2"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 7.0, 0, 0, 0, 0, 0, 0, true);
			macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.0);
			macros::LAST_EFFECT_SET_ALPHA(agent, 0.4);
			macros::PLAY_SE(agent, Hash40::new("se_common_elec_l_damage"));
			timeDilationZoneTimer[entry_id] = 600.0;
			state[entry_id] = 0;
			percSelect[entry_id] = 999.0;
			macros::EFFECT(agent, Hash40::new("marth_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
	} else { // Ice
		frame(agent.lua_state_agent, 9.0);
		if macros::is_excute(agent) {
			WorkModule::on_flag(agent.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
			macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 60.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_NONE"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
			macros::EFFECT(agent, Hash40::new("sys_shield_damage2"), Hash40::new("top"), 0, 30.0, 0, 0, 0, 0, 6.0, 0, 0, 0, 0, 0, 0, true);
			macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.8, 1.0);
			macros::LAST_EFFECT_SET_ALPHA(agent, 0.6);
			macros::PLAY_SE(agent, Hash40::new("se_common_frieze_l"));
		}
		frame(agent.lua_state_agent, 12.0);
		if macros::is_excute(agent) {
			AttackModule::clear_all(agent.module_accessor);
			macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_shield_damage2"), false, false);
			state[entry_id] = 0;
			percSelect[entry_id] = 999.0;
			macros::EFFECT(agent, Hash40::new("marth_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			WorkModule::off_flag(agent.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
		}
	}
}

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool, fighter: &mut L2CAgentBase) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let defender_boma = sv_battle_object::module_accessor(defender_id);
	let attacker_kind = sv_battle_object::kind(attacker_id);
	let def_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	// Detect when ice neutral b searchbox lands
    if attacker_kind == *FIGHTER_KIND_MARTH {
		if WorkModule::is_flag(attacker_boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SEARCH_HIT) {
			WorkModule::off_flag(attacker_boma, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
			WorkModule::on_flag(defender_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FROZEN);
			freezeTimer[def_entry_id] = 180.0;
		}
	}
	// Detect when a frozen player is hit
	if WorkModule::is_flag(defender_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FROZEN) && freezeTimer[def_entry_id] != 180.0 {
		WorkModule::off_flag(defender_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FROZEN);
		WorkModule::on_flag(defender_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_UNFREEZE);
    }
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again, fighter)
}

#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		
		// Ice Neutral B freezing
		if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FROZEN) {
			// Summon blue flash on frozen player
			if freezeTimer[entry_id] > 60.0 {
				macros::FLASH(fighter, 0.0, 0.8, 1.0, 0.8);
			} else {
				if freezeTimer[entry_id] % 12.0 >= 5.0 {
					macros::FLASH(fighter, 0.0, 0.8, 1.0, 0.8);
				} else {
					macros::COL_NORMAL(fighter);
				}
			}
			// Freeze player
			lua_bind::FighterManager::set_position_lock(singletons::FighterManager(), smash::app::FighterEntryID(entry_id as i32), true);
			MotionModule::set_rate(fighter.module_accessor, 0.0);
			AttackModule::clear_all(fighter.module_accessor);
			ControlModule::clear_command(fighter.module_accessor, true);
			ControlModule::reset_button(fighter.module_accessor);
			// Tick timer down
			freezeTimer[entry_id] = freezeTimer[entry_id] - 1.0;
			// Unfreeze due to timer
			if freezeTimer[entry_id] <= 0.0 {
				lua_bind::FighterManager::set_position_lock(singletons::FighterManager(), smash::app::FighterEntryID(entry_id as i32), false);
				MotionModule::set_rate(fighter.module_accessor, 1.0);
				macros::COL_NORMAL(fighter);
				WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FROZEN);
			}
		}
		// Unfreeze due to being hit
		if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_UNFREEZE) {
			lua_bind::FighterManager::set_position_lock(singletons::FighterManager(), smash::app::FighterEntryID(entry_id as i32), false);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::COL_NORMAL(fighter);
			WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FROZEN);
			WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_UNFREEZE);
		}
		
		// Time Dilation Zone
		let mut w_entry_id = 0;
		if timeDilationZoneX[entry_id] != 042105.0 {
			// Tick timer
			timeDilationZoneTimer[entry_id] = timeDilationZoneTimer[entry_id] - 1.0;
			// Speed up Weiss
			let b1x = PostureModule::pos_x(fighter.module_accessor);
			let b1y = PostureModule::pos_y(fighter.module_accessor);
			let dSquared: f32 = (b1x - timeDilationZoneX[entry_id]).powf(2.0) + (b1y - timeDilationZoneY[entry_id]).powf(2.0);
			let d = dSquared.sqrt();
			if d <= 70.0 {
				MotionModule::set_rate(fighter.module_accessor, 2.0);
			}
			// Slow down opponents helper
			let w_entry_id = entry_id;
		} 
		// Slow down opponents
		if entry_id != w_entry_id {
			let b1x = PostureModule::pos_x(fighter.module_accessor);
			let b1y = PostureModule::pos_y(fighter.module_accessor);
			let dSquared: f32 = (b1x - timeDilationZoneX[w_entry_id]).powf(2.0) + (b1y - timeDilationZoneY[w_entry_id]).powf(2.0);
			let d = dSquared.sqrt();
			if d <= 70.0 {
				MotionModule::set_rate(fighter.module_accessor, 0.5);
			}
		}
		// Remove time dilation zone once 10 seconds have passed
		if timeDilationZoneTimer[entry_id] <= 0.0 {
			timeDilationZoneX[entry_id] = 042105.0;
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_shield_damage2"), false, false);
		}
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];

pub fn install() {
	unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
	skyline::install_hook!(
		notify_log_event_collision_hit_replace
	);
    smashline::install_agent_frames!(
        marth_frame,
    );
	smashline::install_agent_frame_callbacks!(
		global_fighter_frame
    );
	smashline::install_acmd_scripts!(
        marth_game_attacks4,
		marth_game_specialn,
    );
}
