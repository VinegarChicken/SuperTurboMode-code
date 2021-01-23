use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPA,
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn bowser_fair(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_rate(10.0)
    }
    frame(Frame=4)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=11)
    if(is_excute){
        MotionModule::set_rate(1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=80, FKB=0, BKB=40, Size=6.0, X=4.4, Y=0.0, Z=-0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=12.0, Angle=361, KBG=80, FKB=0, BKB=40, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("colonells"), Damage=11.0, Angle=361, KBG=80, FKB=0, BKB=40, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=4)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=31)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_air_b",
animcmd = "game_attackairb")]
pub fn bowser_bair(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_rate(8.0)
    }
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=9)
    if(is_excute){
        MotionModule::set_rate(1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=19.0, Angle=35, KBG=88, FKB=0, BKB=20, Size=7.0, X=0.0, Y=8.0, Z=-13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=12)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=31)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_air_hi",
animcmd = "game_attackairhi")]
pub fn bowser_uair(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_rate(8.0)
    }
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        HIT_NODE(hash40("head"), HIT_STATUS_XLU)
        HIT_NODE(hash40("mouth2"), HIT_STATUS_XLU)
    }
    frame(Frame=9)
    if(is_excute){
        MotionModule::set_rate(1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=15.0, Angle=85, KBG=85, FKB=0, BKB=35, Size=6.0, X=2.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
    }
    wait(Frames=5)
    if(is_excute){
        AttackModule::clear_all()
        //HitModule::set_status_all(HIT_STATUS_NORMAL)
    }
    frame(Frame=40)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });

}
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KOOPA,
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn bowser_dair(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_rate(16.0)
    }
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0, 3, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
    }
    frame(Frame=14)
    if(is_excute){
        HIT_NODE(hash40("head"), HIT_STATUS_XLU)
        HIT_NODE(hash40("mouth2"), HIT_STATUS_XLU)
        HIT_NODE(hash40("shoulderr"), HIT_STATUS_XLU)
        HIT_NODE(hash40("shoulderl"), HIT_STATUS_XLU)
        HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
        HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
        HIT_NODE(hash40("legr"), HIT_STATUS_XLU)
        HIT_NODE(hash40("legl"), HIT_STATUS_XLU)
        HIT_NODE(hash40("footr"), HIT_STATUS_XLU)
        HIT_NODE(hash40("footl"), HIT_STATUS_XLU)
    }
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=17)
    if(is_excute){
        MotionModule::set_rate(1.0)
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0, -4, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=275, KBG=100, FKB=0, BKB=30, Size=7.0, X=0.0, Y=-3.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        MotionModule::set_rate(7.0)
    }
    wait(Frames=8)
    if(is_excute){
        MotionModule::set_rate(1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=56, KBG=100, FKB=0, BKB=30, Size=7.0, X=0.0, Y=-3.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
    }
    frame(Frame=50)
    if(is_excute){
        AttackModule::clear_all()
    }
    /*
    frame(Frame=61)
    if(is_excute){
        HitModule::set_status_all(HIT_STATUS_NORMAL)
    }
     */
    frame(Frame=70)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });


}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_s4_s",
animcmd = "game_attacks4")]
pub fn bowser_f_smash(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_rate(7.0)
    }
    frame(Frame=4)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=17)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10)
    }
    frame(Frame=20)
    if(is_excute){
        HIT_NODE(hash40("footr"), HIT_STATUS_INVINCIBLE)
        HIT_NODE(hash40("footl"), HIT_STATUS_INVINCIBLE)
    }
    frame(Frame=22)
    if(is_excute){
        MotionModule::set_rate(1.0)
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
        ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=23.0, Angle=361, KBG=98, FKB=0, BKB=25, Size=7.0, X=-10.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=20.0, Angle=361, KBG=98, FKB=0, BKB=25, Size=5.5, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=24)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=17.0, Angle=361, KBG=103, FKB=0, BKB=10, Size=6.0, X=-9.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=14.0, Angle=361, KBG=103, FKB=0, BKB=10, Size=5.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=26)
    if(is_excute){
        //HitModule::set_status_all(HIT_STATUS_NORMAL)
    }
    frame(Frame=28)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_hi4",
