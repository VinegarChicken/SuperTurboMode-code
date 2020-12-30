use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_CAPTAIN,
animation = "attack_s4_s",
animcmd = "game_attacks4")]
pub fn instant_captain_attack_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(18.0)
    }
    frame(Frame=12)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=19)
    if(is_excute){
        MotionModule::set_rate(1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=19.0, Angle=43, KBG=94, FKB=0, BKB=36, Size=3.8, X=2.2, Y=0.7, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("shoulderl"), Damage=19.0, Angle=43, KBG=94, FKB=0, BKB=36, Size=3.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=4)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=65)
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_CAPTAIN,
animation = "special_n",
animcmd = "game_specialn")]
pub fn captain_n_special(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(50.0)
    }
    frame(Frame=15)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN)
    }
    frame(Frame=52)
    if(is_execute){
    MotionModule::set_rate(1.0)
    }
    frame(Frame=53)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD)
        ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=25.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=4.5, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("shoulderr"), Damage=25.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=2.5, X=-2.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=25.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=4.0, X=4.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=5)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_CAPTAIN,
animation = "special_air_n",
animcmd = "game_specialairn")]
pub fn captain_air_n_special(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(50.0)
    }
    frame(Frame=15)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN)
    }
    frame(Frame=50)
    if(is_execute){
    MotionModule::set_rate(1.0)
    }
    frame(Frame=51)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_DIR_DECIDE)
        WorkModule::set_int(1, FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE)
    }
    frame(Frame=53)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD)
        ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=22.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=5.175, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("shoulderr"), Damage=22.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=2.875, X=-2.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=22.0, Angle=361, KBG=59, FKB=0, BKB=93, Size=4.6, X=4.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=5)
    if(is_excute){
        AttackModule::clear_all()
    }
    wait(Frames=12)
    if(is_excute){
        WorkModule::set_int(2, FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE)
    }
    });



}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_CAPTAIN,
animation = "special_lw",
animcmd = "game_speciallw")]
pub fn captain_special_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=15.0, Angle=52, KBG=88, FKB=0, BKB=60, Size=3.8, X=10.0, Y=0.0, Z=0.0, X2=2.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_KICK)
        MotionModule::set_rate(17.0)
        JostleModule::set_status(false)
    }
    wait(Frames=1)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK)
    }
    frame(Frame=18)
    if(is_execute){
    MotionModule::set_rate(1.0)
    }
    frame(Frame=19)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=11.0, Angle=52, KBG=60, FKB=0, BKB=90, Size=3.2, X=10.0, Y=0.0, Z=0.0, X2=2.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=26)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=7.0, Angle=46, KBG=65, FKB=0, BKB=80, Size=3.2, X=10.0, Y=0.0, Z=0.0, X2=2.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=36)
    if(is_excute){
        AttackModule::clear_all()
        JostleModule::set_status(true)
    }
    });


}
pub fn install() {
    acmd::add_hooks!(
        instant_captain_attack_s4,
        captain_n_special,
        captain_special_lw,
        captain_air_n_special
        //instant_captain_attack_hi4
    );
}
