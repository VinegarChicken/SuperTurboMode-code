use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use skyline::libc::*;
use crate::config::CONFIG;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_11",
    animcmd = "game_attack11")]
pub fn instant_ridley_attack_11(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(3.0, true)
    }
    frame(Frame=4)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=30, Size=2.5, X=0.0, Y=7.0, Z=9.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=30, Size=2.5, X=0.0, Y=7.0, Z=13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=15, FKB=0, BKB=25, Size=2.8, X=0.0, Y=7.0, Z=16.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=15, FKB=0, BKB=25, Size=2.8, X=0.0, Y=7.0, Z=16.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=16)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
    }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_12",
    animcmd = "game_attack12")]
pub fn instant_ridley_attack_12(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(4.0, true)
    }
    frame(Frame=5)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=361, KBG=20, FKB=0, BKB=30, Size=3.2, X=0.0, Y=7.0, Z=9.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=361, KBG=20, FKB=0, BKB=30, Size=3.4, X=0.0, Y=7.0, Z=13.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=361, KBG=20, FKB=0, BKB=25, Size=3.6, X=0.0, Y=7.0, Z=18.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=9)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=12)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
    }
    frame(Frame=18)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
    }
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_13",
    animcmd = "game_attack13")]
pub fn instant_ridley_attack_13(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(3.0, true)
    }
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=42, KBG=128, FKB=0, BKB=50, Size=5.0, X=0.0, Y=7.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BITE)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=42, KBG=128, FKB=0, BKB=50, Size=5.8, X=0.0, Y=7.5, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BITE)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_dash",
    animcmd = "game_attackdash")]
pub fn instant_ridley_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(11.0, true)
    }
    frame(Frame=12)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=42, KBG=70, FKB=0, BKB=70, Size=7.0, X=0.0, Y=6.5, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BITE)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_s3_s",
    animcmd = "game_attacks3")]
pub fn instant_ridley_attack_s3(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(10.0, true)
    }
    frame(Frame=1)
    FT_MOTION_RATE(FSM=0.8)
    frame(Frame=5)
    FT_MOTION_RATE(FSM=1)
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("tail7"), Damage=10.0, Angle=361, KBG=71, FKB=0, BKB=50, Size=5.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=71, FKB=0, BKB=50, Size=7.5, X=0.0, Y=7.0, Z=7.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=2, Part=0, Bone=hash40("tail8"), Damage=13.0, Angle=361, KBG=71, FKB=0, BKB=55, Size=5.0, X=6.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_RIDLEY,
animation = "attack_s3_hi",
animcmd = "game_attacks3hi")]
pub fn instant_ridley_attack_s3hi(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(10.0, true)
    }
    frame(Frame=1)
    FT_MOTION_RATE(FSM=0.8)
    frame(Frame=5)
    FT_MOTION_RATE(FSM=1)
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("tail7"), Damage=10.0, Angle=361, KBG=71, FKB=0, BKB=50, Size=5.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=71, FKB=0, BKB=50, Size=7.5, X=0.0, Y=7.0, Z=7.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=2, Part=0, Bone=hash40("tail8"), Damage=13.0, Angle=361, KBG=71, FKB=0, BKB=55, Size=5.0, X=6.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    });


}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_RIDLEY,
animation = "attack_s3_lw",
animcmd = "game_attacks3lw")]
pub fn instant_ridley_attack_s3lw(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(10.0, true)
    }
    frame(Frame=1)
    FT_MOTION_RATE(FSM=0.8)
    frame(Frame=5)
    FT_MOTION_RATE(FSM=1)
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("tail7"), Damage=10.0, Angle=361, KBG=71, FKB=0, BKB=50, Size=5.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=71, FKB=0, BKB=50, Size=7.5, X=0.0, Y=7.0, Z=7.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=2, Part=0, Bone=hash40("tail8"), Damage=13.0, Angle=361, KBG=71, FKB=0, BKB=55, Size=5.0, X=6.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn instant_ridley_attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(7.0, true)
    }
    frame(Frame=8)
    if(is_excute){
        HIT_NODE(hash40("head"), HIT_STATUS_XLU)
        HIT_NODE(hash40("mouth1"), HIT_STATUS_XLU)
        HIT_NODE(hash40("wingr2"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("wingr2"), Damage=7.0, Angle=88, KBG=130, FKB=0, BKB=30, Size=6.5, X=1.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
        ATTACK(ID=1, Part=0, Bone=hash40("wingr3"), Damage=9.0, Angle=88, KBG=110, FKB=0, BKB=30, Size=5.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
        ATTACK(ID=2, Part=0, Bone=hash40("wingr6"), Damage=7.0, Angle=88, KBG=130, FKB=0, BKB=30, Size=6.5, X=-5.7, Y=2.0, Z=-3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
        ATTACK(ID=3, Part=0, Bone=hash40("wingr6"), Damage=7.0, Angle=88, KBG=130, FKB=0, BKB=30, Size=6.5, X=0.0, Y=2.0, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=7.0, Angle=88, KBG=130, FKB=0, BKB=30, Size=7.0, X=0.0, Y=13.0, Z=0.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=2)
    if(is_excute){
        HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
        HIT_NODE(hash40("mouth1"), HIT_STATUS_NORMAL)
        //AttackModule::clear(ID=4)
    }
    frame(Frame=13)
    if(is_excute){
        HIT_NODE(hash40("wingr2"), HIT_STATUS_NORMAL)
        AttackModule::clear_all()
    }
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn instant_ridley_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(8.0, true)
    }
    frame(Frame=9)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=78, KBG=80, FKB=0, BKB=65, Size=4.5, X=0.0, Y=3.8, Z=22.0, X2=0.0, Y2=3.8, Z2=5.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=80, KBG=80, FKB=0, BKB=65, Size=4.5, X=0.0, Y=3.8, Z=28.5, X2=0.0, Y2=3.8, Z2=27.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
        AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}


#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_s4_s",
    animcmd = "game_attacks4")]
pub fn instant_ridley_attack_s4_s(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(16.0, true)
        EffectModule::set_frame(0 as u32, 16.0)
    }
    frame(Frame=6)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=17)
    if(is_excute){
        FighterAreaModuleImpl::enable_fix_jostle_area(6.0, 7.0)
    }
    frame(Frame=18)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=80, FKB=0, BKB=58, Size=9.0, X=0.0, Y=8.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
    }
    wait(Frames=4)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=54)
    if(is_excute){
        FighterAreaModuleImpl::enable_fix_jostle_area(6.0, 5.0)
    }
    });

}


