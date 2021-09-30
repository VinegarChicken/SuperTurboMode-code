use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use skyline::libc::*;
use crate::config::CONFIG;

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_11",
animcmd = "game_attack11")]
pub fn instant_dedede_attack_11(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(9.0, true)
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=15, FKB=0, BKB=35, Size=2.8, X=0.0, Y=6.5, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=15, FKB=0, BKB=35, Size=2.8, X=0.0, Y=6.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=15, FKB=0, BKB=25, Size=2.8, X=0.0, Y=6.5, Z=14.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.5, Angle=180, KBG=15, FKB=0, BKB=25, Size=2.8, X=0.0, Y=6.5, Z=19.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=15, FKB=0, BKB=25, Size=2.8, X=0.0, Y=6.5, Z=19.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=1, Frames=5.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=2, Frames=5.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=3, Frames=5.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=4, Frames=5.0, Unk=false)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=12)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=22)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
    }
    });

}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_12",
animcmd = "game_attack12")]
pub fn instant_dedede_attack_12(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(10.0, true)
    }
    frame(Frame=11)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.2, Angle=361, KBG=15, FKB=0, BKB=35, Size=3.5, X=0.0, Y=6.5, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.2, Angle=361, KBG=15, FKB=0, BKB=30, Size=3.5, X=0.0, Y=6.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.2, Angle=361, KBG=12, FKB=0, BKB=25, Size=3.8, X=0.0, Y=6.5, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.2, Angle=180, KBG=12, FKB=0, BKB=25, Size=3.8, X=0.0, Y=6.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.2, Angle=361, KBG=12, FKB=0, BKB=25, Size=3.8, X=0.0, Y=6.5, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=1, Frames=5.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=2, Frames=5.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=3, Frames=5.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=4, Frames=5.0, Unk=false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=18)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
    }
    frame(Frame=22)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    });

}


#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_dash",
animcmd = "game_attackdash")]
pub fn instant_dedede_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(25.0, true)
    }
    frame(Frame=26)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=92, FKB=0, BKB=65, Size=7.0, X=0.0, Y=5.0, Z=11.8, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
    }
    frame(Frame=28)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=92, FKB=0, BKB=55, Size=6.0, X=0.0, Y=4.0, Z=11.8, X2=0.0, Y2=4.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
    }
    frame(Frame=36)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=92, FKB=0, BKB=55, Size=4.5, X=0.0, Y=3.5, Z=10.5, X2=0.0, Y2=3.5, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
    }
    frame(Frame=39)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=92, FKB=0, BKB=55, Size=3.0, X=0.0, Y=3.0, Z=9.5, X2=0.0, Y2=3.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
    }
    frame(Frame=42)
    if(is_excute){
        AttackModule::clear_all()
    }
    });
}


#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_s3_s",
animcmd = "game_attacks3")]
pub fn instant_dedede_attack_s3(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(11.0, true)
    }
    frame(Frame=12)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=2.0, Angle=22, KBG=100, FKB=60, BKB=0, Size=3.0, X=-8.0, Y=0.0, Z=0.0, X2=-2.0, Y2=0.0, Z2=0.0, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=2.0, Angle=22, KBG=100, FKB=50, BKB=0, Size=3.5, X=1.0, Y=0.0, Z=0.0, X2=8.0, Y2=0.0, Z2=0.0, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=100, KBG=100, FKB=30, BKB=0, Size=3.0, X=0.0, Y=4.0, Z=19.2, X2=0.0, Y2=4.0, Z2=25.8, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=250, KBG=100, FKB=26, BKB=0, Size=4.0, X=0.0, Y=9.5, Z=20.0, X2=0.0, Y2=9.5, Z2=25.0, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    frame(Frame=22)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=23)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=3.0, Angle=50, KBG=190, FKB=0, BKB=30, Size=3.5, X=-8.0, Y=0.0, Z=0.0, X2=-2.0, Y2=0.0, Z2=0.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=3.0, Angle=50, KBG=190, FKB=0, BKB=30, Size=3.5, X=1.0, Y=0.0, Z=0.0, X2=8.0, Y2=0.0, Z2=0.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=2, Part=0, Bone=hash40("hammer2"), Damage=3.0, Angle=50, KBG=190, FKB=0, BKB=30, Size=8.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    frame(Frame=24)
    if(is_excute){
        AttackModule::clear_all()
    }
    });
}


