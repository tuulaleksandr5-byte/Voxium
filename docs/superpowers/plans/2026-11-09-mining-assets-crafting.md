# Mining, uploaded assets and inventory crafting Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Реализовать добычу по пользовательской таблице, девять новых блоков с исходными PNG, каталог инструментов/брони и работающий крафт 2×2.

**Architecture:** Существующие game_core и game_render расширяются без замены приложения. Ядро проверяет время/дроп/рецепты; клиент связывает их с действиями, UI и сохранениями. Старый генератор сохраняется, руда новых миров относится к генератору 2 со снимком параметров.

**Tech Stack:** Rust 1.99.1, Bevy 0.20.1, существующие serde/RON/image. Без новых зависимостей.

**Spec:** `docs/superpowers/specs/2026-11-09-mining-assets-crafting-design.md`.

## Global Constraints

- Не менять Bevy или существующие зависимости; не добавлять unsafe.
- Сохранить 17 исходных PNG, runtime ID блоков 0…16, слои 0…16 и пустой 17.
- Исходные PNG сохранять побайтно; линейная фильтрация, мипмапы, анизотропия x8.
- Пользовательская таблица задаёт точные секунды и имеет приоритет над прежней общей формулой; скорость и разрешение дропа независимы.
- Держать старые миры на генераторе 1. Не менять результат Chunk::generate для старого seed.
- Пока сохраняется ограничение этапа 3: новые системы инвентаря не восстанавливаются после закрытия мира; не обещать завершение этапов 4–9.
- Работать в текущей feature/2.0-progression; изменения пользователя не затирать. Рабочее дерево исходно чистое, кроме проекта этого обновления.
- Реализация задач последовательно; исполнитель не создаёт других агентов. Проверки не отключать: обновлять только ожидания, действительно изменённые новой спецификацией.

## Review Focus

- Повторное завершение добычи после смены цели/инструмента не выдаёт второй дроп.
- Курсор/полный рюкзак/смерть/закрытие крафта не дублируют и не теряют ингредиенты.
- Старый атлас 768×384 и короткий blocks.ron продолжают работать с новыми слоями и ID.
- Генератор 1 и контрольная сумма старого сохранения остаются совместимыми при новом снимке генератора 2.
- Ошибки PNG/RON сохраняют рабочее состояние; исходное RGBA не меняется при загрузке.

---

### Task 1: Original assets and stable content catalog

**Files:** `assets/textures/{blocks,icons,armor,tools}/`, `assets/data/items.ron`, `crates/game_core/src/{blocks,content}.rs`, `apps/client/src/{tool_textures,item_icons}.rs`, новый `item_textures.rs` при необходимости.

**Interfaces:** произвести константы COAL_ORE=17, IRON_ORE=18, GOLD_ORE=19, DIAMOND_ORE=20, IRON_BLOCK=21, GOLD_BLOCK=22, DIAMOND_BLOCK=23, CRAFTING_TABLE=24, FURNACE=25; BLOCKS.len()=26. Все новые строковые ID имеют namespace survival. Имена ресурсов/инструментов точно совпадают с PNG.

- [x] Добавить тест старых ID, нового RON/drop mapping и PNG RGBA/partial alpha; увидеть отказ до реализации.
- [x] Проверить и распаковать пользовательские архивы без изменения байтов, создать манифесты SHA-256 и README. Пути ZIP материалов уже содержат assets/textures; armor распаковывать под assets/textures/armor. TXT таблицу скопировать как пользовательский референс в docs/references.
- [x] Добавить девять блоков и соответствующие предметы. Руды: coal→coal, iron→raw_iron, gold→raw_gold, diamond→diamond. Сохранить старые drop mapping. Добавить raw_iron/raw_gold/iron_ingot/gold_ingot/diamond/leather. Добавить остальные 24 инструмента и 12 частей брони; убрать «образец» у инструментов, оставить честную пометку отсутствия защиты брони в документации.
- [x] Подключить PNG материалов и брони к общему кешу значков, внешний override + встроенный оригинал + warning/fallback. Блоки продолжают использовать кубы; не подменять их плоскими картинками из icons.
- [x] `cargo test -p game_core --locked` и тесты новых загрузчиков; форматирование; commit только собственных изменений. До Task 2 старая длина renderer mapping ожидаемо потребует расширения.

