use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use crate::config::CONFIG;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_11",
    animcmd = "game_attack11")]
pub fn instant_byleth_attack_11(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(3.0)
		}
        frame(4)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 8.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 180, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 361, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH)
		}
		frame(6)
		if (is_excute) {
			AttackModule::clear_all()
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		frame(16)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_12",
    animcmd = "game_attack12")]
pub fn instant_byleth_attack_12(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(3.0)
		}
        frame(4)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 2.0, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 2.0, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
			AttackModule::set_add_reaction_frame(/*ID*/ 0, /*Frames*/ 7.0, /*Unk*/ false)
			AttackModule::set_add_reaction_frame(/*ID*/ 1, /*Frames*/ 7.0, /*Unk*/ false)
		}
		frame(6)
		if (is_excute) {
			AttackModule::clear_all()
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
		}
		frame(10)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_13",
    animcmd = "game_attack13")]
pub fn instant_byleth_attack_13(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(5.0)
		}
        frame(5)
		MotionModule::set_rate(1.0)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.5, /*Angle*/ 361, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 2.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.5, /*Angle*/ 361, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 2.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_dash",
    animcmd = "game_attackdash")]
pub fn instant_byleth_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(9.0)
		}
        frame(9)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 3.2, /*X*/ -0.6, /*Y*/ 0.0, /*Z*/ -3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 4.0, /*X*/ 6.0, /*Y*/ -2.0, /*Z*/ -2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 4.0, /*X*/ 10.5, /*Y*/ -2.0, /*Z*/ -2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
		}
		frame(10)
		if (is_excute) {
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 13.0, /*X2*/ 0.0, /*Y2*/ 7.0, /*Z2*/ 16.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 4, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 4.5, /*X2*/ 0.0, /*Y2*/ 6.0, /*Z2*/ 4.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
		}
		frame(12)
		if (is_excute) {
			AttackModule::clear_all()
			FighterAreaModuleImpl::enable_fix_jostle_area(3.0, 4.0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_s3_s",
    animcmd = "game_attacks3")]
pub fn instant_byleth_attack_s3(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(10.0)
		}
		frame(10)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 11.0, /*Angle*/ 35, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 11.0, /*Angle*/ 35, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 3.5, /*X*/ 3.0, /*Y*/ 1.0, /*Z*/ -2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 11.0, /*Angle*/ 35, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 3.5, /*X*/ 7.0, /*Y*/ 1.0, /*Z*/ -2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 4, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 11.0, /*Angle*/ 35, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 3.5, /*X*/ 11.5, /*Y*/ 1.0, /*Z*/ -2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
		}
		frame(12)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 11.0, /*Angle*/ 35, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 10.0, /*X2*/ 0.0, /*Y2*/ 7.5, /*Z2*/ 16.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			AttackModule::clear(/*ID*/ 1, false)
			AttackModule::clear(/*ID*/ 2, false)
			AttackModule::clear(/*ID*/ 3, false)
			AttackModule::clear(/*ID*/ 4, false)
		}
		frame(13)
		if (is_excute) {
			AttackModule::clear_all();
			FighterAreaModuleImpl::enable_fix_jostle_area(3.0, 4.0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn instant_byleth_attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(9.0)
		}
        frame(9)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 10.0, /*Angle*/ 92, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 10.0, /*Angle*/ 96, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -1.5, /*Z*/ -2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 10.0, /*Angle*/ 96, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ -1.5, /*Z*/ -2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 10.0, /*Angle*/ 101, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 3.5, /*X*/ 8.0, /*Y*/ -1.5, /*Z*/ -2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 4, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 10.0, /*Angle*/ 101, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 3.5, /*X*/ 12.0, /*Y*/ -1.5, /*Z*/ -2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
		}
		frame(16)
		if (is_excute) {
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn instant_byleth_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(13.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0)
			ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::phx::Hash40::new("attack_lw3"), false, 0.0)
		}
		frame(13)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 8.0, /*Angle*/ 93, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 2.8, /*Z*/ 10.0, /*X2*/ 0.0, /*Y2*/ 2.8, /*Z2*/ 12.5, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 8.0, /*Angle*/ 96, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 2.8, /*Z*/ 17.0, /*X2*/ 0.0, /*Y2*/ 2.8, /*Z2*/ 25.5, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
		}
		frame(16)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(59)
		if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, 0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_s4_hi",
    animcmd = "game_attacks4hi")]
