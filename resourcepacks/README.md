# Ресурс-паки Voxium (формат в духе OptiFine / Minecraft)

Положите папку пака в `resourcepacks/` рядом с `Voxium.exe` (или в рабочую папку). Игра применяет все
включённые паки при запуске; поздний пак перекрывает ранний (сортировка по `priority`, затем по имени).

```
resourcepacks/МойПак/
  pack.json
  assets/voxium/textures/block/<имя>.png
  assets/voxium/models/block/<блок>.json
```
`assets/minecraft/...` читается так же (удобно переносить пакеты как есть; `grass_block_top`/`grass_block_side`
соответствуют `grass_top`/`grass_side`).

## pack.json (необязателен)
```json
{ "name": "Мой пак", "description": "…", "pixel_art": true, "priority": 0, "enabled": true }
```
* `pixel_art: true` — резкие пиксели вблизи (nearest), мипмапы вдали остаются.
* Текстуры — любые квадратные PNG от 1×1 до 4096×4096: меньшие увеличиваются **ближайшим соседом** (пиксели не
  размываются), большие уменьшаются усреднением.

## Имена текстур
`grass_top grass_side dirt stone deepslate sand gravel oak_log oak_log_top oak_leaves oak_planks cobblestone glass
water snow sandstone sandstone_top gold_ore coal_ore diamond_ore gold_block diamond_block iron_ore iron_block
crafting_table_top crafting_table_front crafting_table_side crafting_table_bottom furnace_front furnace_front_on
furnace_side furnace_top`

## 3D-модели блоков
Файл `<блок>.json` (`crafting_table`, `furnace`, `chest`, `stone`, … — имя блока в нижнем регистре через `_`).
Подмножество формата моделей Minecraft: кубоиды в единицах 1/16 блока.
```json
{ "textures": { "top": "crafting_table_top", "side": "crafting_table_side" },
  "elements": [ { "from": [0,12,0], "to": [16,16,16],
      "faces": { "up": {"texture": "#top"}, "north": {"texture": "#side", "uv": [0,0,16,4]} } } ] }
```
Грани: `up down north south east west` (north = −Z — «перед» блока). `uv` = `[u0,v0,u1,v1]` в пикселях 0…16, v вниз.
Ограничения этой версии: нет вращения элементов (`rotation`), `display`, `parent`/наследования и анимации;
коллизия, подсветка при наведении и добыча остаются как у полного куба; модель использует текстуры атласа из списка выше.
Файлы с ошибками пропускаются с предупреждением в логе.

Пример: `Voxium-3D-Example` (верстак на ножках, сундук с крышкой).
