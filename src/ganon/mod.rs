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
        rust{
            let critChance = smash::app::sv_math::rand(hash40("ganon"), 5);
            let mut damage: f32 = 24.0;
            let mut KBG: i32 = 75;
            let mut BKB: i32 = 61;
            let mut criticalSound: bool = false;
            if critChance == 0{
            damage = 48.0;
            KBG = 150;
            BKB = 122;
            criticalSound = true;
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
            let critChance = smash::app::sv_math::rand(hash40("ganon"), 5);
            let mut damage: f32 = 24.0;
            let mut KBG: i32 = 71;
            let mut BKB: i32 = 40;
            let mut criticalSound: bool = false;
            if critChance == 0{
            damage = 48.0;
            KBG = 142;
            BKB = 80;
            criticalSound = true;
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
pub fn install() {
    acmd::add_hooks!(
        instant_ganon_attack_s4,
        instant_ganon_attack_hi4
    );
}