pub fn instant_byleth_attack_s4_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(13.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0)
		}
		frame(14)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(15)
		if (is_excute) {
			MotionModule::set_rate(10.0)
		}
		frame(25)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 1.6, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 0.0, /*X2*/ 0.0, /*Y2*/ 13.0, /*Z2*/ 0.0, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 19.5, /*Angle*/ 36, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.6, /*X*/ -0.5, /*Y*/ 16.5, /*Z*/ 0.0, /*X2*/ -0.5, /*Y2*/ 23.0, /*Z2*/ 0.0, /*Hitlag*/ 1.35, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 19.5, /*Angle*/ 36, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ -0.5, /*Y*/ 24.0, /*Z*/ -0.3, /*X2*/ -0.5, /*Y2*/ 26.5, /*Z2*/ -0.3, /*Hitlag*/ 1.35, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_OBJECT)
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(85)
		if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, 0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_s4_s",
    animcmd = "game_attacks4")]
pub fn instant_byleth_attack_s4_s(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(13.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0)
		}
		frame(14)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(15)
		if (is_excute) {
			MotionModule::set_rate(10.0)
		}
		frame(25)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 1.6, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 0.0, /*X2*/ 0.0, /*Y2*/ 13.0, /*Z2*/ 0.0, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 19.5, /*Angle*/ 36, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.6, /*X*/ -0.5, /*Y*/ 16.5, /*Z*/ 0.0, /*X2*/ -0.5, /*Y2*/ 23.0, /*Z2*/ 0.0, /*Hitlag*/ 1.35, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 19.5, /*Angle*/ 36, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ -0.5, /*Y*/ 24.0, /*Z*/ -0.3, /*X2*/ -0.5, /*Y2*/ 26.5, /*Z2*/ -0.3, /*Hitlag*/ 1.35, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_OBJECT)
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(85)
		if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, 0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_s4_lw",
    animcmd = "game_attacks4lw")]
pub fn instant_byleth_attack_s4_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(13.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0)
		}
		frame(14)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(15)
		if (is_excute) {
			MotionModule::set_rate(10.0)
		}
		frame(25)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 1.6, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 0.0, /*X2*/ 0.0, /*Y2*/ 13.0, /*Z2*/ 0.0, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 19.5, /*Angle*/ 36, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.6, /*X*/ -0.5, /*Y*/ 16.5, /*Z*/ 0.0, /*X2*/ -0.5, /*Y2*/ 23.0, /*Z2*/ 0.0, /*Hitlag*/ 1.35, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 19.5, /*Angle*/ 36, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.0, /*X*/ -0.5, /*Y*/ 24.0, /*Z*/ -0.3, /*X2*/ -0.5, /*Y2*/ 26.5, /*Z2*/ -0.3, /*Hitlag*/ 1.35, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_OBJECT)
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(85)
		if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, 0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
pub fn instant_byleth_attack_hi4(fighter: &mut L2CFighterCommon) {
    let mut _vector = smash::phx::Vector2f {x: 3.0, y: 27.0};
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(3.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0)
			ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::phx::Hash40::new("attack_hi4"), false, 0.0)
		}
		frame(3)
		if (is_excute) {
            MotionModule::set_rate(1.0)
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            rust {
                let _frame : f32 = WorkModule::get_float(module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME) as f32;
            }
            ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::phx::Hash40::new("attack_hi4"), false, _frame)
        }
		frame(8)
		if (is_excute) {
			MotionModule::set_rate(5.0)
		}
		frame(13)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 2.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 9.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			//AttackModule::set_vec_target_pos(0, hash40("top"), 3.0, 27.0, 8.0)
            AttackModule::set_no_finish_camera(0, true, false)
            //AttackModule::set_vec_target_pos(0, smash::phx::Hash40::new("top"), _vector, 8, false)
		}
		frame(14)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 2.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 23.5, /*Z*/ 1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			//AttackModule::set_vec_target_pos(0, hash40("top"), 3.0, 27.0, 8.0);
			AttackModule::set_no_finish_camera(0, true, false);
		}
		frame(17)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 2.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 26.0, /*Z*/ 2.0, /*X2*/ 0.0, /*Y2*/ 26.0, /*Z2*/ 2.0, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 2.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 2.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 26.5, /*Z*/ -6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 2.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 26.5, /*Z*/ 10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
		}
		frame(28)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 79, /*KBG*/ 104, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 26.0, /*Z*/ 2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 1, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 79, /*KBG*/ 104, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 2, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 79, /*KBG*/ 104, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 26.5, /*Z*/ -6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 3, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 79, /*KBG*/ 104, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 26.5, /*Z*/ 10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
		}
		frame(30)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(79)
		if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, 0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
