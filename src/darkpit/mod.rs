use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use skyline::libc::*;
use crate::config::CONFIG;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_11",
    animcmd = "game_attack11")]
pub fn instant_darkpit_attack_11(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(4.0, true)
    }
    frame(Frame=1)
    FT_MOTION_RATE(FSM=0.75)
    frame(Frame=5)
    FT_MOTION_RATE(FSM=1)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=30, Size=1.8, X=0.0, Y=8.0, Z=9.5, X2=0.0, Y2=8.0, Z2=7.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=30, Size=1.8, X=0.0, Y=8.0, Z=12.6, X2=0.0, Y2=8.0, Z2=7.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=15, FKB=0, BKB=20, Size=1.8, X=0.0, Y=8.0, Z=15.2, X2=0.0, Y2=8.0, Z2=7.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=15, FKB=0, BKB=20, Size=1.8, X=0.0, Y=8.0, Z=15.2, X2=0.0, Y2=8.0, Z2=7.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=3, Frames=2.0, Unk=false)
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
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_12",
    animcmd = "game_attack12")]
pub fn instant_darkpit_attack_12(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(4.0, true)
    }
    frame(Frame=5)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=28, Size=2.5, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=28, Size=3.2, X=0.0, Y=8.0, Z=13.6, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=15, FKB=0, BKB=20, Size=3.2, X=0.0, Y=8.0, Z=16.8, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=15, FKB=0, BKB=20, Size=3.2, X=0.0, Y=8.0, Z=16.8, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=3, Frames=2.0, Unk=false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=10)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
    }
    frame(Frame=16)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
    }
    });


}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_13",
    animcmd = "game_attack13")]
pub fn instant_darkpit_attack_13(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
        MotionModule::set_frame(2.0, true)
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
    }
    frame(Frame=3)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=70, KBG=100, FKB=0, BKB=60, Size=5.0, X=0.0, Y=8.0, Z=16.0, X2=0.0, Y2=8.0, Z2=11.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_dash",
    animcmd = "game_attackdash")]
pub fn instant_darkpit_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(6.0, true)
    }
    frame(Frame=7)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=60, KBG=74, FKB=0, BKB=80, Size=3.5, X=0.0, Y=4.5, Z=16.0, X2=0.0, Y2=7.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.37)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
    });


}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_s3_s",
    animcmd = "game_attacks3")]
pub fn instant_darkpit_attack_s3(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(9.0, true)
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=3.5, X=0.0, Y=7.5, Z=8.0, X2=0.0, Y2=7.5, Z2=3.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=3.5, X=0.0, Y=7.5, Z=14.0, X2=0.0, Y2=7.5, Z2=3.2, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    wait(Frames=5)
    if(is_excute){
        AttackModule::clear_all()
    }
    });


}
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn instant_darkpit_attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(5.0, true)
    }
    frame(Frame=6)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=365, KBG=100, FKB=85, BKB=0, Size=4.0, X=0.0, Y=24.0, Z=2.3, X2=0.0, Y2=10.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        //AttackModule::set_vec_target_pos(0, hash40("top"), 3.8, 23.5, 10)
    }
    frame(Frame=9)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=15)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=83, KBG=30, FKB=0, BKB=70, Size=6.0, X=0.0, Y=24.0, Z=5.0, X2=0.0, Y2=22.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
    }
    frame(Frame=17)
    if(is_excute){
        AttackModule::clear_all()
    }
    FT_MOTION_RATE(FSM=0.85)
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn instant_darkpit_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(5.0, true)
    }
    frame(Frame=6)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=83, KBG=46, FKB=0, BKB=70, Size=2.5, X=0.0, Y=3.0, Z=16.0, X2=0.0, Y2=5.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    FT_MOTION_RATE(FSM=0.72)
    });

}


#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_s4_s",
    animcmd = "game_attacks4")]
pub fn instant_darkpit_attack_s4_s(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(8.0, true)
    }
    frame(Frame=4)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=100, KBG=100, FKB=20, BKB=0, Size=6.0, X=0.0, Y=7.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=70, KBG=100, FKB=20, BKB=0, Size=6.0, X=0.0, Y=7.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=35, KBG=100, FKB=53, BKB=0, Size=6.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=5.0, Angle=0, KBG=100, FKB=15, BKB=0, Size=6.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    frame(Frame=11)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=21)
    if(is_excute){
        ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=122, FKB=0, BKB=42, Size=6.0, X=0.0, Y=7.5, Z=13.5, X2=0.0, Y2=7.5, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    frame(Frame=23)
    if(is_excute){
        AttackModule::clear_all()
    }
    });
}


