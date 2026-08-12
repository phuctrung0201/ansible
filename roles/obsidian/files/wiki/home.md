# Vault index

Personal wiki at `~/wiki`. Single source of truth for conventions — read before creating/moving files, don't invent rules or split this back into `_meta/` files.

## Layout

```
~/wiki/
├── _templates/   # journal, literal, permanent (hidden)
├── _assets/      # diagrams, logs (hidden)
├── journals/     # YYYY-MM-DD.md — fleeting, dated
├── literals/     # process records — runbooks, designs, recaps
├── permanents/   # Title.md — templates, handbooks, guidelines, one idea, densely linked
└── home.md       # this file
```

## Journals

No frontmatter, two sections only:

- **Todo** — checklist (`- [ ]`/`- [x]`), terse **verb + link** to a `literals/` note, no trailing notes.
- **Fleet** — plain bullets, no checklist, no linking discipline; short fragments, optionally promoted later. When promoted, log it the same day: `Move [[note]] to literal` or `Move [[note]] to permanent`.

Journals link out to `literals/`/`permanents/`, never the reverse.

```markdown
## Todo
- [ ] Merge traffic/deployment alerts PR for RMS-125898 IBR submission alert implementation

## Fleet
- Idea: cache invalidation race might explain the writer failover flakiness
```

## Literals

Process records tied to a specific piece of work — runbooks, design docs, recaps, ticket follow-ups, execution records. If it documents something that happened or was decided for a particular effort, it's a literal. Frontmatter (template `_templates/literal.md`) is `finished` only — tracks whether the record is complete; no `permanents:` property.

Ticket-prefixed notes (e.g. `RMS-125898 ...`) never link out to their own status, project, or operations index — links only go parent → child. Seed status/project notes (in `permanents/`) link out to the literals currently at that status or common to that background; edit the seed note to change a literal's status, not the literal itself.

## Permanent

Durable, reusable reference — templates, handbooks, guidelines, how-tos, and other one-idea notes meant to outlive any single piece of work. If you'd reach for it again on unrelated future work, it's a permanent, not a literal.

Flat folder, one idea per note, no ID prefix (filename is the title), no frontmatter — just the note. Relations are captured only via inline `[[wikilinks]]`, parent → child (a note never links back to something that already links to it). A handful of seed notes anchor the graph (status, operations index, project background) — each documents its own rules, read them directly rather than duplicating here.

## Obsidian settings

Daily notes folder: `journals/` (template `_templates/journal`). Excluded from explorer: `_templates/`, `_assets/`.