pub fn instant_byleth_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(3.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, 0)
			ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, smash::phx::Hash40::new("attack_lw4"), false, -69491906.0)
		}
		frame(3)
		if (is_excute) {
            MotionModule::set_rate(1.0)
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            rust {
                let _frame : f32 = WorkModule::get_float(module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME) as f32;
            }
            ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, smash::phx::Hash40::new("attack_lw4"), false, _frame)
        }
        frame(4)
        if (is_excute) {
            MotionModule::set_rate(4.0)
        }
		frame(16)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 23.0, /*Angle*/ 63, /*KBG*/ 62, /*FKB*/ 0, /*BKB*/ 73, /*Size*/ 3.7, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 23.0, /*Angle*/ 63, /*KBG*/ 62, /*FKB*/ 0, /*BKB*/ 73, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT)
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all()
		}
		wait(1)
		if (is_excute) {
			MotionModule::set_rate(6.0)
		}
		frame(26)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 23.0, /*Angle*/ 63, /*KBG*/ 62, /*FKB*/ 0, /*BKB*/ 73, /*Size*/ 3.7, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 23.0, /*Angle*/ 63, /*KBG*/ 62, /*FKB*/ 0, /*BKB*/ 73, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT)
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(78)
		if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, 0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
pub fn instant_byleth_attack_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(6.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_BOW, false, 0)
			ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_BOW, smash::phx::Hash40::new("attack_air_n"), false, 0.0)
			FighterAreaModuleImpl::enable_fix_jostle_area(3.0, 3.0)
		}
		frame(6);
		if (is_excute) {
			MotionModule::set_rate(1.0)
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 2.5, /*Angle*/ 120, /*KBG*/ 100, /*FKB*/ 45, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ -4.0, /*Y*/ -2.8, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 2.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ -4.0, /*Y*/ -2.8, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 2.5, /*Angle*/ 94, /*KBG*/ 100, /*FKB*/ 45, /*BKB*/ 0, /*Size*/ 2.7, /*X*/ -4.0, /*Y*/ 1.3, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 2.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 2.7, /*X*/ -4.0, /*Y*/ 1.3, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 4, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 2.5, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 35, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 7, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
		}
		frame(27)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(28)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 51, /*KBG*/ 150, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
		}
		frame(29)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(39)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(53)
		if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_BOW, 0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn instant_byleth_attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(11.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0)
		}
		frame(3)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(12)
		FT_MOTION_RATE(1.0)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 8.5, /*Angle*/ 48, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.0, /*X*/ -0.5, /*Y*/ 4.0, /*Z*/ -0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 8.5, /*Angle*/ 48, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 2.4, /*X*/ -0.5, /*Y*/ 9.2, /*Z*/ -0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 8.5, /*Angle*/ 48, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 2.4, /*X*/ -0.5, /*Y*/ 13.5, /*Z*/ -0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 12.75, /*Angle*/ 361, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.4, /*X*/ -0.5, /*Y*/ 19.0, /*Z*/ -0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 4, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 12.75, /*Angle*/ 361, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.0, /*X*/ -0.5, /*Y*/ 25.0, /*Z*/ -0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_OBJECT)
		}
		frame(13)
		if (is_excute) {
			ATTACK(/*ID*/ 5, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 8.5, /*Angle*/ 48, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 5.0, /*X2*/ 0.0, /*Y2*/ 5.0, /*Z2*/ 13.0, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 6, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.75, /*Angle*/ 361, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 16.5, /*X2*/ 0.0, /*Y2*/ 5.0, /*Z2*/ 22.0, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_OBJECT)
		}
		frame(14)
		if (is_excute) {
			AttackModule::clear_all();
		}
		frame(36)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(53)
		if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, 0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
