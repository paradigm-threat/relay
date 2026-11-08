# Step-by-step plan

Ordered path from “docs on disk” to GA + Israel pilot to optional raise.

## Phase 0 — Paper (now)

- [x] Competition landscape (`01-competition.md`)  
- [x] Vision / product (`02-vision-product.md`)  
- [x] Investor approach + test pools (`03-investors-and-test-pools.md`)  
- [x] Georgia LLC notes (`04-georgia-llc.md`)  
- [x] Spend categories (`05-spend-and-use-of-funds.md`)  
- [x] Infra options (`06-infrastructure.md`)  
- [x] Realtime vision $$$ (`08-realtime-vision-infra-cost.md`)  
- [x] Rackspace Spot prototype plan (`09-rackspace-spot-vision-prototype.md`)  
- [x] Brand lean: **MyProblem** / **myproblem.app** (businesses $0 forever preferred; clevertree.ai retired)  
- [x] Register **myproblem.app** + coming-soon site on Vercel (GitLab deploy)  
- [x] Georgia SOS name search for **Clevertree** (2026-11-08: no data found; confirm at filing)  
- [x] File Articles for **Clevertree LLC** on eCorp (2026-11-08) — SOS confirmation email + receipt PDF on `ari.asulin@gmail.com`; eCorp user **Protricity**  
- [ ] **Acceptance + Certificate of Organization** (not yet in Gmail as of 2026-11-08 evening)  
- [ ] EIN + business bank after acceptance  
- [ ] One-page investor teaser (after pilot metrics, or soft pre-ask)  

## Phase 0.5 — Rackspace Spot vision benchmarks (before / parallel to entity polish)

See [`09-rackspace-spot-vision-prototype.md`](09-rackspace-spot-vision-prototype.md), MyProblem MVP [`../../myproblem/docs/spot-ai-mvp.md`](../../myproblem/docs/spot-ai-mvp.md) (SEND frames, Relay `relay.ai.*` flags, free vs paid pool), and **Projects** [`../../myproblem/docs/projects.md`](../../myproblem/docs/projects.md) (deferred batch investigation = primary server path; realtime = ticket capture only).

1. Spin Spot **IAD CPU** (ingest/keyframes) + **SJC GPU A30** (`gpu.vs2.xlargeplusplus-sjc`, ~$0.15/hr spot)  
2. Optional **LON CPU** for Israel/EU RTT  
3. Bench realtime video/file feed → frames → OCR/VL → Problem patch; log p95 + $/scan  
4. Frontier escalate sample set → learn taxonomy  
5. Tear down when idle; write results under `bench/` and update `08` numbers  
6. Cap sprint ~**$50–150** Spot + **$50–150** APIs unless extended  

**Exit:** Know whether A30 Spot meets realtime targets, or GA should stay CPU+API with GPU for batch distill only.

## Phase 1 — Entity and rails

1. ~~Confirm Georgia LLC name available~~ — **Clevertree LLC**  
2. ~~File Articles~~ — submitted eCorp 2026-11-08 (confirmed by SOS email); **still awaiting acceptance + certificate**  
3. EIN; business bank account; optional DBA **MyProblem**  
4. Draft operating agreement (no-salary + distribution + Relay license language)  
5. Privacy policy + terms (**Terms** + **Privacy** live on myproblem.app; not a clinic; photo retention)  
6. Basic insurance quote  
7. Stripe (or similar) account in LLC name — test mode  

## Phase 2 — Build pilot (Cursor + Relay)

1. PWA: **Problem list/detail** — live camera / What is this / Find help / Voice (inputs attach to a Problem)  
2. **On-device frame gate** (clarity + uniqueness) + **server rate limit** on accepts; route **free → Spot pool** / **paid → reliable pool** with busy copy for free when Spot is down  
3. Problem schema on **Relay tip** (`ticket_uuid`); thin queue index; **anon default** + free recover email ([`../../myproblem/docs/anonymous-tickets.md`](../../myproblem/docs/anonymous-tickets.md)); Project jobs + InvestigationUpdate; brief compiler  
4. Realtime agent = **smart-enough capture** (stills only, no video); audio → object store (Bunny prefer); first ASR + LLM retranscription gate  
5. **Project batch worker** drains status queue ASAP; **KB first**; **Bedrock/frontier** rare escalate after miss → KB write ([`../../myproblem/docs/backbone-escalate.md`](../../myproblem/docs/backbone-escalate.md) · [`../../myproblem/docs/knowledge-base.md`](../../myproblem/docs/knowledge-base.md))  
6. Image preprocess + agent returns **Problem patches** + combine/split (coach live; deep patch in Project)  
7. Each Problem as Relay tip (or Relay-linked store)  
8. Legal-stop fields on the Problem every update  
9. Provider “open Problem” link + bid/decline (always optional beside Project)  
10. Notify: anon in-app poll; email/push when recover email or account attached  
11. Soft caps / TTL for unclaimed silent anon tickets; object-storage $/GB in infra burn  
12. **Local KB:** redacted publish from Projects; GA/Israel partition; channels by unique id  
13. Phone verify; soft daily budgets  
14. Accessibility pass (screen reader, contrast, large targets)  

