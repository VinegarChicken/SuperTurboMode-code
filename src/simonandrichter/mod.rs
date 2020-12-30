use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SIMON,
animation = "special_n",
animcmd = "game_specialn")]
pub fn simon_special_n(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(29.0)
    }
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
    }
    frame(Frame=29)
    if(is_execute){
    MotionModule::set_rate(1.0)
    }
    frame(Frame=30)
    if(is_excute){
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=31)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=32)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=33)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
       ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=34)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=35)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=36)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=37)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=38)
    if(is_execute){
    CancelModule::enable_cancel()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SIMON,
animation = "special_air_n",
animcmd = "game_specialairn")]
pub fn simon_special_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(29.0)
    }
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
    }
    frame(Frame=29)
    if(is_execute){
    MotionModule::set_rate(1.0)
    }
    frame(Frame=30)
    if(is_excute){
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=31)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=32)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=33)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
       ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=34)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=35)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=36)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=37)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=38)
    if(is_execute){
    CancelModule::enable_cancel()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_RICHTER,
animation = "special_n",
animcmd = "game_specialn")]
pub fn richter_special_n(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(29.0)
    }
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
    }
    frame(Frame=29)
    if(is_execute){
    MotionModule::set_rate(1.0)
    }
    frame(Frame=30)
    if(is_excute){
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=31)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=32)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=33)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
       ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=34)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=35)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=36)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=37)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=38)
    if(is_execute){
    CancelModule::enable_cancel()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_RICHTER,
animation = "special_air_n",
animcmd = "game_specialairn")]
pub fn richter_special_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(29.0)
    }
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
    }
    frame(Frame=29)
    if(is_execute){
    MotionModule::set_rate(1.0)
    }
    frame(Frame=30)
    if(is_excute){
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=31)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=32)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=33)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
       ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=34)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=35)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=36)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=37)
    if(is_excute){
        ArticleModule::generate_article(FIGHTER_SIMON_GENERATE_ARTICLE_AXE,false,0)
        ArticleModule::shoot(FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
    }
    frame(Frame=38)
    if(is_execute){
    CancelModule::enable_cancel()
    }
    });

}
pub fn install() {
    acmd::add_hooks!(
        simon_special_n,
        simon_special_air_n,
        richter_special_n,
        richter_special_air_n
    );
}
