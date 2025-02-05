use super::*;

unsafe extern "C" fn marth_iceshard_move_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
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

unsafe extern "C" fn marth_iceshard_move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    // Get closest player boma
    let d_boma = get_boma_of_closest_player_front_close((*(boma)).battle_object_id as usize);
    
    // No close players, kill
    if d_boma.is_null() {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 1.into();
    }

    // Get positions
	let w_pos_x = PostureModule::pos_x(boma);
	let w_pos_y = PostureModule::pos_y(boma);
	let d_pos_x = PostureModule::pos_x(d_boma);
	let d_pox_y = PostureModule::pos_y(d_boma) + (WorkModule::get_param_float(d_boma, hash40("height"), 0) / 2.0);
	
	// Compute x and y components to be added to new speed
	let c = ((w_pos_x - d_pos_x).powf(2.0) + (w_pos_y - d_pox_y).powf(2.0)).powf(0.5);
	let speed_x = (d_pos_x - w_pos_x) / c * 4.0;
	let speed_y = (d_pox_y - w_pos_y) / c * 4.0;

    // Set speed
	weapon.clear_lua_stack();
	weapon.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
	weapon.push_lua_stack(&mut L2CValue::new_num(speed_x));
	weapon.push_lua_stack(&mut L2CValue::new_num(speed_y));
	sv_kinetic_energy::set_speed(weapon.lua_state_agent);
    let rot_x = speed_y.atan2(speed_x).to_degrees();
    PostureModule::set_rot(boma, &Vector3f{ x: rot_x, y: 0.0, z: 0.0 }, 0);

    WorkModule::set_int(boma, 60, WEAPON_MARTH_ICESHARD_INSTANCE_WORK_ID_INT_LIFE);
	MotionModule::change_motion(boma, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);

    weapon.global_table[0x15].assign(&L2CValue::Ptr(marth_iceshard_move_substatus as *const () as _));
	weapon.fastshift(L2CValue::Ptr(marth_iceshard_move_main_loop as *const () as _))
}

unsafe extern "C" fn marth_iceshard_move_substatus(weapon: &mut L2CWeaponCommon) {
    let is_stop = StopModule::is_stop(weapon.module_accessor);
    if !is_stop {
        WorkModule::dec_int(weapon.module_accessor, WEAPON_MARTH_ICESHARD_INSTANCE_WORK_ID_INT_LIFE);
    }
}

unsafe extern "C" fn marth_iceshard_move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, WEAPON_MARTH_ICESHARD_INSTANCE_WORK_ID_INT_LIFE);
    let is_touch = GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32);
    if life <= 0 || is_touch {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

// Helper function to 🫠
unsafe extern "C" fn get_boma_of_closest_player_front_close(actor_id: usize) -> *mut smash::app::BattleObjectModuleAccessor {
    // Decs
    struct BomaData {
        boma: *mut smash::app::BattleObjectModuleAccessor,
        x_pos: f32,
        y_pos: f32,
        distance: f32,
    }
    let fighter_num = lua_bind::FighterManager::total_fighter_num(singletons::FighterManager());
    let mut bomas: Vec<BomaData> = Vec::with_capacity(fighter_num as usize);

    // Init
    for i in 0..fighter_num {
        let boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i));
        let x_pos = PostureModule::pos_x(boma);
        let y_pos = PostureModule::pos_y(boma);
        bomas.push(BomaData {
            boma,
            x_pos,
            y_pos,
            distance: -1.0,
        });
    }

    // Get actor info
    let actor_boma = sv_battle_object::module_accessor(actor_id as u32);
    let actor_x = PostureModule::pos_x(actor_boma);
    let actor_y = PostureModule::pos_y(actor_boma);
    let facing = PostureModule::lr(actor_boma);

    // Calculate distances
    for (i, curr) in bomas.iter_mut().enumerate() {
        if i == actor_id
        || (utility::get_category(&mut *actor_boma) == *BATTLE_OBJECT_CATEGORY_WEAPON // Ignore actor's owner
        && i == WorkModule::get_int(actor_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as usize) { 
            curr.distance = -1.0;
            continue;
        }
        curr.distance = ((curr.x_pos - actor_x).powi(2) + (curr.y_pos - actor_y).powi(2)).sqrt();
    }

    // Return closest
    let closest = bomas
        .iter()
        .filter(|d| {
            d.distance >= 0.0 // Not actor
            && d.distance <= 100.0 // Close to actor
            && ((facing == 1.0 && d.x_pos > actor_x) || (facing == -1.0 && d.x_pos < actor_x)) // In front of actor
        })
        .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
		.map(|fighter| fighter.boma)
		.unwrap_or(std::ptr::null_mut());
    closest
}

unsafe extern "C" fn marth_iceshard_move_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("weiss_ice_shard"), true, true);
	return 0.into();
}

unsafe extern "C" fn marth_iceshard_game_move(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
		macros::ATTACK_FP(agent, 0, 0, Hash40::new("top"), 8.0, 60, 40, 0, 20, 2.0, 0.0, 0.0, 0.0, Hash40::new("collision_attr_normal"), 0.0, 1.0, 1.0, false, false, 0, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *COLLISION_SITUATION_MASK_GA, false, *ATTACK_REGION_NONE, *COLLISION_CATEGORY_MASK_ALL, false, *COLLISION_PART_MASK_ALL, false, true, true, true, 0, false, false, *ATTACK_LR_CHECK_SPEED, false, false, false, false, false, *COLLISION_SHAPE_TYPE_SPHERE);
	}
}

unsafe extern "C" fn marth_iceshard_effect_move(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("weiss_ice_shard"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.9);
	}
}

pub fn install() {
    Agent::new("marth_iceshard")
        .status(Pre, WEAPON_MARTH_ICESHARD_STATUS_KIND_MOVE, marth_iceshard_move_pre)
        .status(Main, WEAPON_MARTH_ICESHARD_STATUS_KIND_MOVE, marth_iceshard_move_main)
        .status(End, WEAPON_MARTH_ICESHARD_STATUS_KIND_MOVE, marth_iceshard_move_end)
        .game_acmd("game_move", marth_iceshard_game_move, Default)
        .effect_acmd("effect_move", marth_iceshard_effect_move, Default)
        .install();
}