#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
pub fn instant_darkpit_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(5.0, true)
    }
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=6)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=365, KBG=100, FKB=20, BKB=40, Size=4.0, X=0.0, Y=28.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=120, KBG=100, FKB=20, BKB=40, Size=4.5, X=0.0, Y=24.0, Z=7.0, X2=0.0, Y2=24.0, Z2=4.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=120, KBG=100, FKB=20, BKB=40, Size=4.5, X=0.0, Y=24.0, Z=-7.0, X2=0.0, Y2=24.0, Z2=-4.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=100, FKB=33, BKB=40, Size=4.5, X=0.0, Y=24.0, Z=7.0, X2=0.0, Y2=24.0, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=3.0, Angle=105, KBG=100, FKB=33, BKB=40, Size=4.5, X=0.0, Y=24.0, Z=7.0, X2=0.0, Y2=24.0, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        //AttackModule::set_vec_target_pos(3, hash40("top"), 0, 24.5, 10)
    }
    wait(Frames=1)
    if(is_excute){
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=110, KBG=100, FKB=140, BKB=0, Size=4.0, X=0.0, Y=14.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=3.0, Angle=123, KBG=100, FKB=140, BKB=0, Size=4.0, X=0.0, Y=14.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        //AttackModule::clear(ID=0)
        //AttackModule::clear(ID=2)
        //AttackModule::clear(ID=3)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=2.0, Angle=98, KBG=100, FKB=20, BKB=0, Size=4.0, X=0.0, Y=28.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=2.0, Angle=120, KBG=100, FKB=50, BKB=0, Size=5.2, X=0.0, Y=24.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=2.0, Angle=120, KBG=100, FKB=50, BKB=0, Size=5.2, X=0.0, Y=24.0, Z=-7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=18)
    if(is_excute){
        ATTACK(ID=0, Part=2, Bone=hash40("top"), Damage=8.0, Angle=89, KBG=111, FKB=0, BKB=66, Size=5.8, X=0.0, Y=34.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=2, Bone=hash40("top"), Damage=8.0, Angle=89, KBG=111, FKB=0, BKB=66, Size=5.0, X=0.0, Y=24.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=2, Part=2, Bone=hash40("top"), Damage=8.0, Angle=89, KBG=111, FKB=0, BKB=66, Size=5.0, X=0.0, Y=31.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=3, Part=2, Bone=hash40("top"), Damage=8.0, Angle=89, KBG=111, FKB=0, BKB=66, Size=5.0, X=0.0, Y=31.0, Z=-8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
pub fn instant_darkpit_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(3.0, true)
    }
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=5)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=48, KBG=98, FKB=0, BKB=40, Size=3.7, X=0.0, Y=3.3, Z=6.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=48, KBG=98, FKB=0, BKB=40, Size=3.6, X=0.0, Y=2.8, Z=12.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=48, KBG=93, FKB=0, BKB=35, Size=3.4, X=0.0, Y=2.0, Z=17.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
    }
    frame(Frame=6)
    if(is_execute){
        MotionModule::set_frame(17.0, true)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=18)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=30, KBG=93, FKB=0, BKB=25, Size=3.7, X=0.0, Y=3.3, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=30, KBG=93, FKB=0, BKB=25, Size=3.6, X=0.0, Y=2.8, Z=-12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=30, KBG=93, FKB=0, BKB=25, Size=3.4, X=0.0, Y=2.0, Z=-17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
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
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
pub fn instant_darkpit_attack_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
    frame(Frame=4)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    for(7 Iterations){
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.7, Angle=48, KBG=100, FKB=50, BKB=0, Size=3.0, X=0.0, Y=4.6, Z=0.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.7, Angle=335, KBG=100, FKB=30, BKB=0, Size=3.0, X=0.0, Y=13.0, Z=1.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.7, Angle=220, KBG=100, FKB=30, BKB=0, Size=2.5, X=0.0, Y=12.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.7, Angle=140, KBG=100, FKB=30, BKB=0, Size=2.5, X=0.0, Y=12.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=0.7, Angle=130, KBG=100, FKB=65, BKB=0, Size=3.0, X=0.0, Y=4.6, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=1)
    }
    if(is_excute){
        ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=4.5, Angle=361, KBG=100, FKB=0, BKB=60, Size=9.5, X=0.0, Y=9.5, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=30)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn instant_darkpit_attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
        MotionModule::set_frame(10.0, true)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=11)
    for(2 Iterations){
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=367, KBG=48, FKB=0, BKB=20, Size=2.3, X=0.0, Y=5.0, Z=10.0, X2=0.0, Y2=5.0, Z2=19.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=1)
    }
    frame(Frame=18)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=147, FKB=0, BKB=20, Size=4.0, X=0.0, Y=5.0, Z=11.0, X2=0.0, Y2=5.0, Z2=19.0, Hitlag=2.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=28)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
