//! Проверенный каталог предметов; числовые ID действуют только в текущем запуске.

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ItemId(pub u16);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum ArmorSlot {
    Head,
    Chest,
    Legs,
    Feet,
}

impl ArmorSlot {
    #[must_use]
    pub fn index(self) -> usize {
        match self {
            Self::Head => 0,
            Self::Chest => 1,
            Self::Legs => 2,
            Self::Feet => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ItemKind {
    Block { block: String },
    Material,
    Tool,
    Armor { slot: ArmorSlot },
    Food,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub enum BreakDrop {
    #[default]
    SelfItem,
    Nothing,
    Item(String),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub max_stack: u16,
    pub kind: ItemKind,
    #[serde(default)]
    pub break_drop: BreakDrop,
}

#[derive(Clone, Debug)]
pub struct ContentRegistry {
    definitions: Vec<ItemDef>,
    by_name: HashMap<String, ItemId>,
    by_block: HashMap<u8, ItemId>,
}

// Имена соответствуют стабильным сохранённым ID; новые блоки только дописываются.
const BLOCK_NAMES: [&str; 26] = [
    "foundation:air",
    "foundation:grass",
    "foundation:dirt",
    "foundation:stone",
    "foundation:deepslate",
    "foundation:sand",
    "foundation:gravel",
    "foundation:oak_log",
    "foundation:oak_leaves",
    "foundation:oak_planks",
    "foundation:cobblestone",
    "foundation:glass",
    "foundation:water",
    "foundation:bedrock",
    "foundation:snow",
    "foundation:sandstone",
    "survival:chest",
    "survival:coal_ore",
    "survival:iron_ore",
    "survival:gold_ore",
    "survival:diamond_ore",
    "survival:iron_block",
    "survival:gold_block",
    "survival:diamond_block",
    "survival:crafting_table",
    "survival:furnace",
];

impl Default for ContentRegistry {
    fn default() -> Self {
        Self::from_ron(include_str!("../../../assets/data/items.ron"))
            .expect("встроенный каталог предметов должен быть корректным")
    }
}

impl ContentRegistry {
    pub fn from_ron(source: &str) -> Result<Self, String> {
        let definitions: Vec<ItemDef> = ron::from_str(source)
            .map_err(|error| format!("Ошибка RON каталога предметов: {error}"))?;
        Self::checked(definitions)
    }

    fn checked(definitions: Vec<ItemDef>) -> Result<Self, String> {
        if definitions.len() > usize::from(u16::MAX) + 1 {
            return Err("Слишком много предметов для числовых ID".into());
        }
        let mut by_name = HashMap::new();
        let mut by_block = HashMap::new();
        for (index, item) in definitions.iter().enumerate() {
            if !valid_namespaced_id(&item.id) {
                return Err(format!("Некорректный ID предмета: {}", item.id));
            }
            if item.name.trim().is_empty() {
                return Err(format!("Пустое название предмета: {}", item.id));
            }
            if !(1..=64).contains(&item.max_stack) {
                return Err(format!("Размер стека должен быть от 1 до 64: {}", item.id));
            }
            if matches!(item.kind, ItemKind::Tool | ItemKind::Armor { .. }) && item.max_stack != 1 {
                return Err(format!("Инструменты и броня требуют стек 1: {}", item.id));
            }
            let id = ItemId(index as u16);
            if by_name.insert(item.id.clone(), id).is_some() {
                return Err(format!("Повторяющийся ID предмета: {}", item.id));
            }
            if let ItemKind::Block { block } = &item.kind {
                let block_id = BLOCK_NAMES
                    .iter()
                    .position(|name| name == block)
                    .ok_or_else(|| format!("Неизвестный блок {block} у предмета {}", item.id))?
                    as u8;
                if matches!(block_id, 0 | 13) {
                    return Err(format!("Блок {block} не может быть предметом"));
                }
                if by_block.insert(block_id, id).is_some() {
                    return Err(format!("Несколько предметов ссылаются на блок {block}"));
                }
            }
        }
        for item in &definitions {
            if let BreakDrop::Item(reference) = &item.break_drop
                && !by_name.contains_key(reference)
            {
                return Err(format!(
                    "Неизвестный предмет добычи {reference} у {}",
                    item.id
                ));
            }
        }
        Ok(Self {
            definitions,
            by_name,
            by_block,
        })
    }

    pub fn reload(&self, source: &str) -> Result<Self, String> {
        let candidate = Self::from_ron(source)?;
        let mut definitions = Vec::with_capacity(candidate.len());
        for previous in &self.definitions {
            let next_id = candidate
                .find(&previous.id)
                .ok_or_else(|| format!("Нельзя удалить существующий предмет: {}", previous.id))?;
            let next = &candidate.definitions[usize::from(next_id.0)];
            if next.kind != previous.kind {
                return Err(format!(
                    "Нельзя изменить тип существующего предмета: {}",
                    previous.id
                ));
            }
            definitions.push(next.clone());
        }
        for next in candidate.definitions {
            if !self.by_name.contains_key(&next.id) {
                definitions.push(next);
            }
        }
        Self::checked(definitions)
    }

    #[must_use]
    pub fn get(&self, id: ItemId) -> Option<&ItemDef> {
        self.definitions.get(usize::from(id.0))
    }

    #[must_use]
    pub fn find(&self, id: &str) -> Option<ItemId> {
        self.by_name.get(id).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (ItemId, &ItemDef)> {
        self.definitions
            .iter()
            .enumerate()
            .map(|(index, item)| (ItemId(index as u16), item))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    #[must_use]
    pub fn item_for_block(&self, block: u8) -> Option<ItemId> {
        self.by_block.get(&block).copied()
    }

    #[must_use]
    pub fn block_for_item(&self, id: ItemId) -> Option<u8> {
        let ItemKind::Block { block } = &self.get(id)?.kind else {
            return None;
        };
        BLOCK_NAMES
            .iter()
            .position(|name| name == block)
            .map(|index| index as u8)
    }

    #[must_use]
    pub fn drop_for_block(&self, block: u8) -> Option<ItemId> {
        let item = self.item_for_block(block)?;
        match &self.get(item)?.break_drop {
            BreakDrop::SelfItem => Some(item),
            BreakDrop::Nothing => None,
            BreakDrop::Item(reference) => self.find(reference),
        }
    }

    #[must_use]
    pub fn max_stack(&self, id: ItemId) -> u16 {
        self.get(id).map_or(0, |item| item.max_stack)
    }
}

fn valid_namespaced_id(id: &str) -> bool {
    let Some((namespace, path)) = id.split_once(':') else {
        return false;
    };
    let valid = |character: u8| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, b'_' | b'-' | b'.')
    };
    !namespace.is_empty()
        && !path.is_empty()
        && namespace.bytes().all(valid)
        && path
            .bytes()
            .all(|character| valid(character) || character == b'/')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progression_catalog_adds_stable_blocks_drops_tools_and_armor() {
        let registry = ContentRegistry::default();
        let blocks = [
            "coal_ore",
            "iron_ore",
            "gold_ore",
            "diamond_ore",
            "iron_block",
            "gold_block",
            "diamond_block",
            "crafting_table",
            "furnace",
        ];
        for (offset, name) in blocks.iter().enumerate() {
            let id = registry
                .find(&format!("survival:{name}"))
                .expect("new block item");
            assert_eq!(registry.block_for_item(id), Some(17 + offset as u8));
            assert_eq!(registry.item_for_block(17 + offset as u8), Some(id));
        }
        for (block, drop) in [
            (17, "coal"),
            (18, "raw_iron"),
            (19, "raw_gold"),
            (20, "diamond"),
        ] {
            assert_eq!(
                registry.drop_for_block(block),
                Some(registry.find(&format!("survival:{drop}")).unwrap())
            );
        }
        for block in 21..=25 {
            assert_eq!(
                registry.drop_for_block(block),
                registry.item_for_block(block)
            );
        }
        for name in [
            "raw_iron",
            "raw_gold",
            "iron_ingot",
            "gold_ingot",
            "diamond",
            "leather",
        ] {
            let id = registry.find(&format!("survival:{name}")).unwrap();
            assert_eq!(registry.get(id).unwrap().kind, ItemKind::Material);
            assert_eq!(registry.max_stack(id), 64);
        }
        for material in ["wooden", "stone", "iron", "golden", "diamond"] {
            for tool in ["axe", "hoe", "pickaxe", "shovel", "sword"] {
                let def = registry
                    .get(
                        registry
                            .find(&format!("survival:{material}_{tool}"))
                            .unwrap(),
                    )
                    .unwrap();
                assert_eq!(def.kind, ItemKind::Tool);
                assert_eq!(def.max_stack, 1);
                assert!(!def.name.contains("образец"));
            }
        }
        for material in ["leather", "iron", "golden", "diamond"] {
            for (part, slot) in [
                ("helmet", ArmorSlot::Head),
                ("chestplate", ArmorSlot::Chest),
                ("leggings", ArmorSlot::Legs),
                ("boots", ArmorSlot::Feet),
            ] {
                let def = registry
                    .get(
                        registry
                            .find(&format!("survival:{material}_{part}"))
                            .unwrap(),
                    )
                    .unwrap();
                assert_eq!(def.kind, ItemKind::Armor { slot });
                assert_eq!(def.max_stack, 1);
            }
        }
        assert_eq!(registry.len(), 74);
    }

    fn material(id: &str, max_stack: u16) -> String {
        format!("(id: \"{id}\", name: \"Материал\", max_stack: {max_stack}, kind: Material)")
    }

    #[test]
    fn default_catalog_preserves_original_stage_three_blocks_and_fixtures() {
        let registry = ContentRegistry::default();
        assert_eq!(registry.len(), 74);
        assert!(!registry.is_empty());
        let names = [
            "foundation:air",
            "foundation:grass",
            "foundation:dirt",
            "foundation:stone",
            "foundation:deepslate",
            "foundation:sand",
            "foundation:gravel",
            "foundation:oak_log",
            "foundation:oak_leaves",
            "foundation:oak_planks",
            "foundation:cobblestone",
            "foundation:glass",
            "foundation:water",
            "foundation:bedrock",
            "foundation:snow",
            "foundation:sandstone",
            "survival:chest",
        ];
        for (block, name) in names.iter().enumerate() {
            if block == 0 || block == 13 {
                assert_eq!(registry.item_for_block(block as u8), None);
            } else {
                let item = registry.item_for_block(block as u8).unwrap();
                assert_eq!(&registry.get(item).unwrap().id, name);
                assert_eq!(registry.block_for_item(item), Some(block as u8));
            }
        }
        assert!(registry.item_for_block(17).is_some());
        assert_eq!(
            registry.max_stack(registry.find("survival:egg").unwrap()),
            16
        );
        assert_eq!(
            registry.max_stack(registry.find("survival:wooden_pickaxe").unwrap()),
            1
        );
        assert_eq!(registry.find("survival:wool"), None);
        assert_eq!(registry.find("survival:raw_beef"), None);
        assert_eq!(registry.get(ItemId(u16::MAX)).map(|d| &d.id), None);
        assert_eq!(registry.max_stack(ItemId(u16::MAX)), 0);
        assert_eq!(
            registry.block_for_item(registry.find("survival:egg").unwrap()),
            None
        );
        for (slot, index) in [
            (ArmorSlot::Head, 0),
            (ArmorSlot::Chest, 1),
            (ArmorSlot::Legs, 2),
            (ArmorSlot::Feet, 3),
        ] {
            assert_eq!(slot.index(), index);
            assert_eq!(
                registry
                    .iter()
                    .filter(|(_, item)| item.kind == ItemKind::Armor { slot })
                    .count(),
                4
            );
        }
    }

    #[test]
    fn break_drops_are_data_driven_and_resolve_forward_item_references() {
        let registry = ContentRegistry::default();
        assert_eq!(registry.drop_for_block(1), registry.item_for_block(2));
        assert_eq!(registry.drop_for_block(3), registry.item_for_block(10));
        for block in [0, 11, 12, 13, 255] {
            assert_eq!(registry.drop_for_block(block), None);
        }
        for block in [2, 4, 5, 6, 7, 8, 9, 10, 14, 15, 16] {
            assert_eq!(
                registry.drop_for_block(block),
                registry.item_for_block(block)
            );
        }
        let source = r#"[(id:"test:block",name:"Блок",max_stack:64,kind:Block(block:"foundation:stone"),break_drop:Item("test:material")),(id:"test:material",name:"Материал",max_stack:16,kind:Material)]"#;
        let registry = ContentRegistry::from_ron(source).unwrap();
        assert_eq!(registry.drop_for_block(3), registry.find("test:material"));
        assert!(
            ContentRegistry::from_ron(&source.replace("test:material", "test:missing").replacen(
                r#"id:"test:missing""#,
                r#"id:"test:material""#,
                1
            ))
            .is_err()
        );
        let changed = registry
            .reload(&source.replace(r#"break_drop:Item("test:material")"#, "break_drop:Nothing"))
            .unwrap();
        assert_eq!(changed.drop_for_block(3), None);
        assert_eq!(registry.drop_for_block(3), registry.find("test:material"));
    }

    #[test]
    fn empty_catalog_is_valid_and_runtime_iteration_is_stable() {
        let empty = ContentRegistry::from_ron("[]").unwrap();
        assert!(empty.is_empty());
        let source = format!("[{}, {}]", material("test:z", 64), material("test:a", 16));
        let registry = ContentRegistry::from_ron(&source).unwrap();
        assert_eq!(
            registry
                .iter()
                .map(|(id, def)| (id, def.id.as_str()))
                .collect::<Vec<_>>(),
            vec![(ItemId(0), "test:z"), (ItemId(1), "test:a")]
        );
    }

    #[test]
    fn reload_preserves_ids_appends_additions_and_updates_properties() {
        let original = ContentRegistry::from_ron(&format!(
            "[{},{}]",
            material("test:a", 64),
            material("test:b", 16)
        ))
        .unwrap();
        let updated = original
            .reload(&format!(
                "[{},{},{}]",
                material("test:c", 32),
                material("test:b", 8),
                material("test:a", 16)
            ))
            .unwrap();
        assert_eq!(updated.find("test:a"), original.find("test:a"));
        assert_eq!(updated.find("test:b"), original.find("test:b"));
        assert_eq!(updated.find("test:c"), Some(ItemId(2)));
        assert_eq!(updated.max_stack(ItemId(0)), 16);
        assert_eq!(original.max_stack(ItemId(0)), 64);
        assert_eq!(
            updated
                .iter()
                .map(|(_, def)| def.id.as_str())
                .collect::<Vec<_>>(),
            ["test:a", "test:b", "test:c"]
        );
    }

    #[test]
    fn reload_rejects_removal_kind_and_block_or_armor_slot_changes_atomically() {
        let original = ContentRegistry::default();
        assert!(original.reload("[]").is_err());
        let ron = include_str!("../../../assets/data/items.ron");
        let remapped = ron
            .replace("block: \"foundation:grass\"", "block: \"test:temporary\"")
            .replace("block: \"foundation:dirt\"", "block: \"foundation:grass\"")
            .replace("block: \"test:temporary\"", "block: \"foundation:dirt\"");
        assert!(ContentRegistry::from_ron(&remapped).is_ok());
        assert!(original.reload(&remapped).is_err());
        for changed in [
            ron.replace("kind: Material", "kind: Food"),
            ron.replace("block: \"foundation:grass\"", "block: \"foundation:air\""),
            ron.replace("slot: Head", "slot: Feet"),
        ] {
            assert!(original.reload(&changed).is_err());
        }
        assert_eq!(original.len(), 74);
        assert_eq!(
            original.max_stack(original.find("survival:egg").unwrap()),
            16
        );
    }

    #[test]
    fn numeric_id_capacity_accepts_last_u16_and_rejects_overflow() {
        let mut source = String::from("[");
        for index in 0..=u16::MAX {
            source.push_str(&material(&format!("test:item_{index}"), 64));
            source.push(',');
        }
        source.push(']');
        let registry = ContentRegistry::from_ron(&source).unwrap();
        assert_eq!(registry.len(), 65_536);
        assert_eq!(registry.find("test:item_65535"), Some(ItemId(u16::MAX)));
        assert_eq!(
            registry.get(ItemId(u16::MAX)).unwrap().id,
            "test:item_65535"
        );
        source.pop();
        source.push_str(&material("test:overflow", 64));
        source.push(']');
        assert!(ContentRegistry::from_ron(&source).is_err());
        assert!(registry.reload(&source).is_err());
        assert_eq!(registry.len(), 65_536);
    }

    #[test]
    fn malformed_names_duplicates_caps_unknown_fields_and_block_references_fail() {
        for id in [
            "",
            "stone",
            ":stone",
            "test:",
            "test:a:b",
            "Test:a",
            "test:a b",
            "test:камень",
        ] {
            assert!(
                ContentRegistry::from_ron(&format!("[{}]", material(id, 64))).is_err(),
                "{id}"
            );
        }
        assert!(
            ContentRegistry::from_ron(&format!(
                "[{},{}]",
                material("test:a", 64),
                material("test:a", 64)
            ))
            .is_err()
        );
        for record in [
            material("test:a", 0),
            material("test:a", 65),
            "(id:\"test:a\",name:\"\",max_stack:64,kind:Material)".into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Tool)".into(),
            "(id:\"test:a\",name:\"A\",max_stack:16,kind:Armor(slot:Head))".into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Material,typo:true)".into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Block(block:\"survival:unknown_ore\"))"
                .into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Block(block:\"foundation:bedrock\"))"
                .into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Block(block:\"foundation:air\"))".into(),
            "(id:\"test:a\",name:\"A\",max_stack:64,kind:Block(block:\"foundation:grass\",typo:1))"
                .into(),
        ] {
            assert!(
                ContentRegistry::from_ron(&format!("[{record}]")).is_err(),
                "{record}"
            );
        }
        let duplicate_block = "[(id:\"test:a\",name:\"A\",max_stack:64,kind:Block(block:\"foundation:grass\")),(id:\"test:b\",name:\"B\",max_stack:64,kind:Block(block:\"foundation:grass\"))]";
        assert!(ContentRegistry::from_ron(duplicate_block).is_err());
    }
}

// Эти типы проверяют только предложенные схемы; игровые системы их не загружают.
#[cfg(test)]
#[allow(dead_code)]
mod example_schemas {
    use super::*;
    use std::collections::HashSet;

    #[derive(Debug, Deserialize)]
    enum ToolType {
        Hand,
        Pickaxe,
    }
    #[derive(Debug, Deserialize)]
    enum Distribution {
        Triangular,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    enum RecipeKind {
        Shapeless,
        Smelting,
    }
    #[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
    enum Biome {
        Plains,
        Forest,
        Desert,
        Mountains,
        Ocean,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct DropRef {
        item: String,
        min: u16,
        max: u16,
        chance: f32,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Textures {
        top: u16,
        side: u16,
        bottom: u16,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ExampleBlock {
        id: String,
        name: String,
        textures: Textures,
        hardness: f32,
        tool_type: ToolType,
        min_tier: u8,
        requires_tool: bool,
        drops: Vec<DropRef>,
        xp: u16,
        collision: bool,
        opacity: bool,
        light_emission: u8,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct MaterialStats {
        durability: u16,
        speed: f32,
        sword_damage: f32,
        tier: u8,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ExampleTool {
        id: String,
        tool_type: ToolType,
        material_stats: MaterialStats,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct CountedItem {
        item: String,
        count: u16,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Recipe {
        id: String,
        kind: RecipeKind,
        inputs: Vec<CountedItem>,
        output: CountedItem,
        seconds: Option<f32>,
        xp: Option<f32>,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct XpRange {
        min: u16,
        max: u16,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct OreTextures {
        stone_layer: u16,
        deepslate_layer: u16,
        deepslate_below_y: i32,
        overlay_layer: u16,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Ore {
        id: String,
        replaces: Vec<String>,
        y_min: i32,
        y_max: i32,
        distribution: Distribution,
        veins_per_chunk: u16,
        max_vein_size: u16,
        hardness: f32,
        tool_type: ToolType,
        min_tier: u8,
        requires_tool: bool,
        drops: Vec<DropRef>,
        xp: XpRange,
        textures: OreTextures,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Food {
        item: String,
        hunger: f32,
        saturation: f32,
        eat_seconds: f32,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Hitbox {
        width: f32,
        height: f32,
    }
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Cow {
        id: String,
        name: String,
        max_health: f32,
        speed: f32,
        hitbox: Hitbox,
        model: String,
        biome_weights: HashMap<Biome, f32>,
        drops: Vec<DropRef>,
        xp: XpRange,
        fall_damage: bool,
        shearing: Option<String>,
        egg_interval_seconds: Option<f32>,
    }

    #[test]
    fn example_catalog_references_resolve() {
        let items: Vec<ItemDef> =
            ron::from_str(include_str!("../../../assets/data/examples/items.ron")).unwrap();
        let blocks: Vec<ExampleBlock> =
            ron::from_str(include_str!("../../../assets/data/examples/blocks.ron")).unwrap();
        let tools: Vec<ExampleTool> =
            ron::from_str(include_str!("../../../assets/data/examples/tools.ron")).unwrap();
        let recipes: Vec<Recipe> = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .from_str(include_str!("../../../assets/data/examples/recipes.ron"))
            .unwrap();
        let ores: Vec<Ore> =
            ron::from_str(include_str!("../../../assets/data/examples/ores.ron")).unwrap();
        let foods: Vec<Food> =
            ron::from_str(include_str!("../../../assets/data/examples/food.ron")).unwrap();
        let cow: Cow = ron::from_str(include_str!(
            "../../../assets/data/examples/animals/cow.ron"
        ))
        .unwrap();
        let by_name: HashMap<_, _> = items.iter().map(|item| (item.id.as_str(), item)).collect();
        assert_eq!(by_name.len(), items.len());
        let mut block_names: HashSet<&str> = BLOCK_NAMES[..16].iter().copied().collect();
        let ore_names: HashSet<_> = ores.iter().map(|ore| ore.id.as_str()).collect();
        assert_eq!(ore_names.len(), ores.len());
        for block in &blocks {
            assert!(valid_namespaced_id(&block.id));
            block_names.insert(&block.id);
        }
        block_names.extend(ore_names);
        for item in &items {
            assert!(valid_namespaced_id(&item.id));
            assert!((1..=64).contains(&item.max_stack));
            if let ItemKind::Block { block } = &item.kind {
                assert!(block_names.contains(block.as_str()), "{}", block);
            }
        }
        let check_drops = |drops: &[DropRef]| {
            for drop in drops {
                let item = by_name
                    .get(drop.item.as_str())
                    .expect("ссылка добычи на предмет");
                assert!(drop.min > 0 && drop.min <= drop.max && drop.max <= item.max_stack);
                assert!(drop.chance.is_finite() && (0.0..=1.0).contains(&drop.chance));
            }
        };
        for block in &blocks {
            check_drops(&block.drops);
        }
        for ore in &ores {
            assert!(valid_namespaced_id(&ore.id));
            assert!(ore.y_min <= ore.y_max);
            for replacement in &ore.replaces {
                assert!(block_names.contains(replacement.as_str()));
            }
            check_drops(&ore.drops);
        }
        for tool in &tools {
            assert_eq!(by_name[tool.id.as_str()].kind, ItemKind::Tool);
        }
        let pickaxe = tools
            .iter()
            .find(|tool| tool.id == "survival:wooden_pickaxe")
            .unwrap();
        assert_eq!(pickaxe.material_stats.durability, 59);
        assert_eq!(pickaxe.material_stats.speed, 2.0);
        assert_eq!(pickaxe.material_stats.tier, 0);
        let recipe_names: HashSet<_> = recipes.iter().map(|recipe| recipe.id.as_str()).collect();
        assert_eq!(recipe_names.len(), recipes.len());
        for recipe in &recipes {
            assert!(valid_namespaced_id(&recipe.id));
            assert!(!recipe.inputs.is_empty());
            for stack in recipe.inputs.iter().chain(std::iter::once(&recipe.output)) {
                let item = by_name
                    .get(stack.item.as_str())
                    .expect("ссылка рецепта на предмет");
                assert!(stack.count > 0 && stack.count <= item.max_stack);
            }
            if recipe.kind == RecipeKind::Smelting {
                assert!(
                    recipe
                        .seconds
                        .is_some_and(|value| value.is_finite() && value > 0.0)
                );
                assert!(
                    recipe
                        .xp
                        .is_some_and(|value| value.is_finite() && value >= 0.0)
                );
            }
        }
        let stone = blocks
            .iter()
            .find(|block| block.id == "foundation:stone")
            .unwrap();
        assert_eq!(stone.drops[0].item, "foundation:cobblestone");
        let planks = recipes
            .iter()
            .find(|recipe| recipe.id == "survival:planks_from_log")
            .unwrap();
        assert_eq!(planks.inputs[0].item, "foundation:oak_log");
        assert_eq!(planks.inputs[0].count, 1);
        assert_eq!(planks.output.item, "foundation:oak_planks");
        assert_eq!(planks.output.count, 4);
        let food_names: HashSet<_> = foods.iter().map(|food| food.item.as_str()).collect();
        assert_eq!(food_names.len(), foods.len());
        for food in &foods {
            assert_eq!(by_name[food.item.as_str()].kind, ItemKind::Food);
            assert!(food.hunger.is_finite() && food.hunger > 0.0);
            assert!(food.saturation.is_finite() && food.saturation >= 0.0);
            assert!(food.eat_seconds.is_finite() && food.eat_seconds > 0.0);
        }
        for meat in ["beef", "pork", "mutton", "chicken"] {
            let raw = format!("survival:raw_{meat}");
            let cooked = format!("survival:cooked_{meat}");
            let recipe = recipes.iter().find(|recipe| recipe.id == cooked).unwrap();
            assert_eq!(recipe.inputs[0].item, raw);
            assert_eq!(recipe.output.item, cooked);
            assert!(food_names.contains(raw.as_str()) && food_names.contains(cooked.as_str()));
        }
        check_drops(&cow.drops);
        assert_eq!(cow.id, "survival:cow");
        assert!(cow.drops.iter().any(|drop| drop.item == "survival:leather"));
        assert!(
            cow.drops
                .iter()
                .any(|drop| drop.item == "survival:raw_beef")
        );
        // Шерсть, мясо и животные остаются примерами будущих этапов.
        let active = ContentRegistry::default();
        for future in ["survival:wool", "survival:raw_beef"] {
            assert_eq!(active.find(future), None);
        }
    }
}
