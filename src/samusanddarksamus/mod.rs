use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon,L2CFighterBase};
use acmd::{acmd, acmd_func};

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SAMUS,
animation = "special_n_s",
animcmd = "game_specialnstart")]
pub fn samus_special_n(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    ArticleModule::generate_article(FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT,false,0);
    ArticleModule::shoot(FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
    WorkModule::on_flag(Flag=FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT)
    }
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SAMUS,
animation = "special_n_h",
animcmd = "game_specialnhold")]
pub fn samus_special_hold(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    ArticleModule::generate_article(FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT,false,0);
    ArticleModule::shoot(FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
    WorkModule::on_flag(Flag=FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT)
    }
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SAMUS,
animation = "special_n_fire",
animcmd = "game_specialnfire")]
pub fn samus_special_fire(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    ArticleModule::generate_article(FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT,false,0);
    ArticleModule::shoot(FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
    WorkModule::on_flag(Flag=FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT)
    }
    });
}
pub fn install() {
    acmd::add_hooks!(
        samus_special_fire,
        samus_special_hold,
        samus_special_n

    );
}
