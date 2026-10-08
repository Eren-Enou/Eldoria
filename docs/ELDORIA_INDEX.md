# Eldoria Project Index

> **Project status:** Active development  
> **Working title:** Eldoria  
> **Purpose:** Canon index, documentation map, and authority guide for the Eldoria setting and the World of Individuals simulation project.

---

# 1. Purpose of This File

This file is the starting point for understanding the Eldoria project.

It does **not** contain the complete lore of the setting.

Instead, it defines:

- where established information belongs,
- which documents are authoritative,
- how canon status is recorded,
- which major subjects have already been developed,
- which subjects remain unresolved,
- and how the fictional setting relates to the World of Individuals simulation.

When answering questions about established Eldoria lore, consult the relevant dedicated document rather than relying only on previous conversations.

If a conversation conflicts with an established canon document, identify the conflict rather than silently replacing the documented material.

---

# 2. Canon Authority

The project uses the following hierarchy of authority.

## 2.1 Established Canon Files

Dedicated worldbuilding files are the primary authority for established setting information.

Examples:

- `COSMOLOGY.md`
- `OCEANIDS.md`
- `MOON_CLAN.md`
- `MUDMEN.md`
- character and concept files

Material explicitly marked **Established** or **Canon** in these files should be treated as current canon.

---

## 2.2 Tentative Material

Ideas marked **Tentative** are currently favored possibilities but may change.

Tentative material should not be presented as established fact.

---

## 2.3 Speculation

Ideas marked **Speculation** are exploratory possibilities.

They exist to support brainstorming and should not be treated as part of the world unless explicitly promoted.

---

## 2.4 Open Questions

Material marked **Open Question** is intentionally unresolved.

Future discussions should explore these questions rather than assuming an answer.

---

## 2.5 Conversations

Chat discussions are developmental material.

A concept being discussed, praised, or explored in a conversation does **not** automatically make it canon.

Canon changes only when the project owner accepts the idea and it is incorporated into the appropriate source document or otherwise explicitly designated as established.

---

# 3. Status Terminology

Use these labels consistently throughout project documentation.

### Established

Accepted as part of the current setting.

### Tentative

Currently favored but subject to revision.

### Speculation

An idea being explored without commitment.

### Open Question

A question deliberately left unresolved.

### Superseded

Older material retained for historical reference but no longer considered current.

---

# 4. Project Structure

Eldoria currently consists of two related but distinct projects.

## 4.1 Eldoria Worldbuilding

The fictional universe, including:

- cosmology,
- metaphysics,
- civilizations,
- characters,
- history,
- magic,
- geography,
- technology,
- religion,
- artifacts,
- politics,
- culture,
- and the central narrative.

The fictional world must make sense independently of the central plot.

Civilizations should have their own histories, institutions, incentives, families, economies, conflicts, beliefs, and ordinary lives.

---

## 4.2 World of Individuals

A Rust + Bevy artificial-individual and history simulation.

Its purpose is to explore mechanisms capable of producing:

- believable individuals,
- persistent personal history,
- subjective interpretation,
- communication,
- social relationships,
- information propagation,
- emergent social behavior,
- institutions,
- and eventually historical societies.

The simulation is **not required to recreate Eldoria's history**.

Simulation experiments should test mechanisms and discover consequences rather than hardcode desired fictional outcomes.

Simulation results may inspire or challenge Eldoria worldbuilding, but simulation results do not automatically establish fictional canon.

Repository:

`Eren-Enou/Eldoria`

---

# 5. Recommended Documentation Structure

The following structure is the current recommended organization.

```text
ELDORIA_INDEX.md

world/
    COSMOLOGY.md
    THEMES.md
    MAGIC.md
    HISTORY.md
    GEOGRAPHY_AND_COSMOS.md

civilizations/
    OCEANIDS.md
    MOON_CLAN.md
    MUDMEN.md
    PIRATES.md
    MOBILE_LABYRINTH_SOCIETY.md

characters/
    LIFE.md
    THE_END.md
    PRISONER.md
    MOON_PRIESTESS.md
    TRICKSTER.md
    CREATOR.md

concepts/
    GATE.md
    SCAR.md
    BOOK_OF_THE_MOON.md
    RESONANCE.md
    SOULS.md
    MELDING.md
    CELESTIAL_FRAGMENTS.md
    ANCIENT_ARK.md

simulation/
    SIMULATION_CONSTITUTION.md
    WORLD_OF_INDIVIDUALS.md
    EXPERIMENT_HISTORY.md

development/
    OPEN_QUESTIONS.md
    TERMINOLOGY.md
    TIMELINE.md