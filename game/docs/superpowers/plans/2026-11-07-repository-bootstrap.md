# Repository Bootstrap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Подготовить Rust/Bevy репозиторий, на основе которого команда сможет независимо начать работу над версией 1.0.

**Architecture:** Cargo workspace с Bevy-клиентом и независимым core. Остальные crates добавляются по мере реализации. Исходная спецификация сохраняется в репозитории.

**Tech Stack:** Rust 2024, Bevy 0.20.1, Cargo, GitHub Actions.

**Spec:** `docs/ORIGINAL_REQUIREMENTS.md`, `docs/ROADMAP.md`, `docs/ARCHITECTURE.md`.

## Global Constraints

- Высота мира Y=-64..=320, секции 16×16×16.
- 1.0 = простой играбельный мир, не полный релиз всего контента.
- Никаких утверждений о результате сборки без фактического выполнения.

## Review Focus

- Отрицательные координаты должны использовать евклидово деление.
- Верхняя граница Y=320 включительна; 321 запрещена.
- До интеграции новых crates интерфейсы game_core согласовываются.
- Пользовательские сохранения нельзя повреждать неатомарной записью.
- В GitHub Actions должны запускаться check, fmt, clippy и test.

---

### Task 1: Bootstrap app + core

**Files:** `Cargo.toml`, `apps/client/Cargo.toml`, `apps/client/src/main.rs`, `crates/game_core/src/lib.rs`.

**Interfaces:** `section_coordinate(i32)->i32`, `local_coordinate(i32)->u8`, `valid_world_height(i32)->bool`.

- [ ] Сначала написать тесты отрицательных координат, обратимости и пределов высоты.
- [ ] Убедиться, что проверки первоначально обнаруживают недостающую реализацию.
- [ ] Реализовать минимальные функции и запуск окна.
- [ ] Выполнить `cargo test --workspace` и `cargo run -p voxel-client`.
- [ ] Зафиксировать проверенный коммит.

### Task 2: Architecture and roadmap

**Files:** `docs/ARCHITECTURE.md`, `docs/ROADMAP.md`, `docs/PARALLEL_WORK.md`.

- [ ] Проверить границы модулей и независимые направления.
- [ ] Привязать каждую будущую функцию к версии.
- [ ] Документировать критерии завершения 1.0.
- [ ] Зафиксировать проверенный коммит.

### Task 3: CI

**Files:** `.github/workflows/rust-ci.yml`.

- [ ] Создать CI workflow.
- [ ] После загрузки на GitHub проверить реальный запуск workflow.
- [ ] Исправить выявленные ошибки.
