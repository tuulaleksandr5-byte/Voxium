# Minecraft Inventory Screen Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox syntax.

**Goal:** Replace side-column Voxium inventory UI with Minecraft-like interactive screens.
**Architecture:** A screen-kind enum defines the board size and canonical slot coordinates. Bevy absolute `Node` children place reusable clickable slots over an optional non-pickable skin image.
**Tech Stack:** Rust 1.99.1, Bevy 0.20.1, existing item and block icons, existing `GuiSkins`.
**Spec:** docs/superpowers/specs/2026-11-13-minecraft-inventory-layout-design.md

## Constraints
- Branch from PR #4; preserve Minecraft ZIP/OptiFine GUI loader and game inventory logic.
- Keep current inventory actions and components.
- Use optional resource-pack PNG as actual panel background, not separate preview.
- No changes to saves, game-core slot model, or main branch.

## Task 1: Screen layout and target tests
- [ ] Add UI tests for personal, workbench, furnace and chest slot counts and canonical coordinates; inspect failing CI.
- [ ] Implement `InventoryView`/layout helpers, square 54px targets and results with Bevy absolute placement.
- [ ] Verify tests and record commit.

## Task 2: Pixel panel and resource-pack background
- [ ] Test GUI background presence and non-pickability; avatar preview exists for personal inventory only; closed station extras absent.
- [ ] Draw fallback blocky panel/avatar; insert selected GuiSkins behind targets; update card constraints and creative catalogue.
- [ ] Re-run tests and record commit.

## Task 3: CI packaging and compatibility verification
- [ ] Run fmt, check, strict clippy and full workspace tests; Windows x64 release build in GitHub Actions.
- [ ] Fix regressions and document layout and remaining differences vs Minecraft.
- [ ] Provide PR and EXE only if artifact produced.