pub fn instant_byleth_attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(13.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0)
		}
		frame(6);
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(13)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 46, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 2.3, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ -4.0, /*X2*/ 0.0, /*Y2*/ 9.5, /*Z2*/ -12.0, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 15.0, /*Angle*/ 28, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ -17.0, /*X2*/ 0.0, /*Y2*/ 9.5, /*Z2*/ -24.0, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_OBJECT)
		}
		frame(15)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -4.0, /*X2*/ 0.0, /*Y2*/ 10.0, /*Z2*/ -12.0, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -17.0, /*X2*/ 0.0, /*Y2*/ 10.0, /*Z2*/ -23.0, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_OBJECT)
		}
		frame(16)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 1.7692, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -3.0, /*X2*/ 0.0, /*Y2*/ 10.0, /*Z2*/ -10.0, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 7.5, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.6923, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -14.0, /*X2*/ 0.0, /*Y2*/ 10.0, /*Z2*/ -21.0, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_OBJECT)
		}
		frame(18)
		if (is_excute) {
			AttackModule::clear_all();
		}
		frame(40)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(54)
		if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, 0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn instant_byleth_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(10.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0)
			ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::phx::Hash40::new("attack_air_hi"), false, 0.0)
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(1.0, 4.0, 5.0, 5.0)
		}
		frame(4)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(10)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 85, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 4.7, /*X*/ 0.0, /*Y*/ 21.5, /*Z*/ 0.0, /*X2*/ 0.0, /*Y2*/ 18.5, /*Z2*/ 0.0, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 4, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 4.7, /*X*/ 0.0, /*Y*/ 21.5, /*Z*/ 0.0, /*X2*/ 0.0, /*Y2*/ 18.5, /*Z2*/ 0.0, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 35, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 28.5, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 35, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 28.5, /*Z*/ -6.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 35, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 28.5, /*Z*/ 6.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
		}
		frame(20)
		if (is_excute) {
			AttackModule::clear(/*ID*/ 0, false);
			ATTACK(/*ID*/ 4, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 5.5, /*Angle*/ 81, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 21.5, /*Z*/ 0.0, /*X2*/ 0.0, /*Y2*/ 18.5, /*Z2*/ 0.0, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 1, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 5.5, /*Angle*/ 81, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 28.5, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 2, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 5.5, /*Angle*/ 81, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 28.5, /*Z*/ -6.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 3, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 5.5, /*Angle*/ 81, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 28.5, /*Z*/ 6.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
		}
		frame(24)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(48)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(65)
		if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, 0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn instant_byleth_attack_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(22.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, 0)
			ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, smash::phx::Hash40::new("attack_air_lw"), false, 0.0)
			FighterAreaModuleImpl::enable_fix_jostle_area(3.0, 3.0)
		}
		frame(2)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(22)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 19.0, /*Angle*/ 58, /*KBG*/ 57, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 4.3, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 19.0, /*Angle*/ 72, /*KBG*/ 52, /*FKB*/ 0, /*BKB*/ 97, /*Size*/ 4.3, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 4, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 19.0, /*Angle*/ 58, /*KBG*/ 57, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 5, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 19.0, /*Angle*/ 72, /*KBG*/ 52, /*FKB*/ 0, /*BKB*/ 97, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ -3.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 19.0, /*Angle*/ 270, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 19.0, /*Angle*/ 270, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 97, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 13.5, /*Z*/ 2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT)
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(52)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(60)
		if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, 0)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "special_n_start",
    animcmd = "game_specialnstart")]
