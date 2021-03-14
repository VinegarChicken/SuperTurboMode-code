use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use crate::config::CONFIG;
use smash::app::lua_bind::*;

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_s4_s",
animcmd = "game_attacks4")]
pub fn instant_ganon_attack_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust{
            let critChance = smash::app::sv_math::rand(hash40("ganon"), CONFIG.ganon_changes.rng_ganon_f_smash_chances_upper.parse().unwrap());
            let mut damage: f32 = 24.0;
            let mut KBG: i32 = 75;
            let mut BKB: i32 = 61;
            let mut criticalSound: bool = false;
            if CONFIG.ganon_changes.rng_ganon_f_smash{
                if critChance == 0{
                    damage = 48.0;
                    KBG = 150;
                    BKB = 122;
                    criticalSound = true;
                }
               }
            }

            if(is_excute){
                MotionModule::set_rate(14.0)
                ArticleModule::remove_exist(0,smash::app::ArticleOperationTarget(*FIGHTER_GANON_GENERATE_ARTICLE_SWORD))
                ArticleModule::generate_article(FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false,0)
                }
    frame(Frame=15)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=28)
    if(is_excute){
        MotionModule::set_rate(1.0)
    }
    frame(Frame=29)
    if(is_excute){
        AttackModule::set_is_critical_attack(criticalSound);
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=damage, Angle=40, KBG=KBG, FKB=0, BKB=BKB, Size=4.5, X=0.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=damage, Angle=40, KBG=KBG, FKB=0, BKB=BKB, Size=4.0, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
    }
    frame(Frame=30)
    if(is_excute){
        AttackModule::set_is_critical_attack(criticalSound);
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=damage, Angle=40, KBG=KBG, FKB=0, BKB=BKB, Size=5.0, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=damage, Angle=40, KBG=KBG, FKB=0, BKB=BKB, Size=4.5, X=0.0, Y=14.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=damage, Angle=40, KBG=KBG, FKB=0, BKB=BKB, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        //AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)
    }
    wait(Frames=2)
    if(is_excute){
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
    rust{
            let critChance = smash::app::sv_math::rand(hash40("ganon"), CONFIG.ganon_changes.rng_ganon_f_smash_chances_upper.parse().unwrap());
            let mut damage: f32 = 24.0;
            let mut KBG: i32 = 71;
            let mut BKB: i32 = 40;
            let mut criticalSound: bool = false;
            if CONFIG.ganon_changes.rng_ganon_u_smash{
                if critChance == 0{
                    damage = 48.0;
                    KBG = 142;
                    BKB = 80;
                    criticalSound = true;
                }
            }
           }

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
        AttackModule::set_is_critical_attack(criticalSound);
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=damage, Angle=85, KBG=KBG, FKB=0, BKB=BKB, Size=5.0, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=damage, Angle=78, KBG=KBG, FKB=0, BKB=BKB, Size=4.5, X=0.0, Y=14.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=damage, Angle=75, KBG=KBG, FKB=0, BKB=BKB, Size=4.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
    }
    wait(Frames=6)
    if(is_excute){
        AttackModule::clear_all()
    }
    });


}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_11",
animcmd = "game_attack11")]
pub fn instant_ganon_attack_11(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(7.0)
        }
        frame(7)
		if (is_excute) {
            MotionModule::set_rate(1.0)
        }
		frame(8)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 41, /*Size*/ 4.4, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 41, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 19.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 74, /*FKB*/ 0, /*BKB*/ 41, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_PUNCH)
		}
		wait(2)
		if (is_excute) {
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_dash",
animcmd = "game_attackdash")]
pub fn instant_ganon_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(10.0)
		}
        frame(10)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 15.0, /*Angle*/ 70, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_BODY)
		}
		wait(3)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 11.0, /*Angle*/ 80, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_BODY)
		}
		wait(7)
		if (is_excute) {
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_s3_s",
animcmd = "game_attacks3")]
pub fn instant_ganon_attack_s3s(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(9.0)
		}
        frame(10)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 13.0, /*Angle*/ 22, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 31, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 13.0, /*Angle*/ 22, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 31, /*Size*/ 5.0, /*X*/ -0.5, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 14.0, /*Angle*/ 22, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 31, /*Size*/ 5.5, /*X*/ 5.3, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
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
animation = "attack_hi3",
animcmd = "game_attackhi3")]
pub fn instant_ganon_attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(30.0)
		}
        frame(6)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 0.0, /*Angle*/ 180, /*KBG*/ 100, /*FKB*/ 22, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 30.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 6, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_NONE, /*Type*/ ATTACK_REGION_NONE)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 0.0, /*Angle*/ 180, /*KBG*/ 100, /*FKB*/ 22, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 25.0, /*Z*/ 30.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 6, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_NONE, /*Type*/ ATTACK_REGION_NONE)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 0.0, /*Angle*/ 0, /*KBG*/ 100, /*FKB*/ 10, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 6, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_NONE, /*Type*/ ATTACK_REGION_NONE)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 0.0, /*Angle*/ 0, /*KBG*/ 100, /*FKB*/ 10, /*BKB*/ 0, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 25.0, /*Z*/ 6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 6, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_NONE, /*Type*/ ATTACK_REGION_NONE)
		}
		frame(53)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(60)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			WorkModule::on_flag(/*Flag*/ FIGHTER_GANON_STATUS_ATTACK_WORK_FLAG_CRITICAL)
			WorkModule::set_int(3, FIGHTER_GANON_STATUS_ATTACK_WORK_INT_IGNORE_CRITICAL_ATTACK_ID)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("legr"), /*Damage*/ 24.0, /*Angle*/ 361, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 20, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("legr"), /*Damage*/ 24.0, /*Angle*/ 361, /*KBG*/ 68, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ 9.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 20, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 24.0, /*Angle*/ 70, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 20, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 24.0, /*Angle*/ 70, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 16.0, /*X2*/ 0.0, /*Y2*/ 13.0, /*Z2*/ 16.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 20, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_KICK)
		}
		wait(2)
        if (is_excute) {
            WorkModule::off_flag(/*Flag*/ FIGHTER_GANON_STATUS_ATTACK_WORK_FLAG_CRITICAL)
            ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 0, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 16.0, /*X2*/ 0.0, /*Y2*/ 22.0, /*Z2*/ 16.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 20, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_KICK)
        }
		wait(2)
		if (is_excute) {
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_lw3",
animcmd = "game_attacklw3")]
pub fn instant_ganon_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(10.0)
		}
        frame(10)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("legr"), /*Damage*/ 14.0, /*Angle*/ 60, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.35, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("legr"), /*Damage*/ 14.0, /*Angle*/ 70, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.8, /*X*/ 5.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.35, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 14.0, /*Angle*/ 80, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.8, /*X*/ 8.5, /*Y*/ -0.5, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.35, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
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
animation = "attack_lw4",
animcmd = "game_attacklw4")]
pub fn instant_ganon_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute) {
			//ArticleModule::remove_exist(FIGHTER_GANON_GENERATE_ARTICLE_SWORD, 0)
			ArticleModule::generate_article(FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, 0)
			MotionModule::set_rate(15.0)
		}
		frame(5)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(6)
		if (is_excute) {
			MotionModule::set_rate(9.0)
		}
		frame(15)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 160, /*KBG*/ 90, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 11.0, /*X2*/ 0.0, /*Y2*/ 4.0, /*Z2*/ 6.0, /*Hitlag*/ 1.0, /*SDI*/ 0.5, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 175, /*KBG*/ 92, /*FKB*/ 95, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 19.0, /*X2*/ 0.0, /*Y2*/ 4.0, /*Z2*/ 6.0, /*Hitlag*/ 1.0, /*SDI*/ 0.5, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 147, /*KBG*/ 90, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 11.0, /*X2*/ 0.0, /*Y2*/ 4.0, /*Z2*/ 6.0, /*Hitlag*/ 1.0, /*SDI*/ 0.5, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_SWORD)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 150, /*KBG*/ 92, /*FKB*/ 95, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 19.0, /*X2*/ 0.0, /*Y2*/ 4.0, /*Z2*/ 6.0, /*Hitlag*/ 1.0, /*SDI*/ 0.5, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_SWORD)
		}
		wait(4)
		if (is_excute) {
			AttackModule::clear_all()
			JostleModule::set_status(false)
		}
		frame(35)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 15.0, /*Angle*/ 35, /*KBG*/ 81, /*FKB*/ 0, /*BKB*/ 61, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 4.8, /*Z*/ -21.0, /*X2*/ 0.0, /*Y2*/ 4.8, /*Z2*/ -6.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD)
		}
		wait(4)
		if (is_excute) {
			AttackModule::clear_all()
			JostleModule::set_status(true)
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_air_n",
animcmd = "game_attackairn")]
pub fn instant_ganon_attack_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(7.0)
		}
        frame(4)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(7)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 7.0, /*Angle*/ 55, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 7.0, /*Angle*/ 55, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.3, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 7.0, /*Angle*/ 100, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.3, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
		}
		wait(2)
		if (is_excute) {
			MotionModule::set_rate(3.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 5.25, /*Angle*/ 55, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 5.25, /*Angle*/ 70, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 5.25, /*Angle*/ 55, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.3, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 5.25, /*Angle*/ 100, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.3, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
		}
		wait(4)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(20)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.8, /*X*/ 6.5, /*Y*/ 0.0, /*Z*/ -3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
		}
		wait(2)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 6.0, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ -3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 5.3, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_HEAVY, /*Type*/ ATTACK_REGION_KICK)
		}
		wait(8)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(41)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_air_f",