pub fn instant_darkpit_attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
        MotionModule::set_frame(9.0, true)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=43, KBG=105, FKB=0, BKB=30, Size=3.7, X=0.0, Y=6.2, Z=-19.4, X2=0.0, Y2=6.2, Z2=-17.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=66, KBG=96, FKB=0, BKB=30, Size=3.5, X=0.0, Y=6.2, Z=-18.4, X2=0.0, Y2=6.2, Z2=-9.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=43, KBG=105, FKB=0, BKB=30, Size=3.5, X=0.0, Y=6.2, Z=-18.4, X2=0.0, Y2=6.2, Z2=-17.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    frame(Frame=13)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=28)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn instant_darkpit_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(11.0, true)
    }
    frame(Frame=1)
    FT_MOTION_RATE(FSM=0.818)
    frame(Frame=5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=12)
    FT_MOTION_RATE(FSM=1)
    for(4 Iterations){
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=90, KBG=100, FKB=80, BKB=0, Size=2.8, X=0.0, Y=9.6, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=270, KBG=100, FKB=20, BKB=0, Size=3.8, X=0.0, Y=19.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=165, KBG=100, FKB=60, BKB=0, Size=3.8, X=0.0, Y=16.8, Z=-8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.5, Angle=165, KBG=100, FKB=60, BKB=0, Size=3.8, X=0.0, Y=16.8, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=1)
    }
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=80, KBG=143, FKB=0, BKB=50, Size=4.0, X=0.0, Y=9.6, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=80, KBG=143, FKB=0, BKB=50, Size=5.6, X=0.0, Y=16.8, Z=-8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=80, KBG=143, FKB=0, BKB=50, Size=5.6, X=0.0, Y=16.8, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=5.0, Angle=80, KBG=143, FKB=0, BKB=50, Size=5.6, X=0.0, Y=17.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=37)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });

}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn instant_darkpit_attack_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(9.0, true)
    }
    frame(Frame=7)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=270, KBG=80, FKB=0, BKB=0, Size=4.0, X=0.0, Y=-4.0, Z=0.0, X2=0.0, Y2=-6.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=270, KBG=80, FKB=0, BKB=10, Size=4.0, X=0.0, Y=-4.0, Z=0.0, X2=0.0, Y2=-6.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=60, KBG=80, FKB=0, BKB=40, Size=6.0, X=0.0, Y=0.0, Z=-8.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    wait(Frames=1)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=80, KBG=80, FKB=0, BKB=50, Size=6.0, X=0.0, Y=-4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=70, KBG=80, FKB=0, BKB=40, Size=6.0, X=0.0, Y=0.0, Z=8.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=36)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PITB,
    animation = "special_s_start",
    animcmd = "game_specialsstart")]
pub fn instant_darkpit_special_s_start(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
        MotionModule::set_frame(15.0, true)
    }
    frame(Frame=11)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT)
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
    }
    frame(Frame=16)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=0, FKB=0, BKB=0, Size=2.0, X=0.0, Y=12.0, Z=9.0, X2=0.0, Y2=4.0, Z2=9.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_search"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        WorkModule::on_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF)
        sv_module_access::shield(MA_MSC_CMD_SHIELD_ON, COLLISION_KIND_REFLECTOR, FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S)
        WorkModule::on_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF)
    }
    frame(Frame=27)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
    }
    frame(Frame=31)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF)
    }
    frame(Frame=36)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MTRANS_AIR_UNABLE)
        AttackModule::clear_all()
        WorkModule::off_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF)
        sv_module_access::shield(MA_MSC_CMD_SHIELD_OFF, COLLISION_KIND_REFLECTOR, FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S)
        WorkModule::off_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF)
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_PITB,
animation = "special_air_s_start",
animcmd = "game_specialairsstart")]
pub fn instant_darkpit_special_air_s_start(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
        MotionModule::set_frame(18.0, true)
        WorkModule::on_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT)
    }
    frame(Frame=11)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
    }
    frame(Frame=19)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=0, FKB=0, BKB=0, Size=2.0, X=0.0, Y=14.0, Z=9.0, X2=0.0, Y2=4.0, Z2=9.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_search"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        WorkModule::on_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF)
        sv_module_access::shield(MA_MSC_CMD_SHIELD_ON, COLLISION_KIND_REFLECTOR, FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S)
    }
    frame(Frame=27)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
    }
    frame(Frame=31)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF)
    }
    frame(Frame=36)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF)
        sv_module_access::shield(MA_MSC_CMD_SHIELD_OFF, COLLISION_KIND_REFLECTOR, FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S)
        AttackModule::clear_all()
    }
    });

}
pub fn install() {
    if CONFIG.instant_info.darkpit{
        acmd::add_hooks!(
        instant_darkpit_attack_11,
        instant_darkpit_attack_12,
        instant_darkpit_attack_13,
        instant_darkpit_attack_dash,
        instant_darkpit_attack_s3,
        instant_darkpit_attack_hi3,
        instant_darkpit_attack_lw3,
        instant_darkpit_attack_s4_s,
        instant_darkpit_attack_hi4,
        instant_darkpit_attack_lw4,
        instant_darkpit_attack_air_n,
        instant_darkpit_attack_air_f,
        instant_darkpit_attack_air_b,
        instant_darkpit_attack_air_hi,
        instant_darkpit_attack_air_lw,
        instant_darkpit_special_s_start,
        instant_darkpit_special_air_s_start,
        //instant_darkpit_special_lw,
        //instant_darkpit_special_air_lw
    );
    }


}