pub fn instant_byleth_special_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(50.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_BOW, false, 0)
		}
		if (is_excute) {
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, false, 0)
		}
		frame(28)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_TURN)
		}
		frame(44)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_TURN)
		}
		frame(45)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_CAN_SHOOT)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "special_air_n_start",
    animcmd = "game_specialairnstart")]
pub fn instant_byleth_special_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(50.0)
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_BOW, false, 0)
		}
		if (is_excute) {
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, false, 0)
		}
		frame(28)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_TURN)
		}
		frame(44)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_TURN)
		}
		frame(45)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_CAN_SHOOT)
		}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "special_s_f",
    animcmd = "game_specialsf")]
pub fn instant_byleth_special_s(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            MotionModule::set_rate(9.0)
        }
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_F_REQUEST)
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=9.5, X2=0.0, Y2=10.0, Z2=9.5, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=9)
        if(is_excute){
            MotionModule::set_rate(1.0)
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=3.5, X=0.0, Y=25.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=3.5, X=0.0, Y=19.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=5.5, X=0.0, Y=6.0, Z=22.0, X2=0.0, Y2=11.0, Z2=22.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=2.0, X=0.0, Y=8.5, Z=28.0, X2=0.0, Y2=14.0, Z2=28.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=3.0, X=0.0, Y=13.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=3.0, X=0.0, Y=8.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=11.0, X2=0.0, Y2=10.0, Z2=11.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=11.0, X2=0.0, Y2=14.0, Z2=11.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=5.5, X=0.0, Y=6.0, Z=22.0, X2=0.0, Y2=21.0, Z2=22.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=2.0, X=0.0, Y=8.5, Z=28.0, X2=0.0, Y2=18.5, Z2=28.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST)
        }
        frame(Frame=29)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST)
        }
        frame(Frame=53)
        if(is_excute){
            //ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "special_air_s_f",
    animcmd = "game_specialairsf")]
pub fn instant_byleth_special_air_s(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            MotionModule::set_rate(9.0)
            SET_SPEED_EX(0, 0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(10.0, 5.0, 5.0, 9.0)
        }
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_F_REQUEST)
        }
        frame(Frame=7)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(5.0, 5.0, 5.0, 5.0)
        }
        frame(Frame=9)
        if(is_excute){
            MotionModule::set_rate(1.0)
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=14.25, Angle=55, KBG=59, FKB=0, BKB=86, Size=4.5, X=0.0, Y=24.5, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=14.25, Angle=55, KBG=59, FKB=0, BKB=86, Size=5.0, X=0.0, Y=19.5, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=9.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=4.5, X=0.0, Y=11.5, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=9.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=4.5, X=0.0, Y=7.0, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING)
        }
        frame(Frame=14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST)
        }
        frame(Frame=29)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST)
        }
        frame(Frame=53)
        if(is_excute){
            //ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR)
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "special_s_f",
    animcmd = "game_specialsfdash")]
pub fn instant_byleth_special_s2(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            MotionModule::set_rate(9.0)
        }
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_UP_REQUEST)
        }
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_DOWN_REQUEST)
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=9.5, X2=0.0, Y2=10.0, Z2=9.5, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=9)
        if(is_excute){
            MotionModule::set_rate(1.0)
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=3.5, X=0.0, Y=25.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=3.5, X=0.0, Y=19.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=5.5, X=0.0, Y=6.0, Z=22.0, X2=0.0, Y2=11.0, Z2=22.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=2.0, X=0.0, Y=8.5, Z=28.0, X2=0.0, Y2=14.0, Z2=28.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=3.0, X=0.0, Y=13.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=3.0, X=0.0, Y=8.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=11.0, X2=0.0, Y2=10.0, Z2=11.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=11.0, X2=0.0, Y2=14.0, Z2=11.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=5.5, X=0.0, Y=6.0, Z=22.0, X2=0.0, Y2=21.0, Z2=22.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=2.0, X=0.0, Y=8.5, Z=28.0, X2=0.0, Y2=18.5, Z2=28.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST)
        }
        frame(Frame=29)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST)
        }
        frame(Frame=53)
        if(is_excute){
            //ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "special_air_s_f",
    animcmd = "game_specialairsfdash")]