animcmd = "game_attackhi4")]
pub fn bowser_u_smash(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_rate(15.0)
    }
    frame(Frame=10)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=11)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8)
    }
    frame(Frame=14)
    if(is_excute){
        HIT_NODE(hash40("bust"), HIT_STATUS_INVINCIBLE)
        HIT_NODE(hash40("hip"), HIT_STATUS_INVINCIBLE)
    }
    frame(Frame=16)
    if(is_excute){
        MotionModule::set_rate(1.0)
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
        ATTACK(ID=0, Part=0, Bone=hash40("rot"), Damage=22.0, Angle=90, KBG=90, FKB=0, BKB=23, Size=7.0, X=0.0, Y=-0.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BODY)
        ATTACK(ID=1, Part=0, Bone=hash40("rot"), Damage=16.0, Angle=85, KBG=100, FKB=0, BKB=23, Size=5.5, X=0.0, Y=-3.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        ATTACK(ID=2, Part=0, Bone=hash40("rot"), Damage=16.0, Angle=85, KBG=100, FKB=0, BKB=23, Size=5.5, X=0.0, Y=-3.0, Z=-5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
    }
    frame(Frame=24)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=28)
    if(is_excute){
        //HitModule::set_status_all(HIT_STATUS_NORMAL)
    }
    frame(Frame=37)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("rot"), Damage=12.0, Angle=70, KBG=70, FKB=0, BKB=30, Size=5.0, X=0.0, Y=-3.0, Z=-7.5, X2=0.0, Y2=-3.0, Z2=4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
    }
    frame(Frame=38)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_lw4",
animcmd = "game_attacklw4")]
pub fn bowser_d_smash(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_rate(5.0)
    }
    frame(Frame=5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=12)
    if(is_excute){
        MotionModule::set_rate(1.0)
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
        ATTACK(ID=0, Part=0, Bone=hash40("handl"), Damage=16.0, Angle=40, KBG=86, FKB=0, BKB=60, Size=6.0, X=2.3, Y=0.0, Z=1.0, X2=5.3, Y2=0.0, Z2=1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=28)
    if(is_excute){
        MotionModule::set_rate(1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=15.0, Angle=40, KBG=92, FKB=0, BKB=60, Size=6.0, X=2.5, Y=-0.6, Z=0.5, X2=4.5, Y2=-0.6, Z2=0.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_s3_s",
animcmd = "game_attacks3")]
pub fn bowser_s_tilt(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(9.0)
    }
    frame(Frame=7)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 4)
    }
    frame(Frame=10)
    if(is_excute){
        MotionModule::set_rate(1.0)
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
        HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=83, FKB=0, BKB=45, Size=5.5, X=5.1, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=83, FKB=0, BKB=45, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=83, FKB=0, BKB=45, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=5)
    if(is_excute){
        //HitModule::set_status_all(HIT_STATUS_NORMAL)
        AttackModule::clear_all()
    }
    });


}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_s3_hi",
animcmd = "game_attacks3hi")]
pub fn bowser_s_tilthi(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(9.0)
    }
    frame(Frame=7)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 4)
    }
    frame(Frame=10)
    if(is_excute){
        MotionModule::set_rate(1.0)
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
        HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=83, FKB=0, BKB=45, Size=5.5, X=5.1, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=83, FKB=0, BKB=45, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=83, FKB=0, BKB=45, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=5)
    if(is_excute){
        //HitModule::set_status_all(HIT_STATUS_NORMAL)
        AttackModule::clear_all()
    }
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_s3_lw",
animcmd = "game_attacks3lw")]
pub fn bowser_s_tiltlw(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(9.0)
    }
    frame(Frame=7)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 4)
    }
    frame(Frame=10)
    if(is_excute){
        MotionModule::set_rate(1.0)
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
        HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=83, FKB=0, BKB=45, Size=5.5, X=5.1, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=83, FKB=0, BKB=45, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=83, FKB=0, BKB=45, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=5)
    if(is_excute){
        //HitModule::set_status_all(HIT_STATUS_NORMAL)
        AttackModule::clear_all()
    }
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_lw3",
animcmd = "game_attacklw3")]
pub fn bowser_d_tilt(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(9.0)
    }
    frame(Frame=7)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 4)
    }
    frame(Frame=10)
    if(is_excute){
        MotionModule::set_rate(1.0)
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
        HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("colonellm"), Damage=7.0, Angle=361, KBG=100, FKB=50, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=7.0, Angle=361, KBG=100, FKB=50, BKB=0, Size=4.5, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=7.0, Angle=130, KBG=100, FKB=30, BKB=0, Size=5.0, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
    }
    wait(Frames=3)
    if(is_excute){
        //HitModule::set_status_all(HIT_STATUS_NORMAL)
        AttackModule::clear_all()
    }
    frame(Frame=15)
    if(is_excute){
        HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("colonells"), Damage=8.0, Angle=361, KBG=111, FKB=0, BKB=20, Size=4.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=8.0, Angle=361, KBG=111, FKB=0, BKB=20, Size=5.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=8.0, Angle=361, KBG=111, FKB=0, BKB=20, Size=5.6, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
    }
    wait(Frames=3)
    if(is_excute){
        //HitModule::set_status_all(HIT_STATUS_NORMAL)
        AttackModule::clear_all()
    }
    });


}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_hi3",
animcmd = "game_attackhi3")]
pub fn bowser_hi_tilt(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(10.0)
    }
    frame(Frame=8)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 4)
    }
    frame(Frame=11)
    if(is_excute){
        MotionModule::set_rate(1.0)
        HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
        ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=11.0, Angle=100, KBG=97, FKB=0, BKB=55, Size=5.5, X=6.9, Y=0.0, Z=-1.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=11.0, Angle=100, KBG=97, FKB=0, BKB=55, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
    }
    frame(Frame=17)
    if(is_excute){
        //HitModule::set_status_all(HIT_STATUS_NORMAL)
        AttackModule::clear_all()
    }
    FT_MOTION_RATE(FSM=0.66)
    frame(Frame=52)
    FT_MOTION_RATE(FSM=1)
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_dash",
animcmd = "game_attackdash")]
pub fn bowser_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(10.0)
    }
    frame(Frame=11)
    if(is_excute){
        MotionModule::set_rate(1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=80, KBG=35, FKB=0, BKB=110, Size=7.0, X=0.0, Y=9.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=80, KBG=35, FKB=0, BKB=110, Size=4.0, X=0.0, Y=9.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=4)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=80, KBG=30, FKB=0, BKB=80, Size=6.0, X=0.0, Y=9.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=80, KBG=30, FKB=0, BKB=80, Size=3.8, X=0.0, Y=9.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=6)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_11",