#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
pub fn instant_ridley_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(11.0, true)
    }
    frame(Frame=7)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=12)
    if(is_excute){
        HIT_NODE(hash40("kneer"), HIT_STATUS_XLU)
        HIT_NODE(hash40("toer"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=17.0, Angle=82, KBG=78, FKB=0, BKB=58, Size=4.5, X=0.0, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=17.0, Angle=82, KBG=78, FKB=0, BKB=58, Size=4.5, X=0.0, Y=-1.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("footr"), Damage=17.0, Angle=82, KBG=78, FKB=0, BKB=58, Size=6.0, X=-1.5, Y=-1.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=6)
    if(is_excute){
        HIT_NODE(hash40("kneer"), HIT_STATUS_NORMAL)
        HIT_NODE(hash40("toer"), HIT_STATUS_NORMAL)
        AttackModule::clear_all()
    }
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
pub fn instant_ridley_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(23.0, true)
    }
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=24)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=85, KBG=76, FKB=0, BKB=69, Size=7.5, X=0.0, Y=5.0, Z=21.0, X2=0.0, Y2=5.0, Z2=-21.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=4)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
pub fn instant_ridley_attack_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(7.0, true)
    }
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=8)
    if(is_excute){
        ATTACK(ID=1, Part=0, Bone=hash40("tail8"), Damage=12.0, Angle=44, KBG=62, FKB=0, BKB=55, Size=6.5, X=7.0, Y=-2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=2, Part=0, Bone=hash40("tail8"), Damage=9.0, Angle=44, KBG=62, FKB=0, BKB=55, Size=6.5, X=0.0, Y=-2.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=0, Part=0, Bone=hash40("tail1"), Damage=9.0, Angle=44, KBG=62, FKB=0, BKB=55, Size=7.5, X=3.0, Y=-2.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
    }
    frame(Frame=15)
    if(is_excute){
        ATTACK(ID=1, Part=0, Bone=hash40("tail8"), Damage=8.0, Angle=44, KBG=62, FKB=0, BKB=55, Size=5.0, X=7.0, Y=-2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=2, Part=0, Bone=hash40("tail8"), Damage=5.0, Angle=44, KBG=62, FKB=0, BKB=55, Size=5.0, X=0.0, Y=-2.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=0, Part=0, Bone=hash40("tail1"), Damage=5.0, Angle=44, KBG=62, FKB=0, BKB=55, Size=6.0, X=3.0, Y=-2.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
    }
    frame(Frame=21)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=28)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=63)
    if(is_excute){
        sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
    }
    });


}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn instant_ridley_attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(9.0, true)
    }
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("tail8"), Damage=5.0, Angle=367, KBG=100, FKB=35, BKB=0, Size=5.0, X=6.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=1, Part=0, Bone=hash40("tail8"), Damage=5.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=5.0, X=6.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=2, Part=0, Bone=hash40("tail8"), Damage=3.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=6.5, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=3, Part=0, Bone=hash40("tail8"), Damage=3.0, Angle=367, KBG=100, FKB=45, BKB=0, Size=6.5, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=7.2, X=0.0, Y=8.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=3.0, Angle=66, KBG=100, FKB=55, BKB=0, Size=7.2, X=0.0, Y=8.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        MotionModule::set_frame(12.0, true)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=13)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("tail8"), Damage=5.0, Angle=367, KBG=100, FKB=35, BKB=0, Size=5.0, X=6.4, Y=-2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=1, Part=0, Bone=hash40("tail8"), Damage=3.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=6.5, X=-0.5, Y=-2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=7.2, X=0.0, Y=8.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        MotionModule::set_frame(15.0, true)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=16)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("tail8"), Damage=7.0, Angle=361, KBG=87, FKB=0, BKB=45, Size=5.0, X=6.4, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=1, Part=0, Bone=hash40("tail8"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=45, Size=6.5, X=-0.5, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=45, Size=7.2, X=0.0, Y=8.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=43)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=65)
    if(is_excute){
        sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
    }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