pub fn instant_byleth_special_air_s2(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            MotionModule::set_rate(9.0)
            SET_SPEED_EX(0, 0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(10.0, 5.0, 5.0, 9.0)
        }
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_UP_REQUEST)
        }
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_DOWN_REQUEST)
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(5.0, 5.0, 5.0, 5.0)
        }
        frame(Frame=9)
        if(is_excute){
            MotionModule::set_rate(1.0)
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=14.25, Angle=55, KBG=59, FKB=0, BKB=86, Size=4.5, X=0.0, Y=24.5, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=14.25, Angle=55, KBG=59, FKB=0, BKB=86, Size=5.0, X=0.0, Y=19.5, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=9.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=4.5, X=0.0, Y=11.5, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=9.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=4.5, X=0.0, Y=7.0, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING)
        }
        frame(Frame=14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST)
        }
        frame(Frame=29)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST)
        }
        frame(Frame=53)
        if(is_excute){
            //ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR)
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "special_lw",
    animcmd = "game_speciallw")]
pub fn instant_byleth_special_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(3.0, 3.0)
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1)
            MotionModule::set_rate(30.0)
        }
        frame(Frame=12)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK)
        }
        frame(Frame=34)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_START_SUPER_ARMOR)
        }
        frame(Frame=42)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK)
        }
        frame(Frame=51)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1)
        }
        frame(Frame=60)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 30.0, /*Angle*/ 51, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.7, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT);
            MotionModule::set_rate(1.0)
        }
        frame(Frame=64)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_END_SUPER_ARMOR)
            ArticleModule::set_flag(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, true, WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND)
        }
        frame(Frame=65)
        if(is_excute){
            ArticleModule::set_flag(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND)
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING)
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2)
        }
        frame(Frame=96)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY)
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED)
        }
        frame(Frame=117)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2)
        }
        frame(Frame=118)
        if(is_excute){
            //ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_AXE)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "special_air_lw",
    animcmd = "game_specialairlw")]
pub fn instant_byleth_special_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(3.0, 3.0)
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1)
            MotionModule::set_rate(30.0)
        }
        frame(Frame=12)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK)
        }
        frame(Frame=34)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_START_SUPER_ARMOR)
        }
        frame(Frame=42)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK)
        }
        frame(Frame=51)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1)
        }
        frame(Frame=60)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 30.0, /*Angle*/ 51, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.7, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 25, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ ATTACK_REGION_OBJECT);
            MotionModule::set_rate(1.0)
        }
        frame(Frame=64)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_END_SUPER_ARMOR)
            ArticleModule::set_flag(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, true, WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND)
        }
        frame(Frame=65)
        if(is_excute){
            ArticleModule::set_flag(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND)
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING)
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2)
        }
        frame(Frame=96)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY)
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED)
        }
        frame(Frame=117)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2)
        }
        frame(Frame=118)
        if(is_excute){
            //ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_AXE)
        }
    });
}

pub fn install() {
    if CONFIG.instant_info.byleth{
        acmd::add_hooks!(
        instant_byleth_attack_11,
        instant_byleth_attack_12,
        instant_byleth_attack_13,
        instant_byleth_attack_dash,
        instant_byleth_attack_s3,
        instant_byleth_attack_hi3,
        instant_byleth_attack_lw3,
        instant_byleth_attack_s4_hi,
        instant_byleth_attack_s4_s,
        instant_byleth_attack_s4_lw,
        instant_byleth_attack_hi4,
        instant_byleth_attack_lw4,
        instant_byleth_attack_air_n,
        instant_byleth_attack_air_f,
        instant_byleth_attack_air_b,
        instant_byleth_attack_air_hi,
        instant_byleth_attack_air_lw,
        instant_byleth_special_n,
        instant_byleth_special_air_n,
        //instant_byleth_special_s,
        //instant_byleth_special_air_s,
        //instant_byleth_special_s2,
        //instant_byleth_special_air_s2,
        instant_byleth_special_lw,
        instant_byleth_special_air_lw
    );
    }

}