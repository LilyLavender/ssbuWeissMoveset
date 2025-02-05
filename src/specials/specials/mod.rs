use super::*;

unsafe extern "C" fn marth_game_specials1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        if !helper::is_exist_glyph(agent.module_accessor) {
            ArticleModule::generate_article(agent.module_accessor, FIGHTER_MARTH_GENERATE_ARTICLE_ICESHARD, false, -1);
        } else {
            helper::change_status_glyph(agent.module_accessor, WEAPON_MARTH_ICESHARD_STATUS_KIND_MOVE);
        }
    }
}

pub fn install() {
    Agent::new("marth")
        .game_acmd("game_specials1_weiss", marth_game_specials1, Default)
        .game_acmd("game_specialairs1_weiss", marth_game_specials1, Default)
        .install();
}