### Task 2: Extended block graphics and face mapping

**Files:** `crates/game_render/src/{atlas,materials,face_textures,icons,meshing,tests}.rs`, `assets/blocks.ron`, `apps/client/src/graphics_assets.rs`.

**Interfaces:** использует 26 стабильных блоков Task 1. Новые слои 18…32 следуют порядку supplied blocks_atlas_layout.ron, весь внутренний атлас 6×6 с 36 слоями, 17/33/34/35 пусты. FaceTextures добавляет `front: Option<u8>` с serde default; перед — axis2 negative. Старые API from_png/from_tiles сохраняют входные размеры исходного атласа; дополнительный API присоединяет новые PNG без переворота/масштабирования.

- [x] Добавить тест старого атласа/короткого mapping, новых граней и точного RGBA каждого нового слоя; увидеть отказ до реализации.
- [x] Расширить внутреннюю сборку atlas/texture array/icons на 36 слоёв, сохранить повторение и отдельные мипцепочки. Исходные публичные 18 слоёв по прежнему можно загрузить и дополнить. Встроенные defaults новых PNG также нужны при вызове старых API, используемых тестами/клиентом.
- [x] Добавить mapping 26 блоков, front только для furnace/crafting_table. Новые блоки должны отображаться и в мире, и кубами в UI. Передняя грань фиксирована; не обещать поворот печи или работающую плавку.
- [x] `cargo test -p game_render --locked`, `cargo test -p voxel-client --locked`, форматирование, commit.

### Task 3: Transactional inventory crafting 2×2

**Files:** новые `crates/game_core/src/crafting.rs`, `assets/data/recipes.ron`; изменения `game_core/src/{lib,inventory}.rs`, `apps/client/src/{inventory_client,session}.rs`, `apps/client/src/ui/inventory.rs`.

**Interfaces:** `SlotRef::Crafting(usize)` и `Inventory.crafting: [Option<ItemStack>;4]`. Ядро `RecipeBook::default/from_ron`, `preview(&[Option<ItemStack>;4], &ContentRegistry) -> Option<ItemStack>`, атомарная операция взятия результата с флагом Shift. Клиент хранит RecipeBook в Game или отдельном Resource; результат не настоящий слот хранения. InventoryAction добавляет TakeCraftResult { all: bool }.

- [x] Тесты recipes: одна log в любой клетке→4 planks; две planks вертикально в любом столбце→4 sticks; квадрат4 planks→crafting_table; квадрат4 sand→sandstone; один iron/gold/diamond block→9 соответствующего материала. Лишние клетки не совпадают. Добавить отказ неподходящего/переполненного курсора без расходования.
- [x] Реализовать проверяемый RON рецептов и операции с количеством; Shift+результат делает максимум порций в свободное место без переполнения стека. Повторная команда проверяет актуальный вход.
- [x] Интегрировать четыре обычных слота с текущими ЛКМ/ПКМ/Shift правилами и отдельный интерактивный результат в UI. Оставить исходные хотбар/броню/сундук/прокрутку. Показать названия при наведении и видимую недоступность пустого результата.
- [x] При закрытии вернуть ингредиенты в рюкзак или выбросить остатки; при смерти/keepInventory соблюдать единое владение и drain_all. Тесты закрытия с полным инвентарём, смерти, Shift/cursor/count conservation и клиентского результата.
- [x] Все core/client тесты, форматирование, commit.

### Task 4: Timed mining using exact supplied seconds

