use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon,L2CFighterBase};
use acmd::{acmd, acmd_func};

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SZEROSUIT,
animation = "attack_11",
animcmd = "game_attack11")]
pub fn zss_attack_11(fighter: &mut L2CFighterCommon) {
    acmd!({
    frame(Frame=1)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=50, FKB=0, BKB=20, Size=2.0, X=0.0, Y=10.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=33, FKB=0, BKB=15, Size=2.3, X=0.0, Y=10.0, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.5, Angle=180, KBG=20, FKB=0, BKB=10, Size=2.5, X=0.0, Y=10.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.5, Angle=361, KBG=20, FKB=0, BKB=10, Size=2.5, X=0.0, Y=10.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        //AttackModule::set_add_reaction_frame(ID=0, Frames=10, Unk=false)
        //AttackModule::set_add_reaction_frame(ID=1, Frames=6, Unk=false)
        //AttackModule::set_add_reaction_frame(ID=2, Frames=2, Unk=false)
        //AttackModule::set_add_reaction_frame(ID=3, Frames=2, Unk=false)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    frame(Frame=8)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
    }
    });

}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SZEROSUIT,
animation = "attack_12",
animcmd = "game_attack12")]
pub fn zss_attack_12(fighter: &mut L2CFighterCommon) {
    acmd!({
    frame(Frame=2)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.5, Angle=361, KBG=40, FKB=0, BKB=30, Size=2.2, X=0.0, Y=9.5, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.5, Angle=361, KBG=40, FKB=0, BKB=25, Size=2.8, X=0.0, Y=9.8, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.5, Angle=361, KBG=30, FKB=0, BKB=20, Size=3.5, X=0.0, Y=10.0, Z=8.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
    });


}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SZEROSUIT,
animation = "attack_lw3",
animcmd = "game_attacklw3")]
pub fn instant_zss_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
	    MotionModule::set_rate(6.0)
		}
	frame(Frame=7)
	if(is_excute){
	    MotionModule::set_rate(1.0)
		}
    frame(Frame=8)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=10.0, Angle=70, KBG=85, FKB=0, BKB=60, Size=4.2, X=2.7, Y=0.0, Z=0.0, X2=-2.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=8.0, Angle=70, KBG=85, FKB=0, BKB=60, Size=3.8, X=5.2, Y=0.0, Z=0.0, X2=-2.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SZEROSUIT,
