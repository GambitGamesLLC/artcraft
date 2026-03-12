# Consolidate ArtCraft Plans → Single Canonical Plan

**Date:** 2026-03-08  
**Status:** ✅ Complete  
**Agent:** Chip 🐱‍💻

---

## Goal

Merge the relevant content from the two existing ArtCraft plans into a single canonical plan, then **HARD DELETE** the older plans from disk.

---

## Context

Plans consolidated:
1. `plans/artcraft/artcraft-external-ipc-control-2026-03-07.md` (external IPC/CLI scope; Phase 1–3 complete; Phase 4 pending)
2. `plans/artcraft/2026-03-08-artcraft-install-bugfixes.md` (current execution blocker: Linux build/install; RPM bundling hang)

---

## Approach

- Create a **new canonical plan** that:
  - Leads with **current execution blocker**: Linux build/install blocked by RPM bundling hang.
  - Preserves key **branch names / commit IDs / critical commands**.
  - Includes the external IPC/CLI scope and status (Phase 1–3 ✅, Phase 4 ⏳).
  - Adds a clear **“Resume Here”** section with exact commands.
- Then **HARD DELETE** the two older plan files (per requirement).  

**Key decision during merge:** original draft suggested archiving old plans, but the requirement for this run was explicit **hard delete**, so we deleted them instead of moving to an archive folder.

---

## Tasks

### Task 1: Draft canonical merged plan

**Status:** ✅ Complete

**Files created:**
- `plans/artcraft/2026-03-08-artcraft-canonical.md`

**Notes:**
- Canonical plan leads with RPM bundling hang blocker.
- Includes external IPC/CLI Phase status (1–3 complete; 4 pending).
- Includes “Resume Here” with exact commands.

---

### Task 2: Remove old plans (HARD DELETE)

**Status:** ✅ Complete

**Files deleted:**
- `plans/artcraft/2026-03-08-artcraft-install-bugfixes.md`
- `plans/artcraft/artcraft-external-ipc-control-2026-03-07.md`

---

### Task 3: Verify + report

**Status:** ✅ Complete

**Verification performed:**
- Confirmed canonical plan exists and is readable.
- Confirmed both old plan files are removed from disk.

---

## Final Results

**Status:** ✅ Complete

**Canonical Plan Path:**
- `plans/artcraft/2026-03-08-artcraft-canonical.md`

*Completed: 2026-03-08*
