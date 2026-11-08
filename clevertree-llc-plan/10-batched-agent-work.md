# Batched agent work — coding, networking, documents, media (vs realtime IDEs)

**Status:** **Major coming-soon product bet.** Knowledge and website copy should already advertise it. **Not shipped** as a self-serve MyProblem ticket type yet — it rides the same **Project / Intake / Spot batch** architecture as investigation.

Canonical product note: [`../../myproblem/docs/batched-agent-work.md`](../../myproblem/docs/batched-agent-work.md).  
Related: [`02-vision-product.md`](02-vision-product.md) · [`09-rackspace-spot-vision-prototype.md`](09-rackspace-spot-vision-prototype.md) · [`05-spend-and-use-of-funds.md`](05-spend-and-use-of-funds.md) · [`../../myproblem/docs/coding-and-websites.md`](../../myproblem/docs/coding-and-websites.md).

---

## What users already get elsewhere

Products like **Cursor** and **Codex-class** coding agents optimize for **realtime** (or near-realtime) interactive loops: fast context, chatty iteration, human in the loop every few seconds.

That is valuable. It is **not** what MyProblem is trying to be first.

## What MyProblem will offer instead

**Batched, automated completion** on a **Problem ticket**:

1. Person writes **proper instructions** (and optional repo / files / credentials via safe channels).  
2. Ticket joins the **Project queue** (same honesty as investigation: ETA, capacity, notify when ready).  
3. When a **Batch** worker wakes (Spot / MCP):  
   - Pull the repo or workspace  
   - Read the ask and the tree  
   - **Finish the job completely** — implement, harden, test, lint, fix failures  
   - Deploy or publish artifacts when asked  
   - Update or close the ticket with a clear report  
4. Tear down **ephemeral scratch** (local clones, render caches, temp media) so we are not storing giant project trees forever.  
5. Person gets a **notice when it is ready** — not a live pair-programming session.

Difference in one line: **set it in motion → thorough automated finish later**, not **stare at a streaming chat until it is done**.

---

## Scope (same worker family)

| Kind | Examples | Batch shape |
|---|---|---|
| **Programming** | Features, fixes, tests, hardening, PR/commit, deploy | Clone → agent → CI/tests → push/deploy → report |
| **Networking / infra** | DNS park to free host, TLS, simple MCP/Spot wiring, env setup | Same; secrets via vault/DM, not tip.json |
| **Documents** | Write/edit docs, proposals, README, policy drafts | Files in/out on object store or repo |
| **Office data** | Spreadsheets, CSVs, transforms LLMs are expected to do | Ephemeral workspace; return results |
| **Heavy media / NLE pipelines** | Timeline projects, encode final outputs, verify, upload artifact, **delete scratch** | Large Spot disk + local scratch; never keep source masters on mesh tips |

Document and spreadsheet help is in-scope as ordinary “LLM expected work.” Media/render jobs follow the same **batch → verify → publish → purge cache** discipline as other heavy Projects (internal ops experience with NLE/ffmpeg-style pipelines informs this; **no public association** with experimental edit repos).

---

## Resources — how much more do we need?

We **reuse** what is already designed: Intake always-on, ephemeral **Spot** Batch, MCP/agent tooling, git SoT tips, object store for blobs, notify on ticket update.

| Need | Already have / planned | Incremental for batched agent work |
|---|---|---|
| Queue + honest ETA | Projects / job files | Longer-running job states; “agent” / `codegen` / `render` bins |
| Cheap burst compute | Spot A30/CPU pools | **Larger scratch disks** for clone+test or encode; tear-down alarms |
| Secrets | Intake secrets / DM | Per-ticket deploy keys or OAuth to user repos (high care) |
| Models | Mid LLM + rare Bedrock | Coding-capable model budget on Batch; cache prompts in KB later |
| Networking help | KB + docs | Same Batch; no always-on IDE GPU |
| Notify | In-app / email when identified | Required for “done later” UX |

**Rough burn (order-of-magnitude, not a quote):** early pilots can stay inside existing Spot + API envelopes if jobs are capped and queued. Material add-ons are **scratch storage**, **longer Spot lease windows**, and **model tokens per completed ticket** — not a second realtime IDE fleet. Prefer **one thorough Batch pass** over many interactive turns (often cheaper and more complete).

**Ready today?** Architecture: **mostly yes** (queue + Spot + agents exist in ops). Self-serve product: **not yet**. Speed of work: **near-future / next major slice after Intake+KB honesty** — can move as soon as Project bins for `agent`/`render` are wired and a closed beta ticket type exists. No fake ship date.

---

## Product promises (copy-safe)

- **Do say:** batched automated coding and document/media work; thorough finish; notify when ready; nearly-free website path already in KB; check back soon.  
- **Do not say:** realtime Cursor replacement; instant codegen; unlimited free GPU encodes; association with unrelated experimental media repos.

---

## Implementation checklist (docs → code)

1. Ticket kinds / job bins: `agent_code`, `agent_docs`, `agent_office`, `agent_render` (names TBD).  
2. Worker: clone → plan → implement → test → deploy/upload → tip update → purge scratch.  
3. Caps: max Spot hours, max disk, max tokens per ticket; busy = wait (honest).  
4. Secrets path: never commit tokens into tip git.  
5. Website + KB: major “coming soon” callout (this plan segment).  
6. Pilot: internal dogfood on MyProblem’s own repos before public coding tickets.

*Last updated: 2026-11-08*