animcmd = "game_attackairf")]
pub fn instant_ganon_attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(13.0)
		}
        frame(7)
        if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(13)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("shoulderr"), /*Damage*/ 17.0, /*Angle*/ 361, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.0, /*X*/ -1.1, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 18.0, /*Angle*/ 361, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 5.5, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_PUNCH)
		}
		wait(7)
		if (is_excute) {
			AttackModule::clear_all()
		}
        frame(45)
        if (is_excute) {
            WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_air_b",
animcmd = "game_attackairb")]
pub fn instant_ganon_attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(10.0)
		}
		frame(7)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(10)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 17.0, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 10.4, /*Z*/ -10.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 18.5, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.1, /*Z*/ -15.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 17.0, /*Angle*/ 361, /*KBG*/ 86, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 12.6, /*Z*/ -7.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH)
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(22)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_air_hi",
animcmd = "game_attackairhi")]
pub fn instant_ganon_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(8.0)
		}
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(8)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 4.8, /*X*/ 3.2, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 5.8, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
		}
		wait(3)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 12.0, /*Angle*/ 30, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.8, /*X*/ 3.2, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 10.0, /*Angle*/ 30, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.8, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
		}
		wait(3)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 8.0, /*Angle*/ 0, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.8, /*X*/ 3.2, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 6.0, /*Angle*/ 0, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 5.8, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all()
		}
		frame(25)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "attack_air_lw",