#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_hi3",
animcmd = "game_attackhi3")]
pub fn instant_dedede_attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(6.0, true)
    }
    frame(Frame=6)
    if(is_excute){
        FighterAreaModuleImpl::enable_fix_jostle_area(7.0, 6.0)
    }
    frame(Frame=7)
    if(is_excute){
        HIT_NODE(hash40("head"), HIT_STATUS_XLU)
        HIT_NODE(hash40("shoulderl"), HIT_STATUS_XLU)
        HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=11.5, Angle=96, KBG=94, FKB=0, BKB=65, Size=6.0, X=-1.5, Y=1.0, Z=1.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
        ATTACK(ID=1, Part=0, Bone=hash40("head"), Damage=9.5, Angle=96, KBG=94, FKB=0, BKB=65, Size=6.5, X=4.8, Y=0.9, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
        FighterAreaModuleImpl::enable_fix_jostle_area(2.0, 6.0)
        AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
    }
    frame(Frame=8)
    frame(Frame=14)
    if(is_excute){
        HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
        HIT_NODE(hash40("shoulderl"), HIT_STATUS_NORMAL)
        HIT_NODE(hash40("arml"), HIT_STATUS_NORMAL)
        AttackModule::clear_all()
    }
    });


}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_lw3",
animcmd = "game_attacklw3")]
pub fn instant_dedede_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(5.0, true)
    }
    frame(Frame=6)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=35, KBG=75, FKB=0, BKB=60, Size=7.0, X=0.0, Y=6.5, Z=12.0, X2=0.0, Y2=6.5, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=2)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=35, KBG=75, FKB=0, BKB=60, Size=5.0, X=0.0, Y=5.0, Z=12.0, X2=0.0, Y2=5.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=4)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}


#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_s4_s",
animcmd = "game_attacks4s")]
pub fn instant_dedede_attack_s4_s(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(39.0, true)
    }
    frame(Frame=32)
    if(is_excute){
        FighterAreaModuleImpl::enable_fix_jostle_area(6.0, 7.0)
    }
    frame(Frame=34)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=40)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("hammer2"), Damage=0.0, Angle=0, KBG=0, FKB=0, BKB=0, Size=3.5, X=-6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("hammer2"), Damage=0.0, Angle=0, KBG=0, FKB=0, BKB=0, Size=3.5, X=-12.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=2, Part=0, Bone=hash40("hammer2"), Damage=16.0, Angle=45, KBG=55, FKB=0, BKB=85, Size=4.5, X=1.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        //AttackModule::set_vec_target_pos(0, hash40("top"), 16, 2, 2)
        //AttackModule::set_vec_target_pos(1, hash40("top"), 16, 2, 2)
    }
    wait(Frames=2)
    if(is_excute){
        //AttackModule::clear(ID=0)
        ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=18.5, Angle=45, KBG=85, FKB=0, BKB=50, Size=3.5, X=6.0, Y=-3.0, Z=0.0, X2=3.0, Y2=-5.0, Z2=0.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=2, Part=0, Bone=hash40("hammer2"), Damage=25.0, Angle=361, KBG=74, FKB=0, BKB=70, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    frame(Frame=44)
    if(is_excute){
        //AttackModule::clear(ID=1)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=11.0, Angle=60, KBG=30, FKB=0, BKB=100, Size=8.0, X=0.0, Y=5.0, Z=5.0, X2=0.0, Y2=5.0, Z2=27.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    });


}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_hi4",
animcmd = "game_attackhi4")]
pub fn instant_dedede_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(16.0, true)
    }
    frame(Frame=7)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=17)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("hammer2"), Damage=16.0, Angle=80, KBG=95, FKB=0, BKB=40, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("hammer2"), Damage=16.0, Angle=80, KBG=96, FKB=0, BKB=40, Size=4.0, X=-6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=2, Part=0, Bone=hash40("hammer2"), Damage=16.0, Angle=80, KBG=96, FKB=0, BKB=40, Size=4.0, X=-12.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=80, KBG=96, FKB=0, BKB=40, Size=4.0, X=0.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    frame(Frame=25)
    if(is_excute){
        AttackModule::clear_all()
    }
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_lw4",
animcmd = "game_attacklw4")]
pub fn instant_dedede_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(13.0, true)
    }
    frame(Frame=6)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=14)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("hammer2"), Damage=13.0, Angle=30, KBG=80, FKB=0, BKB=60, Size=7.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("hammer2"), Damage=13.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=4.0, X=-7.0, Y=-3.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    frame(Frame=23)
    if(is_excute){
        AttackModule::clear_all()
    }
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_air_n",
animcmd = "game_attackairn")]
pub fn instant_dedede_attack_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(6.0, true)
    }
    frame(Frame=3)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=7)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=95, FKB=0, BKB=40, Size=10.0, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
    }
    frame(Frame=9)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=80, KBG=95, FKB=0, BKB=40, Size=6.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
    }
    frame(Frame=30)
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
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_air_f",
animcmd = "game_attackairf")]
pub fn instant_dedede_attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(12.0, true)
    }
    frame(Frame=5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=13)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("hammer2"), Damage=12.0, Angle=361, KBG=98, FKB=0, BKB=15, Size=8.0, X=-2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("hammer2"), Damage=12.0, Angle=361, KBG=98, FKB=0, BKB=15, Size=5.0, X=-12.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    frame(Frame=16)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=40)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_air_b",
