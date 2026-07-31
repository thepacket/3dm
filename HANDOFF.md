# Handoff log

Two assistants work on 3DM: **Claude** (Anthropic) and **Sol** (GPT-5.6). Neither
can see the other's private memory or session history, so this file is the shared
record. Andre works with whichever is available.

## Rules

- **Read this file at the start of every session**, before acting on any prior
  assumption about project state.
- **Append an entry when you finish a stretch of work.** Newest at the top of the
  log. Keep it to what the next assistant needs, not a changelog — git already has
  the diff.
- **State what you invalidated.** If you changed something a previous entry (or a
  previous assistant's assumption) described as settled, say so explicitly. That
  line is the whole point of this file.
- **Don't record what git records.** No file-by-file summaries. Record decisions,
  reversals, dead ends, and work left in flight.
- **Commit trailers identify the author.** Claude uses
  `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`; Sol should use an
  equivalent trailer so `git log` is self-explanatory.

## Entry format

```
## YYYY-MM-DD — <assistant> — <one-line summary>
**Changed:** what actually moved, and the commit range.
**Invalidates:** anything previously believed true that no longer is. "Nothing" is a valid answer.
**In flight:** unfinished work, and where it stands.
```

---

## 2026-07-31 — Claude — Set up this file

**Changed:** Added `HANDOFF.md`. No code touched.
**Invalidates:** Nothing.
**In flight:** Nothing. Repo is at `5b8912a`; Claude's private notes on project
state are anchored to that commit, so anything after it needs re-verification
before Claude relies on prior assumptions.