animcmd = "game_attackairlw")]
pub fn instant_ganon_attack_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(16.0)
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(4.5, 4.5, 12.5, 0.0)
		}
		frame(4);
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(16)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 19.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 17.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_KICK)
		}
		wait(3)
		if (is_excute) {
			AttackModule::clear_all();
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(4.5, 4.5, 12.5, 11.0)
		}
		frame(32)
		if (is_excute) {
			WorkModule::off_flag(/*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "special_n",
animcmd = "game_specialn")]
pub fn instant_ganon_special_n(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(35.0)
		}
		frame(11)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN)
		}
		frame(70)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("shoulderl"), /*Damage*/ 30.0, /*Angle*/ 50, /*KBG*/ 46, /*FKB*/ 0, /*BKB*/ 120, /*Size*/ 5.0, /*X*/ 2.4, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("bust"), /*Damage*/ 30.0, /*Angle*/ 50, /*KBG*/ 46, /*FKB*/ 0, /*BKB*/ 120, /*Size*/ 4.7, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 30.0, /*Angle*/ 50, /*KBG*/ 46, /*FKB*/ 0, /*BKB*/ 120, /*Size*/ 4.8, /*X*/ 5.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
		}
		frame(74)
		if (is_excute) {
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "special_n_turn",
animcmd = "game_specialnturn")]
pub fn instant_ganon_special_n_turn(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(70.0)
		}
		frame(11)
		if (is_excute) {
			REVERSE_LR()
		}
		frame(70)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("shoulderl"), /*Damage*/ 37.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 2.4, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("bust"), /*Damage*/ 37.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.7, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 37.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.8, /*X*/ 5.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
		}
		frame(74)
		if (is_excute) {
			AttackModule::clear_all()
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "special_air_n",
animcmd = "game_specialairn")]
pub fn instant_ganon_special_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(70.0)
		}
		frame(11)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN)
		}
		frame(66)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_DIR_DECIDE)
			WorkModule::set_int(1, FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE)
		}
		frame(70)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("shoulderl"), /*Damage*/ 38.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.0, /*X*/ 2.4, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("bust"), /*Damage*/ 38.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.7, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 38.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.8, /*X*/ 5.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
		}
		frame(74)
		if (is_excute) {
			AttackModule::clear_all()
			WorkModule::set_int(2, FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE)
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "special_air_n_turn",
animcmd = "game_specialairnturn")]
pub fn instant_ganon_special_air_n_turn(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(70.0)
		}
		frame(11)
		if (is_excute) {
			REVERSE_LR()
		}
		frame(66)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_DIR_DECIDE)
			WorkModule::set_int(1, FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE)
		}
		frame(70)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("shoulderl"), /*Damage*/ 40.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 2.4, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("bust"), /*Damage*/ 40.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.7, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
			ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 40.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.8, /*X*/ 5.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH)
		}
		frame(74)
		if (is_excute) {
			AttackModule::clear_all()
			WorkModule::set_int(2, FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE)
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "special_lw",
animcmd = "game_speciallw")]
pub fn instant_ganon_special_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(16.0)
		}
		frame(10)
		if (is_excute) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(3.0, 6.0, 8.5, 9.5)
		}
		frame(13)
		if (is_excute) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(2.0, 6.0, 8.5, 10.0)
		}
		frame(16)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 14.0, /*Angle*/ 45, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.0, /*X*/ 2.7, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 16.0, /*Angle*/ 45, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 4.0, /*X*/ 7.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK)
			JostleModule::set_status(false)
		}
		wait(1)
		if (is_excute) {
			WorkModule::on_flag(/*Flag*/ FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK)
        }
        frame(24)
        if (is_excute) {
            CancelModule::enable_cancel()
        }
		frame(35)
		if (is_excute) {
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(8.0, 8.0, 8.0, 4.0)
		}
		frame(36)
		if (is_excute) {
			AttackModule::clear_all()
			JostleModule::set_status(true)
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "special_air_lw",
animcmd = "game_specialairlw")]
pub fn instant_ganon_special_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
		if (is_excute) {
			MotionModule::set_rate(16.0)
		}
		frame(16)
		if (is_excute) {
			MotionModule::set_rate(1.0)
			WorkModule::on_flag(/*Flag*/ FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK)
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 15.0, /*Angle*/ 290, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ 12.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 15.0, /*Angle*/ 290, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 8.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
			JostleModule::set_status(false);
		}
		frame(19)
		if (is_excute) {
			ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 14.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ 12.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
			ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 14.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 8.0, /*Y*/ 0.0, /*Z*/ 0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_purple"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        frame(25)
        if (is_excute) {
            CancelModule::enable_cancel()
        }
		frame(39)
		if (is_excute) {
			AttackModule::clear_all()
			JostleModule::set_status(true)
		}
    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_GANON,
animation = "special_s_start",
animcmd = "game_specialsstart")]
pub fn instant_ganon_special_s_start(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=4.0, Angle=0, KBG=10, FKB=0, BKB=100, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            MotionModule::set_rate(16.0)
        }
        frame(Frame=15)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(6.8, 6.7, 7.5, 3.8)
        }
        frame(Frame=16)
        if(is_excute){
            MotionModule::set_rate(1.0)
            CATCH(ID=0, Bone=hash40("top"), Size=5.0, X=0.0, Y=8.0, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Status=FIGHTER_STATUS_KIND_CATCHED_GANON, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=1.0, X=0.0, Y=8.0, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Status=FIGHTER_STATUS_KIND_CATCHED_GANON, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
        }
        frame(Frame=31)
        if(is_excute){
            GrabModule::clear_all()
            AttackModule::clear_all()
        }
        frame(Frame=42)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(7.0, 7.0, 7.5, 7.5)
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        instant_ganon_attack_11,
        instant_ganon_attack_dash,
        instant_ganon_attack_s3s,
        instant_ganon_attack_hi3,
        instant_ganon_attack_lw3,
        instant_ganon_attack_s4,
        instant_ganon_attack_hi4,
        instant_ganon_attack_lw4,
        instant_ganon_attack_air_n,
        instant_ganon_attack_air_f,
        instant_ganon_attack_air_b,
        instant_ganon_attack_air_hi,
        instant_ganon_attack_air_lw,
        instant_ganon_special_n,
        instant_ganon_special_n_turn,
        instant_ganon_special_air_n,
        instant_ganon_special_air_n_turn,
        instant_ganon_special_lw,
        instant_ganon_special_air_lw,
        instant_ganon_special_s_start,
    );
}

