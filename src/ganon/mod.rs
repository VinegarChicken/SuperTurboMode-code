use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_s4_s",
animcmd = "game_attacks4")]
pub fn instant_ganon_attack_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			MotionModule::set_rate(15.0)
			//ArticleModule::remove_exist(FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ARTICLE_OPE_TARGET_ALL)
			ArticleModule::generate_article(FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, 0)
		}
		frame(15)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(16)
		if (is_excute) {
			MotionModule::set_rate(12.0)
		}
		frame(28)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 24.0, /*Angle*/ 40, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 24.0, /*Angle*/ 40, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
		}
		frame(29)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 24.0, /*Angle*/ 40, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 24.0, /*Angle*/ 40, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 14.5, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 24.0, /*Angle*/ 40, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all()
		}
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_hi4",
animcmd = "game_attackhi4")]
pub fn instant_ganon_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
        MotionModule::set_rate(17.0)
        ArticleModule::remove_exist(FIGHTER_GANON_GENERATE_ARTICLE_SWORD,smash::app::ArticleOperationTarget(*FIGHTER_GANON_GENERATE_ARTICLE_SWORD))
        ArticleModule::generate_article(FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false,0)
    }
    frame(Frame=10)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=19)
    if(is_excute){
        MotionModule::set_rate(1.0)
    }
    frame(Frame=20)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=24.0, Angle=85, KBG=71, FKB=0, BKB=40, Size=5.0, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=24.0, Angle=78, KBG=71, FKB=0, BKB=40, Size=4.5, X=0.0, Y=14.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=21.0, Angle=75, KBG=75, FKB=0, BKB=40, Size=4.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
    }
    wait(Frames=6)
    if(is_excute){
        AttackModule::clear_all()
    }
    });


}
pub fn install() {
    acmd::add_hooks!(
        instant_ganon_attack_s4,
        instant_ganon_attack_hi4
    );
}