**Files:** новые `crates/game_core/src/mining.rs`, `assets/data/mining.ron`; изменения `game_core/src/lib.rs`, `apps/client/src/{player,inventory_client,session,main}.rs`, новый `mining_client.rs` и UI/progress/crack module при необходимости.

**Interfaces:** определение инструмента по whitelisted survival:<material>_<type>, отдельные duration и can_harvest. Клиентский прогресс связан с координатой, ID блока и ItemId текущего слота. InventoryAction::Break больше не позволяет обойти требуемое время в Survival; повторная проверка в consumer обязательна. Creative быстрый; BEDROCK/water в Survival запрещены.

- [x] Перенести все существующие в игре строки пользовательского TXT точно в RON; тестировать каждую ячейку, включая golden_pickaxe+iron_ore=1.30 и отсутствие дропа. Snow layer не добавлять — такого блока нет. Для dirt/wood/leaves/deepslate и остальных отсутствующих строк задать проверяемые hardness/speed defaults в том же RON.
- [x] Реализовать таймер/сброс и отдельную проверку уровня: coal wood+, iron stone+, gold/diamond iron/diamond; gold имеет harvest-level дерева. Неподходящий тип равен руке. Тесты некорректного времени, неизвестных ID, повторного завершения и неверного инструмента.
- [x] Подключить удержание ЛКМ, сброс при отпускании/смене цели/блока/инструмента/меню/фокуса/мира/смерти/режима. Перед реальным разрушением проверить луч, загрузку, цель, время и tier. Не расходовать предмет/истощение на неуспехе; звуки/дроп один раз. Добавить прогресс и трещины поверх цели, не меняя исходные PNG.
- [x] Клиентские тесты завершения/раннего Break/смены цели и старые регрессии, Clippy, commit.

### Task 5: Versioned ores, compatibility and delivery

**Files:** новые `game_core/src/ores.rs`, `assets/data/ores.ron`; изменения `game_core/src/{lib,world,persistence}.rs`, `apps/client/src/{session,streaming,main}.rs`, release version strings/docs.

**Interfaces:** Chunk::generate остаётся v1. Новый versioned generator с сохранённым OreConfig. Все клиенты фоновой генерации получают версию и snapshot мира; новые миры=2, старые без snapshot=1. Проверка checksum старого payload предшествует обогащению default полями. Не писать конфигурацию генератора2 в старый мир по умолчанию.

- [x] Тест legacy save/checksum и точной генерации1, нового save round-trip, snapshot после изменения defaults, порядка генерации соседей/отрицательных координат и высот руд.
- [x] Добавить четыре руды: coal Y0…192/20 veins/17 blocks, iron −64…72/12/9, gold −64…32/4/9, diamond −64…16/7/8. Распределение треугольное, заменяет только stone/deepslate; соседние origins проверяются детерминированно, цель жилы загружена независимо. Не внедрять остальные четыре руды прежнего большого плана без PNG в эту итерацию.
- [x] Маршрутизировать load/create/generation/save с snapshot, сохранить старые номера блоков и фоновые stream cancellations. Проверить новые размещённые блоки в правках старого мира. Состояние печи и полный инвентарь не обещать сохраняемыми.
- [x] Поставить честную версию alpha обновления и переписать README/отчёт вокруг фактически сделанного. Не помечать завершённым весь этап 4 прежнего плана (там восемь руд).
- [x] Проверить весь workspace: fmt, Clippy -D warnings, tests; native GUI сценарии; Windows GNU build/tests через Wine; Windows GUI без assets. Затем собрать уникальный ZIP/BUILD-INFO/SHA256SUMS/полный diff, отдельно опубликовать source+download refs без замены прежних выпусков и проверить реальное скачивание.

## Builder decisions and preflight

Девять блоков ограничены предоставленными ресурсами. Палка с inline-картинки не имеет доступного отдельного PNG; не перерисовывать её молча. Плавка, 3×3, прочность/защита, восемь руд целиком и полное сохранение новых систем не включены. Тесты проверяют реальные границы владения, а не только длины новых массивов.
