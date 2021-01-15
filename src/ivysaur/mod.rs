use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};


#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PFUSHIGISOU,
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn ivy_dair(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(10.0)
    }
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=10)
    if(is_execute){
    MotionModule::set_rate(1.0)
    }
    frame(Frame=11)
    if(is_excute){
        AttackModule::clear_all()
        KineticModule::get_sum_speed_y(KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
    }
    KineticModule::suspend_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=270, KBG=80, FKB=0, BKB=27, Size=4.0, X=0.0, Y=-13.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=270, KBG=62, FKB=0, BKB=17, Size=13.0, X=0.0, Y=-14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=270, KBG=98, FKB=0, BKB=9, Size=4.0, X=0.0, Y=-13.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
    ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=270, KBG=62, FKB=0, BKB=8, Size=13.0, X=0.0, Y=-14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        KineticModule::resume_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    }
    frame(Frame=52)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_PFUSHIGISOU,
animation = "special_hi",
animcmd = "game_specialhi")]
pub fn ivy_up_b(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(13.0)
    }
    frame(Frame=14)
    if(is_execute){
    MotionModule::set_rate(1.0)
    }
    frame(Frame=15)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_HI_FLAG_SET_ANGLE)
        ATTACK(ID=0, Part=0, Bone=hash40("viner2"), Damage=15.0, Angle=50, KBG=95, FKB=0, BKB=85, Size=4.5, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_OBJECT)
        ATTACK(ID=1, Part=0, Bone=hash40("viner3"), Damage=15.0, Angle=50, KBG=95, FKB=0, BKB=85, Size=4.2, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_OBJECT)
        ATTACK(ID=2, Part=0, Bone=hash40("viner4"), Damage=15.0, Angle=50, KBG=95, FKB=0, BKB=85, Size=3.8, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_OBJECT)
        ATTACK(ID=3, Part=0, Bone=hash40("viner5"), Damage=15.0, Angle=50, KBG=95, FKB=0, BKB=85, Size=3.5, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_OBJECT)
        ATTACK(ID=4, Part=0, Bone=hash40("viner5"), Damage=15.0, Angle=50, KBG=95, FKB=0, BKB=85, Size=4.5, X=8.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_OBJECT)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_PFUSHIGISOU,
animation = "special_air_hi",
animcmd = "game_specialairhi")]
pub fn ivy_air_up_b(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
        MotionModule::set_rate(13.0)
        ArticleModule::generate_article(FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_VINE,false,0)
        ArticleModule::set_visibility_whole(FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_VINE, false,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        WorkModule::on_flag(Flag=FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_HI_SET_MAP_COLL_OFFSET)
    }
    frame(Frame=13)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)
    }
    frame(Frame=14)
    if(is_excute){
        MotionModule::set_rate(1.0)
        WorkModule::off_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)
    }
    frame(Frame=15)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_HI_FLAG_SET_ANGLE)
        ATTACK(ID=0, Part=0, Bone=hash40("viner2"), Damage=15.0, Angle=50, KBG=95, FKB=0, BKB=85, Size=5.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_OBJECT)
        ATTACK(ID=1, Part=0, Bone=hash40("viner3"), Damage=15.0, Angle=50, KBG=95, FKB=0, BKB=85, Size=4.5, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_OBJECT)
        ATTACK(ID=2, Part=0, Bone=hash40("viner4"), Damage=15.0, Angle=50, KBG=95, FKB=0, BKB=85, Size=4.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_OBJECT)
        ATTACK(ID=3, Part=0, Bone=hash40("viner5"), Damage=15.0, Angle=50, KBG=95, FKB=0, BKB=85, Size=3.5, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_OBJECT)
        ATTACK(ID=4, Part=0, Bone=hash40("viner5"), Damage=15.0, Angle=50, KBG=95, FKB=0, BKB=85, Size=5.5, X=8.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_OBJECT)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
        MotionModule::set_rate(5.0)
    }
    frame(Frame=24)
    if(is_excute){
        MotionModule::set_rate(1.0)
    }
    frame(Frame=25)
    if(is_excute){
        sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
    }
    });


}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_PFUSHIGISOU,
animation = "attack_hi4",
animcmd = "game_attackhi4")]
pub fn instant_ivy_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(21.0)
    }
    frame(Frame=8)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=22)
    if(is_execute){
    MotionModule::set_rate(1.0)
    }
    frame(Frame=26)
    if(is_excute){
        HIT_NODE(hash40("flower"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=82, KBG=78, FKB=0, BKB=78, Size=40.0, X=0.0, Y=17.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=24.0, Angle=82, KBG=78, FKB=0, BKB=78, Size=25.0, X=0.0, Y=5.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
    }
    wait(Frames=4)
    if(is_excute){
        AttackModule::clear_all()
        HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
    }
    });

}

pub fn install() {
    acmd::add_hooks!(
        ivy_dair,
        ivy_up_b,
        ivy_air_up_b,
        instant_ivy_attack_hi4
    );
}
