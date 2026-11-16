//! Пиксельные значки предметов из `tools/generate_pixel_art.py`, встроенные в EXE.

macro_rules! icon {
    ($id:literal, $name:literal) => {
        (
            $id,
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../assets/textures/pixel/icons/",
                $name,
                ".png"
            )) as &'static [u8],
        )
    };
}

const ICONS: [(&str, &[u8]); 350] = [
    icon!("survival:stick", "stick"),
    icon!("survival:egg", "egg"),
    icon!("builder:clay_ball", "clay_ball"),
    icon!("builder:brick", "brick"),
    icon!("builder:string", "string"),
    icon!("builder:feather", "feather"),
    icon!("builder:raw_beef", "raw_beef"),
    icon!("builder:cooked_beef", "cooked_beef"),
    icon!("builder:raw_pork", "raw_pork"),
    icon!("builder:cooked_pork", "cooked_pork"),
    icon!("builder:raw_mutton", "raw_mutton"),
    icon!("builder:cooked_mutton", "cooked_mutton"),
    icon!("builder:raw_chicken", "raw_chicken"),
    icon!("builder:cooked_chicken", "cooked_chicken"),
    icon!("builder:rotten_meat", "rotten_meat"),
    icon!("flora:dye_white", "dye_white"),
    icon!("flora:dye_orange", "dye_orange"),
    icon!("flora:dye_magenta", "dye_magenta"),
    icon!("flora:dye_light_blue", "dye_light_blue"),
    icon!("flora:dye_yellow", "dye_yellow"),
    icon!("flora:dye_lime", "dye_lime"),
    icon!("flora:dye_pink", "dye_pink"),
    icon!("flora:dye_gray", "dye_gray"),
    icon!("flora:dye_light_gray", "dye_light_gray"),
    icon!("flora:dye_cyan", "dye_cyan"),
    icon!("flora:dye_purple", "dye_purple"),
    icon!("flora:dye_blue", "dye_blue"),
    icon!("flora:dye_brown", "dye_brown"),
    icon!("flora:dye_green", "dye_green"),
    icon!("flora:dye_red", "dye_red"),
    icon!("flora:dye_black", "dye_black"),
    icon!("mine:raw_copper", "raw_copper"),
    icon!("mine:copper_ingot", "copper_ingot"),
    icon!("mine:emerald", "emerald"),
    icon!("mine:lapis", "lapis"),
    icon!("fluid:bucket", "bucket"),
    icon!("fluid:water_bucket", "water_bucket"),
    icon!("fluid:lava_bucket", "lava_bucket"),
    icon!("voxium:saddle", "item_saddle"),
    icon!("voxium:minecart", "item_minecart"),
    icon!("voxium:chest_minecart", "item_chest_minecart"),
    icon!("voxium:furnace_minecart", "item_furnace_minecart"),
    icon!("voxium:tnt_minecart", "item_tnt_minecart"),
    icon!("voxium:hopper_minecart", "item_hopper_minecart"),
    icon!("voxium:carrot_on_a_stick", "item_carrot_on_a_stick"),
    icon!(
        "voxium:warped_fungus_on_a_stick",
        "item_warped_fungus_on_a_stick"
    ),
    icon!("voxium:elytra", "item_elytra"),
    icon!("voxium:oak_boat", "item_oak_boat"),
    icon!("voxium:oak_chest_boat", "item_oak_chest_boat"),
    icon!("voxium:spruce_boat", "item_spruce_boat"),
    icon!("voxium:spruce_chest_boat", "item_spruce_chest_boat"),
    icon!("voxium:birch_boat", "item_birch_boat"),
    icon!("voxium:birch_chest_boat", "item_birch_chest_boat"),
    icon!("voxium:jungle_boat", "item_jungle_boat"),
    icon!("voxium:jungle_chest_boat", "item_jungle_chest_boat"),
    icon!("voxium:acacia_boat", "item_acacia_boat"),
    icon!("voxium:acacia_chest_boat", "item_acacia_chest_boat"),
    icon!("voxium:cherry_boat", "item_cherry_boat"),
    icon!("voxium:cherry_chest_boat", "item_cherry_chest_boat"),
    icon!("voxium:dark_oak_boat", "item_dark_oak_boat"),
    icon!("voxium:dark_oak_chest_boat", "item_dark_oak_chest_boat"),
    icon!("voxium:mangrove_boat", "item_mangrove_boat"),
    icon!("voxium:mangrove_chest_boat", "item_mangrove_chest_boat"),
    icon!("voxium:bamboo_raft", "item_bamboo_raft"),
    icon!("voxium:bamboo_chest_raft", "item_bamboo_chest_raft"),
    icon!("voxium:turtle_helmet", "item_turtle_helmet"),
    icon!("voxium:scute", "item_scute"),
    icon!("voxium:flint_and_steel", "item_flint_and_steel"),
    icon!("voxium:apple", "item_apple"),
    icon!("voxium:bow", "item_bow"),
    icon!("voxium:arrow", "item_arrow"),
    icon!("voxium:charcoal", "item_charcoal"),
    icon!("voxium:quartz", "item_quartz"),
    icon!("voxium:amethyst_shard", "item_amethyst_shard"),
    icon!("voxium:netherite_ingot", "item_netherite_ingot"),
    icon!("voxium:netherite_scrap", "item_netherite_scrap"),
    icon!("survival:netherite_sword", "item_netherite_sword"),
    icon!("survival:netherite_shovel", "item_netherite_shovel"),
    icon!("survival:netherite_pickaxe", "item_netherite_pickaxe"),
    icon!("survival:netherite_axe", "item_netherite_axe"),
    icon!("survival:netherite_hoe", "item_netherite_hoe"),
    icon!("voxium:bowl", "item_bowl"),
    icon!("voxium:mushroom_stew", "item_mushroom_stew"),
    icon!("voxium:gunpowder", "item_gunpowder"),
    icon!("voxium:wheat_seeds", "item_wheat_seeds"),
    icon!("voxium:bread", "item_bread"),
    icon!("voxium:chainmail_helmet", "item_chainmail_helmet"),
    icon!("voxium:chainmail_chestplate", "item_chainmail_chestplate"),
    icon!("voxium:chainmail_leggings", "item_chainmail_leggings"),
    icon!("voxium:chainmail_boots", "item_chainmail_boots"),
    icon!("voxium:netherite_helmet", "item_netherite_helmet"),
    icon!("voxium:netherite_chestplate", "item_netherite_chestplate"),
    icon!("voxium:netherite_leggings", "item_netherite_leggings"),
    icon!("voxium:netherite_boots", "item_netherite_boots"),
    icon!("voxium:flint", "item_flint"),
    icon!("voxium:painting", "item_painting"),
    icon!("voxium:golden_apple", "item_golden_apple"),
    icon!(
        "voxium:enchanted_golden_apple",
        "item_enchanted_golden_apple"
    ),
    icon!("voxium:powder_snow_bucket", "item_powder_snow_bucket"),
    icon!("voxium:snowball", "item_snowball"),
    icon!("voxium:milk_bucket", "item_milk_bucket"),
    icon!("voxium:pufferfish_bucket", "item_pufferfish_bucket"),
    icon!("voxium:salmon_bucket", "item_salmon_bucket"),
    icon!("voxium:cod_bucket", "item_cod_bucket"),
    icon!("voxium:tropical_fish_bucket", "item_tropical_fish_bucket"),
    icon!("voxium:axolotl_bucket", "item_axolotl_bucket"),
    icon!("voxium:tadpole_bucket", "item_tadpole_bucket"),
    icon!("voxium:paper", "item_paper"),
    icon!("voxium:book", "item_book"),
    icon!("voxium:slime_ball", "item_slime_ball"),
    icon!("voxium:compass", "item_compass"),
    icon!("voxium:recovery_compass", "item_recovery_compass"),
    icon!("voxium:bundle", "item_bundle"),
    icon!("voxium:fishing_rod", "item_fishing_rod"),
    icon!("voxium:clock", "item_clock"),
    icon!("voxium:spyglass", "item_spyglass"),
    icon!("voxium:glowstone_dust", "item_glowstone_dust"),
    icon!("voxium:cod", "item_cod"),
    icon!("voxium:salmon", "item_salmon"),
    icon!("voxium:tropical_fish", "item_tropical_fish"),
    icon!("voxium:pufferfish", "item_pufferfish"),
    icon!("voxium:cooked_cod", "item_cooked_cod"),
    icon!("voxium:cooked_salmon", "item_cooked_salmon"),
    icon!("voxium:ink_sac", "item_ink_sac"),
    icon!("voxium:glow_ink_sac", "item_glow_ink_sac"),
    icon!("voxium:cocoa_beans", "item_cocoa_beans"),
    icon!("voxium:bone_meal", "item_bone_meal"),
    icon!("voxium:bone", "item_bone"),
    icon!("voxium:sugar", "item_sugar"),
    icon!("voxium:cookie", "item_cookie"),
    icon!("voxium:filled_map", "item_filled_map"),
    icon!("voxium:shears", "item_shears"),
    icon!("voxium:melon_slice", "item_melon_slice"),
    icon!("voxium:dried_kelp", "item_dried_kelp"),
    icon!("voxium:pumpkin_seeds", "item_pumpkin_seeds"),
    icon!("voxium:melon_seeds", "item_melon_seeds"),
    icon!("voxium:ender_pearl", "item_ender_pearl"),
    icon!("voxium:blaze_rod", "item_blaze_rod"),
    icon!("voxium:ghast_tear", "item_ghast_tear"),
    icon!("voxium:gold_nugget", "item_gold_nugget"),
    icon!("voxium:potion", "item_potion"),
    icon!("voxium:glass_bottle", "item_glass_bottle"),
    icon!("voxium:spider_eye", "item_spider_eye"),
    icon!("voxium:fermented_spider_eye", "item_fermented_spider_eye"),
    icon!("voxium:blaze_powder", "item_blaze_powder"),
    icon!("voxium:magma_cream", "item_magma_cream"),
    icon!("voxium:ender_eye", "item_ender_eye"),
    icon!(
        "voxium:glistering_melon_slice",
        "item_glistering_melon_slice"
    ),
    icon!("voxium:allay_spawn_egg", "item_allay_spawn_egg"),
    icon!("voxium:axolotl_spawn_egg", "item_axolotl_spawn_egg"),
    icon!("voxium:bat_spawn_egg", "item_bat_spawn_egg"),
    icon!("voxium:bee_spawn_egg", "item_bee_spawn_egg"),
    icon!("voxium:blaze_spawn_egg", "item_blaze_spawn_egg"),
    icon!("voxium:cat_spawn_egg", "item_cat_spawn_egg"),
    icon!("voxium:camel_spawn_egg", "item_camel_spawn_egg"),
    icon!("voxium:cave_spider_spawn_egg", "item_cave_spider_spawn_egg"),
    icon!("voxium:chicken_spawn_egg", "item_chicken_spawn_egg"),
    icon!("voxium:cod_spawn_egg", "item_cod_spawn_egg"),
    icon!("voxium:cow_spawn_egg", "item_cow_spawn_egg"),
    icon!("voxium:creeper_spawn_egg", "item_creeper_spawn_egg"),
    icon!("voxium:dolphin_spawn_egg", "item_dolphin_spawn_egg"),
    icon!("voxium:donkey_spawn_egg", "item_donkey_spawn_egg"),
    icon!("voxium:drowned_spawn_egg", "item_drowned_spawn_egg"),
    icon!(
        "voxium:elder_guardian_spawn_egg",
        "item_elder_guardian_spawn_egg"
    ),
    icon!(
        "voxium:ender_dragon_spawn_egg",
        "item_ender_dragon_spawn_egg"
    ),
    icon!("voxium:enderman_spawn_egg", "item_enderman_spawn_egg"),
    icon!("voxium:endermite_spawn_egg", "item_endermite_spawn_egg"),
    icon!("voxium:evoker_spawn_egg", "item_evoker_spawn_egg"),
    icon!("voxium:fox_spawn_egg", "item_fox_spawn_egg"),
    icon!("voxium:frog_spawn_egg", "item_frog_spawn_egg"),
    icon!("voxium:ghast_spawn_egg", "item_ghast_spawn_egg"),
    icon!("voxium:glow_squid_spawn_egg", "item_glow_squid_spawn_egg"),
    icon!("voxium:goat_spawn_egg", "item_goat_spawn_egg"),
    icon!("voxium:guardian_spawn_egg", "item_guardian_spawn_egg"),
    icon!("voxium:hoglin_spawn_egg", "item_hoglin_spawn_egg"),
    icon!("voxium:horse_spawn_egg", "item_horse_spawn_egg"),
    icon!("voxium:husk_spawn_egg", "item_husk_spawn_egg"),
    icon!("voxium:iron_golem_spawn_egg", "item_iron_golem_spawn_egg"),
    icon!("voxium:llama_spawn_egg", "item_llama_spawn_egg"),
    icon!("voxium:magma_cube_spawn_egg", "item_magma_cube_spawn_egg"),
    icon!("voxium:mooshroom_spawn_egg", "item_mooshroom_spawn_egg"),
    icon!("voxium:mule_spawn_egg", "item_mule_spawn_egg"),
    icon!("voxium:ocelot_spawn_egg", "item_ocelot_spawn_egg"),
    icon!("voxium:panda_spawn_egg", "item_panda_spawn_egg"),
    icon!("voxium:parrot_spawn_egg", "item_parrot_spawn_egg"),
    icon!("voxium:phantom_spawn_egg", "item_phantom_spawn_egg"),
    icon!("voxium:pig_spawn_egg", "item_pig_spawn_egg"),
    icon!("voxium:piglin_spawn_egg", "item_piglin_spawn_egg"),
    icon!(
        "voxium:piglin_brute_spawn_egg",
        "item_piglin_brute_spawn_egg"
    ),
    icon!("voxium:pillager_spawn_egg", "item_pillager_spawn_egg"),
    icon!("voxium:polar_bear_spawn_egg", "item_polar_bear_spawn_egg"),
    icon!("voxium:pufferfish_spawn_egg", "item_pufferfish_spawn_egg"),
    icon!("voxium:rabbit_spawn_egg", "item_rabbit_spawn_egg"),
    icon!("voxium:ravager_spawn_egg", "item_ravager_spawn_egg"),
    icon!("voxium:salmon_spawn_egg", "item_salmon_spawn_egg"),
    icon!("voxium:sheep_spawn_egg", "item_sheep_spawn_egg"),
    icon!("voxium:shulker_spawn_egg", "item_shulker_spawn_egg"),
    icon!("voxium:silverfish_spawn_egg", "item_silverfish_spawn_egg"),
    icon!("voxium:skeleton_spawn_egg", "item_skeleton_spawn_egg"),
    icon!(
        "voxium:skeleton_horse_spawn_egg",
        "item_skeleton_horse_spawn_egg"
    ),
    icon!("voxium:slime_spawn_egg", "item_slime_spawn_egg"),
    icon!("voxium:sniffer_spawn_egg", "item_sniffer_spawn_egg"),
    icon!("voxium:snow_golem_spawn_egg", "item_snow_golem_spawn_egg"),
    icon!("voxium:spider_spawn_egg", "item_spider_spawn_egg"),
    icon!("voxium:squid_spawn_egg", "item_squid_spawn_egg"),
    icon!("voxium:stray_spawn_egg", "item_stray_spawn_egg"),
    icon!("voxium:strider_spawn_egg", "item_strider_spawn_egg"),
    icon!("voxium:tadpole_spawn_egg", "item_tadpole_spawn_egg"),
    icon!(
        "voxium:trader_llama_spawn_egg",
        "item_trader_llama_spawn_egg"
    ),
    icon!(
        "voxium:tropical_fish_spawn_egg",
        "item_tropical_fish_spawn_egg"
    ),
    icon!("voxium:turtle_spawn_egg", "item_turtle_spawn_egg"),
    icon!("voxium:vex_spawn_egg", "item_vex_spawn_egg"),
    icon!("voxium:villager_spawn_egg", "item_villager_spawn_egg"),
    icon!("voxium:vindicator_spawn_egg", "item_vindicator_spawn_egg"),
    icon!(
        "voxium:wandering_trader_spawn_egg",
        "item_wandering_trader_spawn_egg"
    ),
    icon!("voxium:warden_spawn_egg", "item_warden_spawn_egg"),
    icon!("voxium:witch_spawn_egg", "item_witch_spawn_egg"),
    icon!("voxium:wither_spawn_egg", "item_wither_spawn_egg"),
    icon!(
        "voxium:wither_skeleton_spawn_egg",
        "item_wither_skeleton_spawn_egg"
    ),
    icon!("voxium:wolf_spawn_egg", "item_wolf_spawn_egg"),
    icon!("voxium:zoglin_spawn_egg", "item_zoglin_spawn_egg"),
    icon!("voxium:zombie_spawn_egg", "item_zombie_spawn_egg"),
    icon!(
        "voxium:zombie_horse_spawn_egg",
        "item_zombie_horse_spawn_egg"
    ),
    icon!(
        "voxium:zombie_villager_spawn_egg",
        "item_zombie_villager_spawn_egg"
    ),
    icon!(
        "voxium:zombified_piglin_spawn_egg",
        "item_zombified_piglin_spawn_egg"
    ),
    icon!("voxium:experience_bottle", "item_experience_bottle"),
    icon!("voxium:fire_charge", "item_fire_charge"),
    icon!("voxium:writable_book", "item_writable_book"),
    icon!("voxium:written_book", "item_written_book"),
    icon!("voxium:item_frame", "item_item_frame"),
    icon!("voxium:glow_item_frame", "item_glow_item_frame"),
    icon!("voxium:carrot", "item_carrot"),
    icon!("voxium:potato", "item_potato"),
    icon!("voxium:baked_potato", "item_baked_potato"),
    icon!("voxium:poisonous_potato", "item_poisonous_potato"),
    icon!("voxium:map", "item_map"),
    icon!("voxium:golden_carrot", "item_golden_carrot"),
    icon!("voxium:nether_star", "item_nether_star"),
    icon!("voxium:pumpkin_pie", "item_pumpkin_pie"),
    icon!("voxium:firework_rocket", "item_firework_rocket"),
    icon!("voxium:firework_star", "item_firework_star"),
    icon!("voxium:enchanted_book", "item_enchanted_book"),
    icon!("voxium:nether_brick", "item_nether_brick"),
    icon!("voxium:prismarine_shard", "item_prismarine_shard"),
    icon!("voxium:prismarine_crystals", "item_prismarine_crystals"),
    icon!("voxium:rabbit", "item_rabbit"),
    icon!("voxium:cooked_rabbit", "item_cooked_rabbit"),
    icon!("voxium:rabbit_stew", "item_rabbit_stew"),
    icon!("voxium:rabbit_foot", "item_rabbit_foot"),
    icon!("voxium:rabbit_hide", "item_rabbit_hide"),
    icon!("voxium:armor_stand", "item_armor_stand"),
    icon!("voxium:iron_horse_armor", "item_iron_horse_armor"),
    icon!("voxium:golden_horse_armor", "item_golden_horse_armor"),
    icon!("voxium:diamond_horse_armor", "item_diamond_horse_armor"),
    icon!("voxium:leather_horse_armor", "item_leather_horse_armor"),
    icon!("voxium:lead", "item_lead"),
    icon!("voxium:name_tag", "item_name_tag"),
    icon!(
        "voxium:command_block_minecart",
        "item_command_block_minecart"
    ),
    icon!("voxium:end_crystal", "item_end_crystal"),
    icon!("voxium:chorus_fruit", "item_chorus_fruit"),
    icon!("voxium:popped_chorus_fruit", "item_popped_chorus_fruit"),
    icon!("voxium:torchflower_seeds", "item_torchflower_seeds"),
    icon!("voxium:pitcher_pod", "item_pitcher_pod"),
    icon!("voxium:beetroot", "item_beetroot"),
    icon!("voxium:beetroot_seeds", "item_beetroot_seeds"),
    icon!("voxium:beetroot_soup", "item_beetroot_soup"),
    icon!("voxium:dragon_breath", "item_dragon_breath"),
    icon!("voxium:splash_potion", "item_splash_potion"),
    icon!("voxium:spectral_arrow", "item_spectral_arrow"),
    icon!("voxium:tipped_arrow", "item_tipped_arrow"),
    icon!("voxium:lingering_potion", "item_lingering_potion"),
    icon!("voxium:shield", "item_shield"),
    icon!("voxium:totem_of_undying", "item_totem_of_undying"),
    icon!("voxium:shulker_shell", "item_shulker_shell"),
    icon!("voxium:iron_nugget", "item_iron_nugget"),
    icon!("voxium:knowledge_book", "item_knowledge_book"),
    icon!("voxium:debug_stick", "item_debug_stick"),
    icon!("voxium:music_disc_13", "item_music_disc_13"),
    icon!("voxium:music_disc_cat", "item_music_disc_cat"),
    icon!("voxium:music_disc_blocks", "item_music_disc_blocks"),
    icon!("voxium:music_disc_chirp", "item_music_disc_chirp"),
    icon!("voxium:music_disc_far", "item_music_disc_far"),
    icon!("voxium:music_disc_mall", "item_music_disc_mall"),
    icon!("voxium:music_disc_mellohi", "item_music_disc_mellohi"),
    icon!("voxium:music_disc_stal", "item_music_disc_stal"),
    icon!("voxium:music_disc_strad", "item_music_disc_strad"),
    icon!("voxium:music_disc_ward", "item_music_disc_ward"),
    icon!("voxium:music_disc_11", "item_music_disc_11"),
    icon!("voxium:music_disc_wait", "item_music_disc_wait"),
    icon!("voxium:music_disc_otherside", "item_music_disc_otherside"),
    icon!("voxium:music_disc_relic", "item_music_disc_relic"),
    icon!("voxium:music_disc_5", "item_music_disc_5"),
    icon!("voxium:music_disc_pigstep", "item_music_disc_pigstep"),
    icon!("voxium:disc_fragment_5", "item_disc_fragment_5"),
    icon!("voxium:trident", "item_trident"),
    icon!("voxium:phantom_membrane", "item_phantom_membrane"),
    icon!("voxium:nautilus_shell", "item_nautilus_shell"),
    icon!("voxium:heart_of_the_sea", "item_heart_of_the_sea"),
    icon!("voxium:crossbow", "item_crossbow"),
    icon!("voxium:suspicious_stew", "item_suspicious_stew"),
    icon!("voxium:flower_banner_pattern", "item_flower_banner_pattern"),
    icon!(
        "voxium:creeper_banner_pattern",
        "item_creeper_banner_pattern"
    ),
    icon!("voxium:skull_banner_pattern", "item_skull_banner_pattern"),
    icon!("voxium:mojang_banner_pattern", "item_mojang_banner_pattern"),
    icon!("voxium:globe_banner_pattern", "item_globe_banner_pattern"),
    icon!("voxium:piglin_banner_pattern", "item_piglin_banner_pattern"),
    icon!("voxium:goat_horn", "item_goat_horn"),
    icon!("voxium:sweet_berries", "item_sweet_berries"),
    icon!("voxium:glow_berries", "item_glow_berries"),
    icon!("voxium:honeycomb", "item_honeycomb"),
    icon!("voxium:honey_bottle", "item_honey_bottle"),
    icon!("voxium:echo_shard", "item_echo_shard"),
    icon!("voxium:brush", "item_brush"),
    icon!(
        "voxium:netherite_upgrade_smithing_template",
        "item_netherite_upgrade_smithing_template"
    ),
    icon!(
        "voxium:sentry_armor_trim_smithing_template",
        "item_sentry_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:dune_armor_trim_smithing_template",
        "item_dune_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:coast_armor_trim_smithing_template",
        "item_coast_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:wild_armor_trim_smithing_template",
        "item_wild_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:ward_armor_trim_smithing_template",
        "item_ward_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:eye_armor_trim_smithing_template",
        "item_eye_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:vex_armor_trim_smithing_template",
        "item_vex_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:tide_armor_trim_smithing_template",
        "item_tide_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:snout_armor_trim_smithing_template",
        "item_snout_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:rib_armor_trim_smithing_template",
        "item_rib_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:spire_armor_trim_smithing_template",
        "item_spire_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:wayfinder_armor_trim_smithing_template",
        "item_wayfinder_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:shaper_armor_trim_smithing_template",
        "item_shaper_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:silence_armor_trim_smithing_template",
        "item_silence_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:raiser_armor_trim_smithing_template",
        "item_raiser_armor_trim_smithing_template"
    ),
    icon!(
        "voxium:host_armor_trim_smithing_template",
        "item_host_armor_trim_smithing_template"
    ),
    icon!("voxium:angler_pottery_sherd", "item_angler_pottery_sherd"),
    icon!("voxium:archer_pottery_sherd", "item_archer_pottery_sherd"),
    icon!("voxium:arms_up_pottery_sherd", "item_arms_up_pottery_sherd"),
    icon!("voxium:blade_pottery_sherd", "item_blade_pottery_sherd"),
    icon!("voxium:brewer_pottery_sherd", "item_brewer_pottery_sherd"),
    icon!("voxium:burn_pottery_sherd", "item_burn_pottery_sherd"),
    icon!("voxium:danger_pottery_sherd", "item_danger_pottery_sherd"),
    icon!(
        "voxium:explorer_pottery_sherd",
        "item_explorer_pottery_sherd"
    ),
    icon!("voxium:friend_pottery_sherd", "item_friend_pottery_sherd"),
    icon!("voxium:heart_pottery_sherd", "item_heart_pottery_sherd"),
    icon!(
        "voxium:heartbreak_pottery_sherd",
        "item_heartbreak_pottery_sherd"
    ),
    icon!("voxium:howl_pottery_sherd", "item_howl_pottery_sherd"),
    icon!("voxium:miner_pottery_sherd", "item_miner_pottery_sherd"),
    icon!("voxium:mourner_pottery_sherd", "item_mourner_pottery_sherd"),
    icon!("voxium:plenty_pottery_sherd", "item_plenty_pottery_sherd"),
    icon!("voxium:prize_pottery_sherd", "item_prize_pottery_sherd"),
    icon!("voxium:sheaf_pottery_sherd", "item_sheaf_pottery_sherd"),
    icon!("voxium:shelter_pottery_sherd", "item_shelter_pottery_sherd"),
    icon!("voxium:skull_pottery_sherd", "item_skull_pottery_sherd"),
    icon!("voxium:snort_pottery_sherd", "item_snort_pottery_sherd"),
];

