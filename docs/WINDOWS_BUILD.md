# Windows x64: сборка и проверка

8 ноября 2026 собрана оптимизированная Windows x64 версия текущего творческого фундамента (pre-1.0).

## Артефакты текущей облачной сессии

- `/workspace/artifacts/VoxelSandbox.exe` — самостоятельный PE32+ x86-64 EXE (~103 MiB).
- `/workspace/artifacts/VoxelSandbox-Windows-x64.zip` — EXE, инструкция, команды запуска и контрольная сумма (~36 MiB).

Это локальные артефакты сессии, не опубликованный GitHub release. Нужны Windows 10/11 x64 и актуальный графический драйвер Vulkan/DirectX 12. Текстуры/шрифты/шейдеры встроены. Проверка PE-импортов не обнаружила сторонних DLL (MinGW runtime DLL не требуются). Подсистема — Windows GUI, отдельная консоль не открывается.

SHA-256 EXE:

```text
d44c46b9223b2fcf7380d5edd5532e9e0d7e22152e1a1ea2a3eb1735191ad96a
```

## Проверка

- Кросс-компиляция: `cargo build --release --locked --target x86_64-pc-windows-gnu -p voxel-client`, exit0.
- Rust1.99.1, MinGW GCC14.2.0, release opt-level2, codegen-units16, LTO выключен, debug-символы удалены.
- 40 Windows-тестов `game_core` выполнены через Wine10.0: 40 passed, 0 failed, exit0. Unix-тест симлинков не компилируется в Windows-конфигурации.
- Реальный release EXE через Wine/Vulkan/llvmpipe открыл меню, создал и отобразил мир, открыл паузу, сохранил мир и штатно завершился. Проверены основной файл сохранения и резервная копия; exit0. Скриншот: [мир Windows EXE](screenshots/windows-world.png).
- **Настоящая Windows-машина и аппаратная видеокарта не проверены.** Wine не заменяет этот этап QA. Производительность на целевом компьютере не измерена.

Обычный Wine/DX12-запуск в этой среде завершился `RequestDeviceError::Lost`; переход на Vulkan устранил ошибку устройства. Для Wine/Xvfb также понадобился виртуальный рабочий стол из-за нулевой частоты монитора в Xvfb. Это ограничения тестовой среды, а не доказательство поведения на настоящем Windows.

## Воспроизводимость в подготовленной среде

Локальная команда:

```bash
bash /workspace/.voxel-env/build-windows.sh
```

Скрипт использует Rust из `/workspace/.rustup`, Cargo из `/workspace/.cargo`, распакованный MinGW из `/workspace/.voxel-env/sysroot`. Все пакеты скачаны с проверкой подписи/контрольной суммы Debian, Rust-компоненты — через rustup с обычной проверкой. Выходной каталог: `/workspace/.voxel-env/windows-target/x86_64-pc-windows-gnu/release/`.

На другой Linux-машине установите Rust target `x86_64-pc-windows-gnu` и MinGW-w64; задайте `CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER`, target-specific `CC`/`AR` и запустите ту же Cargo-команду. На Windows предпочтителен обычный MSVC toolchain с C++ Build Tools; его сборка пока не выполнялась.

Разархивируйте пакет в доступную для записи папку. Мир хранится в `saves/` относительно рабочей директории. `Start-with-log.bat` сохраняет лог запуска; `Start-Vulkan.bat` явно выбирает Vulkan вместо автоматического выбора backend.
