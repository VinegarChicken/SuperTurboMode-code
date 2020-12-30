use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};


#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DIDDY,
animation = "special_lw",
animcmd = "game_speciallw")]
pub fn diddy_special_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
    frame(Frame=3)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA,false,0)
    }
    frame(Frame=14)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_DIDDY_STATUS_SPECIAL_LW_FLAG_ITEM_THROW)
    }

    frame(Frame=20)
    for(30 Iterations){
    if(is_excute){
        ArticleModule::shoot(FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
    }
    }

    });



}
pub fn install() {
    acmd::add_hooks!(
        diddy_special_lw
        //instant_captain_attack_hi4
    );
}