**Verticals locked for pilot:** e.g. plumbing leak / fixture + glasses frame only.

## Phase 3 — Georgia soft launch

1. Pitch 5–15 local businesses: **$0 application, free forever** — website→ticket deflection (we do **not** answer their phone yet) + dossier when real; zero obligation to reply  

2. Zero billing setup for them; dashboard for incoming Problems / deflect stats  
3. Seed consumers with no ads (friends, Nextdoor carefully, local groups)  
4. Log: scans, solo close (never called the shop), dossiers sent, opens, accepts, consumer priority conversions, backbone $  

**Exit criteria to expand:** businesses stay because unsure traffic lands in tickets instead of ringing the desk; consumers resolve alone or hand off with a file; cost per useful scan known without invoicing shops. Revisit **inbound VoIP** only after that loop is proven.

## Phase 4 — Israel soft launch

1. Brother activates as bounty / settlement-cut operator  
2. Hebrew (or bilingual) simple UI strings  
3. Same verticals; local provider list  
4. Compare trust / stop-language / conversion to GA  

## Phase 5 — Raise or don’t

**Raise** if: unit economics sketch works and free depth is the bottleneck.  
**Don’t raise** if: providers won’t bid from dossiers — fix product first.

Raise materials: one pager + this folder + live metrics from Phases 3–4. Instrument per `03-investors-and-test-pools.md`.

## Phase 6 — Aggressive free campaign (post-metrics)

1. Increase backbone budget  
2. Light ads in GA / Israel  
3. Keep fraud loose; watch cost curves weekly  
4. Kill or throttle if settled-match revenue cannot cover free burn within agreed runway  

## Phase 7 — Later (explicitly out of pilot)

- More trades  
- Licensed video consults paid per case  
- Image-gen explain features at scale  
- Second legal entity if Israel/EU volume demands it  
- Health beyond stop-and-redirect only with counsel + insurance  

## Decision log (fill as you go)

| Date | Decision | Why |
|---|---|---|
| 2026-11-07 | Docs folder created; NAS sync on nobiserv home (not public) | LLC plan workspace |
| | Georgia not Arizona revive | AZ entity terminated |
| | GA + Israel first tests | Home LLC + brother on ground |
| | No salaries | Vision + investor ROI story |
| 2026-11-07 | Context = Problem (Jira-like), not chat window | Combine/split; brief compiler; Relay tip per Problem |
| 2026-11-07 | Phase 0.5 Rackspace Spot benches | SJC A30 GPU Spot + IAD/LON CPU; realtime feed→context |
| 2026-11-07 | **Projects** = deferred batch investigation (primary) | Realtime = ticket capture only; see myproblem `docs/projects.md` |
| 2026-11-07 | **Shared local KB** + place / Wi-Fi channels | GA/Israel only; redacted public docs; see myproblem `docs/knowledge-base.md` |
| 2026-11-07 | No on-demand frontier; Bedrock = rare batch escalate | Capture → queue → batch ASAP; see `docs/backbone-escalate.md` |
| 2026-11-08 | KB = primary learning (git Markdown, PT categories) | Check KB before escalate; research/Bedrock write KB; browser honesty; LLM tweaks = vision/function only |
| 2026-11-08 | KB = problem-solving only; no boost; no web cache | Speculative user+AI; local resources + DIY; re-test/refine/don’t forget; site+terms copy |
| 2026-11-08 | Nearly-free website KB + coding assist planned | Vercel free + DNS; LLM build/deploy; Spot/MCP; MyProblem site-build coming soon |
| 2026-11-08 | Batched agent work (vs realtime IDE) | Plan `10`: thorough Project coding/docs/render; Spot scratch; major coming-soon site copy |
| 2026-11-08 | Public **/privacy** + Terms filing-status wording | PII client-held policy on site; Articles pending SOS accept |
| 2026-11-07 | Anon tickets + Relay tip SoT + Bunny/S3 audio | Free recover email; no server video; see `docs/anonymous-tickets.md` |
| 2026-11-07 | Israel competition refresh in `01` | Hendiman / DontSearch / AI Trust; LLM stack same as US |