animcmd = "game_attack11")]
pub fn bowser_attack_11(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(6.0)
    }
    frame(Frame=7)
    if(is_excute){
        MotionModule::set_rate(1.0)
        HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=18, FKB=0, BKB=40, Size=3.2, X=0.0, Y=11.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=18, FKB=0, BKB=35, Size=3.2, X=0.0, Y=8.6, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=180, KBG=18, FKB=0, BKB=30, Size=3.8, X=0.0, Y=8.0, Z=13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=18, FKB=0, BKB=30, Size=3.8, X=0.0, Y=8.0, Z=13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        //HitModule::set_status_all(HIT_STATUS_NORMAL)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "attack_12",
animcmd = "game_attack12")]
pub fn bowser_attack_12(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(6.0)
    }
    frame(Frame=7)
    if(is_execute){
    MotionModule::set_rate(1.0)
    }
    frame(Frame=9)
    if(is_excute){
        HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=5.0, X=5.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=4.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=3.5, X=-5.0, Y=1.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
        //HitModule::set_status_all(HIT_STATUS_NORMAL)
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "special_lw",
animcmd = "game_speciallw")]
pub fn bowser_special_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(10.0)
    }
    frame(Frame=11)
    if(is_excute){
        MotionModule::set_rate(1.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=80, KBG=100, FKB=60, BKB=0, Size=4.0, X=0.0, Y=1.0, Z=17.0, X2=0.0, Y2=9.0, Z2=17.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=100, KBG=100, FKB=60, BKB=0, Size=2.0, X=0.0, Y=0.5, Z=22.5, X2=0.0, Y2=9.5, Z2=22.5, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=90, KBG=100, FKB=60, BKB=0, Size=4.0, X=0.0, Y=2.5, Z=13.0, X2=0.0, Y2=7.5, Z2=13.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        //AttackModule::set_add_reaction_frame(ID=0, Frames=8, Unk=false)
        //AttackModule::set_add_reaction_frame(ID=1, Frames=8, Unk=false)
        //AttackModule::set_add_reaction_frame(ID=2, Frames=8, Unk=false)
    }
    frame(Frame=12)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::on_flag(Flag=FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1)
    }
    frame(Frame=37)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=290, KBG=65, FKB=0, BKB=13, Size=8.3, X=0.0, Y=5.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPA,
animation = "special_air_lw",
animcmd = "game_specialairlw")]
pub fn bowser_special_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    MotionModule::set_rate(30.0)
    }
    frame(Frame=31)
    if(is_excute){
        MotionModule::set_rate(1.0)
        WorkModule::on_flag(Flag=FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=290, KBG=65, FKB=0, BKB=13, Size=8.6, X=0.0, Y=5.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
    }
    frame(Frame=33)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=76, KBG=75, FKB=0, BKB=45, Size=8.3, X=0.0, Y=5.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
    }
    });



}
pub fn install() {
    acmd::add_hooks!(
        bowser_fair,
        bowser_dair,
        bowser_bair,
        bowser_uair,
        bowser_f_smash,
        bowser_u_smash,
        bowser_d_smash,
        bowser_s_tilt,
        bowser_s_tilthi,
        bowser_s_tiltlw,
        bowser_d_tilt,
        bowser_hi_tilt,
        bowser_attack_dash,
        bowser_attack_11,
        bowser_attack_12,
        bowser_special_lw,
        bowser_special_air_lw
    );
}