pub fn instant_ridley_attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
        MotionModule::set_frame(13.0, true)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=1)
    FT_MOTION_RATE(FSM=0.5)
    frame(Frame=8)
    FT_MOTION_RATE(FSM=1)
    frame(Frame=14)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=16.0, Angle=361, KBG=83, FKB=0, BKB=35, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=16.0, Angle=361, KBG=83, FKB=0, BKB=35, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("footr"), Damage=16.0, Angle=361, KBG=83, FKB=0, BKB=35, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=42)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_RIDLEY,
animation = "attack_air_b",
animcmd = "game_attackairb")]
pub fn instant_ridley_attack_air_b_default(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=1)
    FT_MOTION_RATE(FSM=0.5)
    frame(Frame=8)
    FT_MOTION_RATE(FSM=1)
    frame(Frame=14)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=16.0, Angle=361, KBG=83, FKB=0, BKB=35, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=16.0, Angle=361, KBG=83, FKB=0, BKB=35, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("footr"), Damage=16.0, Angle=361, KBG=83, FKB=0, BKB=35, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=42)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn instant_ridley_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(10.0, true)
    }
    frame(Frame=4)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=5)
    if(is_excute){
        JostleModule::set_status(false)
    }
    frame(Frame=10)
    if(is_excute){
        JostleModule::set_status(true)
    }
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=86, KBG=62, FKB=0, BKB=70, Size=12.0, X=0.0, Y=22.5, Z=0.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=90, KBG=64, FKB=0, BKB=70, Size=5.5, X=0.0, Y=35.5, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=2)
    if(is_excute){
        //AttackModule::clear(ID=0)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=35)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn instant_ridley_attack_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
        MotionModule::set_frame(10.0, true)
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0, 1, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        KineticModule::suspend_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE)
    }
    frame(Frame=4)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=7)
    if(is_excute){
        JostleModule::set_status(false)
    }
    frame(Frame=11)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
        SET_SPEED_EX(0, -3.8, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
    }
    FT_MOTION_RATE(FSM=1.5)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=275, KBG=75, FKB=0, BKB=55, Size=3.5, X=0.0, Y=-0.5, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=50, KBG=95, FKB=0, BKB=45, Size=8.0, X=0.0, Y=3.5, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=5)
    if(is_excute){
        //AttackModule::clear(ID=0)
    }
    frame(Frame=31)
    FT_MOTION_RATE(FSM=1)
    if(is_excute){
        AttackModule::clear_all()
        JostleModule::set_status(true)
    }
    frame(Frame=40)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=45)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE)
        KineticModule::resume_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    }
    frame(Frame=69)
    if(is_excute){
        sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
    }
    });


}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "special_s",
    animcmd = "game_specials")]
pub fn instant_ridley_special_s(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(20.0, true)
    }
    frame(Frame=21)
    if(is_excute){
        //KineticModule::add_speed(-0.9, 0.1, 0.0)
        ArticleModule::generate_article(FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FIRE, false, 0)
    }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY,
    animation = "special_air_s",
    animcmd = "game_specialairs")]
pub fn instant_ridley_special_air_s(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(20.0, true)
    }
    frame(Frame=21)
    if(is_excute){
        //KineticModule::add_speed(-0.9, 0.1, 0.0)
        ArticleModule::generate_article(FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FIRE, false, 0)
    }
    });


}


pub fn install() {
    if CONFIG.instant_info.ridley{
        acmd::add_hooks!(
        instant_ridley_attack_11,
        instant_ridley_attack_12,
        instant_ridley_attack_13,
        instant_ridley_attack_dash,
        instant_ridley_attack_s3,
        instant_ridley_attack_hi3,
        instant_ridley_attack_lw3,
        instant_ridley_attack_s4_s,
        instant_ridley_attack_hi4,
        instant_ridley_attack_lw4,
        instant_ridley_attack_air_n,
        instant_ridley_attack_air_f,
        instant_ridley_attack_air_b,
        instant_ridley_attack_air_hi,
        instant_ridley_attack_air_lw,
        //instant_ridley_special_s,
        //instant_ridley_special_air_s,
        //instant_ridley_special_lw,
        //instant_ridley_special_air_lw
    );
    }
    else{
        acmd::add_hooks!(
        instant_ridley_attack_air_b_default,
        );
    }

}