animation = "attack_hi4",
animcmd = "game_attackhi4")]
pub fn zss_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_execute){
    ArticleModule::generate_article(FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP,false,0)
    }
    frame(Frame=5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
    }
    frame(Frame=10)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=110, KBG=100, FKB=150, BKB=0, Size=6.0, X=0.0, Y=10.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    for(7 Iterations){
        wait(Frames=2)
        if(is_excute){
            ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=1.3, Angle=110, KBG=25, FKB=0, BKB=45, Size=5.2, X=0.0, Y=22.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
            ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=1.3, Angle=100, KBG=25, FKB=0, BKB=45, Size=5.6, X=0.0, Y=25.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
            ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=1.3, Angle=255, KBG=25, FKB=0, BKB=45, Size=6.5, X=0.0, Y=30.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
    }
    frame(Frame=28)
    if(is_excute){
        ATTACK(ID=1, Part=2, Bone=hash40("top"), Damage=5.0, Angle=80, KBG=212, FKB=0, BKB=45, Size=6.0, X=0.0, Y=17.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        ATTACK(ID=2, Part=2, Bone=hash40("top"), Damage=5.0, Angle=80, KBG=212, FKB=0, BKB=45, Size=7.5, X=0.0, Y=25.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        ATTACK(ID=3, Part=2, Bone=hash40("top"), Damage=5.0, Angle=80, KBG=212, FKB=0, BKB=45, Size=9.6, X=0.0, Y=34.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=42)
    if(is_excute){
        ArticleModule::remove_exist(FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
    }
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SZEROSUIT,
animation = "special_s",
animcmd = "game_specials")]
pub fn instant_zss_special_s(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
			MotionModule::set_rate(20.0)
		}
    frame(Frame=21)
    if(is_excute){
	MotionModule::set_rate(1.0)
		}
    frame(Frame=22)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=20, KBG=100, FKB=50, BKB=0, Size=4.0, X=0.0, Y=8.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.2, Angle=10, KBG=100, FKB=80, BKB=0, Size=3.5, X=0.0, Y=8.0, Z=19.0, X2=0.0, Y2=8.0, Z2=17.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        AttackModule::set_no_damage_fly_smoke_all(true, false)
    }
    frame(Frame=24)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=17, KBG=100, FKB=80, BKB=0, Size=4.0, X=0.0, Y=8.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=3.2, Angle=17, KBG=100, FKB=60, BKB=0, Size=3.5, X=0.0, Y=8.0, Z=23.0, X2=0.0, Y2=8.0, Z2=21.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        AttackModule::set_no_damage_fly_smoke_all(true, false)
    }
    frame(Frame=26)
    if(is_excute){
        AttackModule::clear(ID=0,false)
        ATTACK(ID=1, Part=2, Bone=hash40("top"), Damage=3.2, Angle=15, KBG=100, FKB=65, BKB=0, Size=3.7, X=0.0, Y=8.0, Z=27.0, X2=0.0, Y2=8.0, Z2=25.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        AttackModule::set_no_damage_fly_smoke_all(true, false)
    }
    frame(Frame=28)
    if(is_excute){
        ATTACK(ID=1, Part=3, Bone=hash40("top"), Damage=3.2, Angle=15, KBG=90, FKB=60, BKB=0, Size=4.0, X=0.0, Y=8.0, Z=30.0, X2=0.0, Y2=8.0, Z2=26.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        AttackModule::set_no_damage_fly_smoke_all(true, false)
    }
    frame(Frame=30)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_SZEROSUIT_STATUS_SPECIAL_S_FLAG_S2)
        AttackModule::clear_all()
    }
    frame(Frame=31)
    if(is_excute){
        ATTACK(ID=0, Part=4, Bone=hash40("top"), Damage=11.0, Angle=55, KBG=138, FKB=0, BKB=47, Size=6.0, X=0.0, Y=8.5, Z=38.0, X2=0.0, Y2=8.5, Z2=35.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
    }
    wait(Frames=5)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SZEROSUIT,
animation = "special_air_s",
animcmd = "game_specialairs")]
pub fn instant_zss_special_air_s(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
			MotionModule::set_rate(20.0)
		}
    frame(Frame=20)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)
    }
    frame(Frame=21)
    if(is_excute){
        MotionModule::set_rate(1.0)
        WorkModule::off_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)
    }
    frame(Frame=22)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.2, Angle=367, KBG=100, FKB=85, BKB=0, Size=3.5, X=0.0, Y=8.0, Z=19.0, X2=0.0, Y2=8.0, Z2=17.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=0, KBG=100, FKB=95, BKB=0, Size=3.0, X=0.0, Y=8.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        AttackModule::set_no_damage_fly_smoke_all(true, false)
    }
    frame(Frame=24)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=0, KBG=100, FKB=100, BKB=0, Size=3.0, X=0.0, Y=8.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=2.2, Angle=345, KBG=100, FKB=85, BKB=0, Size=3.5, X=0.0, Y=8.0, Z=25.0, X2=0.0, Y2=8.0, Z2=17.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        AttackModule::set_no_damage_fly_smoke_all(true, false)
    }
    frame(Frame=26)
    if(is_excute){
        AttackModule::clear(ID=0,false)
        ATTACK(ID=1, Part=2, Bone=hash40("top"), Damage=3.2, Angle=10, KBG=100, FKB=85, BKB=0, Size=3.7, X=0.0, Y=8.0, Z=28.0, X2=0.0, Y2=8.0, Z2=25.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        AttackModule::set_no_damage_fly_smoke_all(true, false)
    }
    frame(Frame=28)
    if(is_excute){
        ATTACK(ID=1, Part=3, Bone=hash40("top"), Damage=3.2, Angle=345, KBG=100, FKB=85, BKB=0, Size=4.0, X=0.0, Y=8.0, Z=30.0, X2=0.0, Y2=8.0, Z2=28.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        AttackModule::set_no_damage_fly_smoke_all(true, false)
    }
    frame(Frame=30)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_SZEROSUIT_STATUS_SPECIAL_S_FLAG_S2)
        AttackModule::clear_all()
    }
    frame(Frame=31)
    if(is_excute){
        ATTACK(ID=0, Part=4, Bone=hash40("top"), Damage=11.0, Angle=60, KBG=105, FKB=0, BKB=45, Size=6.6, X=0.0, Y=8.5, Z=38.0, X2=0.0, Y2=8.5, Z2=34.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
    }
    wait(Frames=5)
    if(is_excute){
        AttackModule::clear_all()
    }
    });


}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SZEROSUIT,
animation = "special_s_2",
animcmd = "game_specials2")]
pub fn instant_zss_special_s2(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
			MotionModule::set_rate(29.0)
		}
	frame(Frame=30)
	if(is_excute){
			MotionModule::set_rate(1.0)
		}
    frame(Frame=31)
    if(is_excute){
        ATTACK(ID=0, Part=5, Bone=hash40("top"), Damage=6.0, Angle=107, KBG=62, FKB=0, BKB=98, Size=4.5, X=0.0, Y=8.0, Z=36.0, X2=0.0, Y2=8.0, Z2=33.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        //AttackModule::set_add_reaction_frame(ID=0, Frames=4, Unk=false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=37)
    if(is_excute){
        ATTACK(ID=0, Part=5, Bone=hash40("top"), Damage=6.0, Angle=95, KBG=62, FKB=0, BKB=98, Size=4.5, X=0.0, Y=7.0, Z=24.0, X2=0.0, Y2=7.0, Z2=24.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        //AttackModule::set_add_reaction_frame(ID=0, Frames=4, Unk=false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=42)
    if(is_excute){
        ATTACK(ID=0, Part=5, Bone=hash40("top"), Damage=6.0, Angle=80, KBG=62, FKB=0, BKB=98, Size=3.5, X=0.0, Y=16.0, Z=16.0, X2=0.0, Y2=16.0, Z2=16.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        //AttackModule::set_add_reaction_frame(ID=0, Frames=4, Unk=false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=46)
    if(is_excute){
        ATTACK(ID=0, Part=5, Bone=hash40("top"), Damage=6.0, Angle=75, KBG=62, FKB=0, BKB=98, Size=4.0, X=0.0, Y=23.0, Z=6.0, X2=0.0, Y2=23.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
        ATTACK(ID=1, Part=5, Bone=hash40("top"), Damage=6.0, Angle=75, KBG=62, FKB=0, BKB=98, Size=2.5, X=0.0, Y=23.5, Z=6.0, X2=0.0, Y2=23.5, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
       // AttackModule::set_add_reaction_frame(ID=0, Frames=4, Unk=false)
        //AttackModule::set_add_reaction_frame(ID=1, Frames=4, Unk=false)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_SZEROSUIT,
animation = "special_air_s_2",
animcmd = "game_specialairs2")]
pub fn instant_zss_special_air_s2(fighter: &mut L2CFighterCommon) {
    acmd!({
    if(is_excute){
			MotionModule::set_rate(29.0)
		}
	frame(Frame=30)
	if(is_excute){
			MotionModule::set_rate(1.0)
		}
    frame(Frame=31)
    if(is_excute){
        ATTACK(ID=0, Part=5, Bone=hash40("top"), Damage=4.0, Angle=108, KBG=105, FKB=0, BKB=60, Size=5.0, X=0.0, Y=8.0, Z=36.0, X2=0.0, Y2=8.0, Z2=36.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=37)
    if(is_excute){
        ATTACK(ID=0, Part=5, Bone=hash40("top"), Damage=4.0, Angle=100, KBG=105, FKB=0, BKB=60, Size=5.0, X=0.0, Y=2.0, Z=26.0, X2=0.0, Y2=2.0, Z2=26.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=42)
    if(is_excute){
        ATTACK(ID=0, Part=6, Bone=hash40("top"), Damage=4.0, Angle=80, KBG=105, FKB=0, BKB=60, Size=5.0, X=0.0, Y=16.0, Z=16.0, X2=0.0, Y2=16.0, Z2=16.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=46)
    if(is_excute){
        ATTACK(ID=0, Part=7, Bone=hash40("top"), Damage=4.0, Angle=75, KBG=105, FKB=0, BKB=60, Size=5.0, X=0.0, Y=24.0, Z=5.0, X2=0.0, Y2=24.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_WHIP)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    });



}
pub fn install() {
    acmd::add_hooks!(
    zss_attack_11,
    zss_attack_12,
    instant_zss_attack_lw3,
    instant_zss_special_s,
    instant_zss_special_air_s,
    instant_zss_special_s2,
    instant_zss_special_air_s2
    //zss_attack_hi4
    );
}
