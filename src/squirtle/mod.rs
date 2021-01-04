use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon,L2CFighterBase};
use acmd::{acmd, acmd_func};

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_PZENIGAME,
animation = "special_n_shot",
animcmd = "game_specialnshot")]
pub fn squirtle_special_n(fighter: &mut L2CFighterCommon) {
    acmd!({
    frame(Frame=5)
    if(is_excute){
        CancelModule::enable_cancel();
        WorkModule::on_flag(Flag=FIGHTER_PZENIGAME_STATUS_SPECIAL_N_FLAG_SHOOT_ANGLE_ENABLE)
    }
    FT_MOTION_RATE(FSM=0.85)
    frame(Frame=7)
    for(6 Iterations){
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_PZENIGAME_GENERATE_ARTICLE_WATER,false,0)
        }
        wait(Frames=4)
    }
    frame(Frame=40)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_PZENIGAME_STATUS_SPECIAL_N_FLAG_SHOOT_ANGLE_ENABLE)
    }
    });


}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_PZENIGAME,
animation = "special_air_n_shot",
animcmd = "game_specialairnshot")]
pub fn squirtle_special_air_n_shot(fighter: &mut L2CFighterCommon) {
    acmd!({
    frame(Frame=5)
    if(is_excute){
        CancelModule::enable_cancel();
        WorkModule::on_flag(Flag=FIGHTER_PZENIGAME_STATUS_SPECIAL_N_FLAG_SHOOT_ANGLE_ENABLE)
    }
    FT_MOTION_RATE(FSM=0.85)
    frame(Frame=7)
    for(6 Iterations){
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_PZENIGAME_GENERATE_ARTICLE_WATER,false,0)
        }
        wait(Frames=4)
    }
    frame(Frame=40)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_PZENIGAME_STATUS_SPECIAL_N_FLAG_SHOOT_ANGLE_ENABLE)
    }
    });



}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON,
battle_object_kind = WEAPON_KIND_PZENIGAME_WATER,
animation = "regular",
animcmd = "game_regular")]
pub fn squirtle_water(fighter : &mut L2CFighterBase) {
    acmd!({
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=45, KBG=10000, FKB=10000, BKB=10000, Size=40.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
        AttackModule::enable_safe_pos()
    }
    });

}
pub fn install() {
    acmd::add_hooks!(
        squirtle_special_n,
        squirtle_special_air_n_shot,
        squirtle_water
    );
}