animcmd = "game_attackairb")]
pub fn instant_dedede_attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(16.0, true)
    }
    frame(Frame=5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=17)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=88, FKB=0, BKB=30, Size=6.5, X=0.0, Y=10.0, Z=-18.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=88, FKB=0, BKB=30, Size=6.5, X=0.0, Y=10.0, Z=-12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=88, FKB=0, BKB=30, Size=6.5, X=0.0, Y=10.0, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    frame(Frame=20)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=33)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_air_hi",
animcmd = "game_attackairhi")]
pub fn instant_dedede_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(9.0, true)
    }
    frame(Frame=5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=10)
    for(7 Iterations){
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=1.0, Angle=367, KBG=100, FKB=20, BKB=0, Size=6.0, X=17.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=1.0, Angle=367, KBG=100, FKB=20, BKB=0, Size=6.0, X=17.0, Y=0.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=2, Part=0, Bone=hash40("hammer1"), Damage=1.0, Angle=95, KBG=100, FKB=40, BKB=0, Size=4.0, X=9.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
            ATTACK(ID=3, Part=0, Bone=hash40("hammer1"), Damage=1.0, Angle=95, KBG=100, FKB=40, BKB=0, Size=4.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=1)
    }
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=5.0, Angle=60, KBG=167, FKB=0, BKB=55, Size=6.5, X=17.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=5.0, Angle=60, KBG=167, FKB=0, BKB=55, Size=6.5, X=17.0, Y=0.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=2, Part=0, Bone=hash40("hammer1"), Damage=5.0, Angle=60, KBG=167, FKB=0, BKB=55, Size=5.0, X=9.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=3, Part=0, Bone=hash40("hammer1"), Damage=5.0, Angle=60, KBG=167, FKB=0, BKB=55, Size=5.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    wait(Frames=2)
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
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "attack_air_lw",
animcmd = "game_attackairlw")]
pub fn instant_dedede_attack_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(21.0, true)
    }
    frame(Frame=7)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(Frame=22)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=270, KBG=90, FKB=0, BKB=20, Size=5.5, X=0.0, Y=-6.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=6.0, X=0.0, Y=-6.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=8.5, X=0.0, Y=-5.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    frame(Frame=25)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=44)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "special_lw",
animcmd = "game_speciallw")]
pub fn dedede_special_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(8.0, true)
    }
    frame(Frame=9)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=40.0, Angle=361, KBG=46, FKB=0, BKB=60, Size=9.0, X=16.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=30.0, Angle=361, KBG=46, FKB=0, BKB=60, Size=5.0, X=0.0, Y=7.0, Z=2.0, X2=0.0, Y2=7.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=15)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_DEDEDE,
animation = "special_lw_max",
animcmd = "game_speciallwmax")]
pub fn dedede_special_lw_max(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
		MotionModule::set_frame(8.0, true)
    }
    frame(Frame=9)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=40.0, Angle=361, KBG=46, FKB=0, BKB=60, Size=9.0, X=16.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=30.0, Angle=361, KBG=46, FKB=0, BKB=60, Size=5.0, X=0.0, Y=7.0, Z=2.0, X2=0.0, Y2=7.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=15)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
    }
    });

}
pub fn install() {
    //if CONFIG.instant_info.banjo{
    acmd::add_hooks!(
        instant_dedede_attack_11,
        instant_dedede_attack_12,
        instant_dedede_attack_dash,
        instant_dedede_attack_s3,
        instant_dedede_attack_hi3,
        instant_dedede_attack_lw3,
        instant_dedede_attack_s4_s,
        instant_dedede_attack_hi4,
        instant_dedede_attack_lw4,
        instant_dedede_attack_air_n,
        instant_dedede_attack_air_f,
        instant_dedede_attack_air_b,
        instant_dedede_attack_air_hi,
        instant_dedede_attack_air_lw,
        dedede_special_lw,
        dedede_special_lw_max
    );
    //}

}