/// RGBA 128×128 для предмета с пиксельным значком.
pub(crate) fn pixels(id: &str) -> Option<Vec<u8>> {
    let (_, bytes) = ICONS.iter().find(|(known, _)| *known == id)?;
    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .ok()?
        .into_rgba8();
    (image.dimensions() == (128, 128)).then(|| image.into_raw())
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_core::content::{ContentRegistry, ItemKind};

    #[test]
    fn every_new_flat_item_has_a_decodable_transparent_icon() {
        let catalog = ContentRegistry::default();
        for (id, _) in ICONS {
            assert!(catalog.find(id).is_some(), "{id}");
            let rgba = pixels(id).unwrap();
            assert_eq!(rgba.len(), 128 * 128 * 4);
            assert!(rgba.as_chunks::<4>().0.iter().any(|p| p[3] == 0), "{id}");
            assert!(rgba.as_chunks::<4>().0.iter().any(|p| p[3] == 255), "{id}");
        }
        for (_, item) in catalog.iter() {
            if (item.id.starts_with("builder:")
                || item.id.starts_with("flora:")
                || item.id.starts_with("mine:")
                || item.id.starts_with("fluid:"))
                && !matches!(item.kind, ItemKind::Block { .. })
            {
                assert!(pixels(&item.id).is_some(), "{} has no icon", item.id);
            }
        }
    }
}
