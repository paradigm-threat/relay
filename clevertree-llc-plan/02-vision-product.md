# Vision and product

## Brand

**MyProblem** — product domain **myproblem.app** (registered 2026-11-07; Namecheap → Vercel DNS; live at https://myproblem.app).  
**Legal entity:** **Clevertree LLC** (Georgia, new filing — not a revival of AZ Clevertree Solutions). Public brand stays MyProblem; optional DBA **MyProblem**. GA eCorp Clevertree search returned no hits (2026-11-08).  
Earlier brand lean (**clevertree.ai**) is retired as a product domain. AZ Clevertree Solutions history is terminated entity-only.  
Contact: **ari@asu.edu**. Coming-soon repos: GitLab `clevertree/myproblem` (deploy) · GitHub `paradigm-threat/myproblem` (mirror).

## What MyProblem is

A Georgia LLC that runs a **Relay-native investigation service**: problem-solving AI (image, chat, voice) that helps people get as far as they can alone, then packages a dossier so a **local business** can help **before** any meeting — or so the person never needs to call at all.

**Local businesses sign up at $0 application / $0 subscription.** Preferred stance: **free for businesses forever.** They use MyProblem as a **web / app front door** for uncertain customers: link the shop site (or QR) to **ticket creation** (audio recording + stills → deferred Project). **We do not answer the business phone line today** — no inbound call-center / VoIP product yet. Same “deflect unsure traffic” outcome without ringing the desk.

No salaries. Investor return rides residual margin from **consumer-side** monetization after free uses (and optional payment-rail fees), not from invoicing shops.

This is the Relay **vision** (companies without salary machinery), not only Relay **infrastructure**. Relay carries Problem tips. The LLC owns the product, processor, and splits.

## Pitch to local businesses (zero application)

| Promise | Detail |
|---|---|
| **$0 to join** | No application fee, no monthly bill, no lead invoice (default forever-free for the business) |
| **Zero obligation** | Like a social network: shops may **ignore any request entirely** — no time, not a fit, don’t feel like answering. No SLA, no response timer, no score penalty, no “must choose Accept / Decline.” Silence is a valid outcome. |
| **Funnel for the unsure** | People who are not ready to book still get structured advice via **tickets** (app / site link); when they are ready, the business already has the Problem dossier |
| **Lighter front desk (no phone answering)** | Replace “call the shop first” loops with **website → MyProblem ticket** (nearly identical: speak the problem as audio, add photos). Staff are not interrupted by those rings. **Not** a phone tree we staff or AI that answers inbound PSTN/VoIP calls — that is a future investment |
| **Redirect when needed** | Legal stops + “call emergency / see a licensed pro” baked in; MyProblem is not a replacement for licensed care |
| **Ready when it is real** | If the case needs them, they open a tip with photos + AI notes already done — bid, accept, decline, **or leave it unread** |

Example (vet / clinic / trade): today the receptionist fields “is this urgent?” and “what do I do until Monday?” on the phone. With MyProblem, the shop site sends those people into a ticket instead — staff see fewer tire-kickers and warmer handoffs, without us picking up the line.

## Future: inbound VoIP (planned investment)

**Today:** tickets only (app, web, business-site deep link / QR) — audio + stills capture; no MyProblem-operated phone number for the shop.

**Want:** optional **VoIP / telephony** (Twilio-class or peer) so a published shop number or SIP trunk can land in the **same automated ticket pipeline** — speech → ticket → Project — with legal stops and no human call center.

Why it is worth investor / plan spend later:

- Fully automatable once tickets + Projects work (ASR → Evidence → queue → batch).
- Closes the gap for customers who still dial by habit; keeps businesses on the free, zero-obligation model.
- Cost is metered infra (minutes + storage), not salaries — fits the zero-payroll Relay vision.
- Do **not** buy VoIP before ticket quality and merchant ignore/accept UX are proven on web/app intake.

## Problems, not chats

The primary object is a **Problem** (live issue), not a conversation thread.

| Chat-shaped product | Problem-shaped product (ours) |
|---|---|
| Context = rolling transcript / token window | Context = the Problem record (photos, facts, stops, attempts, status) |
| User “continues the chat” | User updates or opens a Problem |
| History grows until the model forgets | Model is fed a **compiled problem brief**, not the raw log |
| One blob of ambiguity | Problems can be **split** or **combined** (like Jira) when AI or user sees two issues or one root cause |

Voice and short messages are **inputs into a Problem**, same as a photo — not the source of truth. Businesses open a Problem dossier, not a chat export.

Lifecycle (ticket-like): `open` → `packaged` → `project_queued` / `project_running` → `needs_info` / `ai_resolved` / `ready_for_provider` → `bidding` → `settled` / `declined` / `closed`.

## Projects — deferred server investigation (primary)

Canonical product doc: [`../../myproblem/docs/projects.md`](../../myproblem/docs/projects.md).

**No on-demand frontier LLM or premium vision.** Smart-enough realtime AI **captures** questions / requirements / **stills** (no server video) + **audio**, then **queues a JIRA-like ticket by status**. Tickets are **anonymous by default** (server `ticket_uuid`; APK local backup; warn: lose phone → lose tickets). **Free** secret recover email or register for durable access ([`../../myproblem/docs/anonymous-tickets.md`](../../myproblem/docs/anonymous-tickets.md)). Dossier = **Relay tip**; audio/stills in cheap object store (Bunny prefer). Deep research is a **Project**: batch ASAP — **KB git first**, HF/Spot VL + optional gen + allowed browser research; **Bedrock** = rare escalate after miss, then KB distill ([`../../myproblem/docs/backbone-escalate.md`](../../myproblem/docs/backbone-escalate.md) · [`../../myproblem/docs/knowledge-base.md`](../../myproblem/docs/knowledge-base.md)). Merchant submit anytime. Same ticket timeline for bot / humans / providers.

| Mode | Job |
|---|---|
| Realtime | Smart-enough intake — stills + coach; audio Evidence; not live Sonnet / video upload |
| **Project (primary)** | Batch investigate ASAP + completely; HF/Spot VL + optional diagram gen |
| Knowledge base | Git Markdown (PT categories) — primary learning; check before escalate |
| Backbone escalate | Rare Bedrock/API on KB misses; cache + KB distill |
| Merchant | Optional handoff; updates stay on the Problem |

## Shared local knowledge base (primary learning)

Canonical: [`../../myproblem/docs/knowledge-base.md`](../../myproblem/docs/knowledge-base.md).

The KB is a **git repo of Markdown** (Paradigm Threat patterns: **category folders**, hub `page.md`, reader articles, `investigations/` dossiers, discovery links). It exists so the app can **solve problems** — especially with **local resources** that ad-driven search rankings bury — plus DIY advice when place does not matter. **Disclaimer:** speculative; **user contributions + AI investigations only**; **no boosting / paid ranking**. Do **not** freeze-frame the greater internet; cite live URLs. Private ticket data stays private.

**Learning loop (locked):** Project workers **check the KB before** Bedrock / expensive research; **write useful non-searchable residue after**; **re-test / refine / ditch** advice that stops helping; **do not forget** (git history). Browser automation runs **where bots are allowed**, with **honest** feedback. The server LLM is tweaked only for **vision/function**.

**Compete on problem-solving, not directory scrapes:** outrun ChatGPT/Grok/Google/Yelp on *local resource wisdom* — Relay vision — instead of caching the open web or fine-tuning trivia into weights.

**Channels (opt-in):** anything with a **unique `channel_id`** — house alias, street/town, Wi-Fi locality, **Bluetooth dongle/beacon**, etc. Tickets associated with a channel **appear to anyone subscribed** to that id (no passwords/pairing secrets). More non-sensitive locality → more trust; street address and phone stay optional.

## Core loop

1. Person opens MyProblem (big actions, voice-first, ADA-capable) and lands on **Problems**.
2. **Realtime camera UX** on device; app keeps only clear/unique frames; server **rate-limits** accepts — **intake only**.
3. Ticket packaged → default **start Project** (deferred investigation) and/or submit to merchant.
4. AI may recommend **combine** or **split**; user confirms.
5. Problem-oriented AI only. Legal stops on every update. Timeline is the one-stop shop.
6. Often: Project → `ai_resolved` — person never needed to ring the shop (and we never answered a phone for them).
7. When not: local business gets the **Problem tip** and can bid / accept / decline — **or ignore it entirely** (no obligation).
8. After handshake, the deal is between them. Ignoring never creates a bill or a duty.
9. Hard cases: KB miss → allowed research / tools → rare frontier escalate inside the Project worker; answer written onto the Problem **and** into the region KB.
10. Public lessons → region KB git (categorized Markdown); optional channel post (place / Wi-Fi locality) without exposing secrets.

## Domains (start narrow, design for more)

| Phase | Domains | Note |
|---|---|---|
| Pilot | Home repair (plumbing / appliance), simple glasses/frame; optionally one clinic/vet-style triage partner | Photo + “should I call?” load |
| Parallel (KB now / product soon) | **Batched agent work** — coding, networking, docs/office, heavy media pipelines; nearly-free websites | **Not** realtime Cursor/Codex: queue → thorough finish → notify. KB website path today; product soon ([`10-batched-agent-work.md`](10-batched-agent-work.md) · [`../../myproblem/docs/batched-agent-work.md`](../../myproblem/docs/batched-agent-work.md)) |
| Later | Artists, caretakers, other local services | Same dossier pattern |
| Careful | Health / vet / mushrooms | Stop-and-redirect as product; AI lightens ticket load, does not practice medicine/vet medicine |

## Who pays (and who does not)

### Businesses — preferred: free forever

- **$0 application, $0 subscription, $0 per-lead bill** as the default product promise.
- **No obligation to respond.** Inbox is social-feed shaped: engage when it is a fit; **ignore** when busy or uninterested. Product must never imply every tip requires Accept / Call / Decline.
- ROI for them is **time saved** + **warmer bookings**, not a SaaS invoice or a lead quota.
- Charging businesses later is **optional and dangerous** to the pitch — only revisit if consumer economics cannot cover infra and investors demand it. If revisited: never surprise; never “bait free then claw back leads.” Prefer optional **priority/reliable compute** the *business* buys for *their* customers, not a tax on every lead.

### Consumers (people with Problems)

- Free uses with no card (Spot pool; “busy” when demand spikes).
- After free depth: **payee / consumer fee** for continued or priority use (reliable pool) — this is the careful monetization surface.
- Optional: small platform fee on **consumer→business payment** if/when MyProblem hosts checkout — framed as payment processing, not “we sold your name to the shop.”

### Unlimited AI

= unlimited for **solving problems** in-product, not a general everything-LLM. Soft rate limits and Spot capacity still apply on free.

## Capacity tiers (Spot vs reliable)

| Tier | Compute | When busy |
|---|---|---|
| **Free** (default consumers + all business tooling) | Rackspace **Spot** | “High demand — free scanning temporarily unavailable.” Preview may stay live. |
| **Paid consumer priority** | Reliable pool (on-demand / reserved / local colo) | No Spot dead-end; higher rate limits |

Businesses stay on the free tooling path unless they *choose* a paid priority add-on for their branded funnel (optional later).

## UX principles

- Beyond chat: **Problem list + detail**, voice, checklist.
- Image gen to **explain**, not invent.
- ADA / older adults from day one.
- Trust = work + legal stop on screen.
- Combine/split confirmable in plain language.
- Business dashboard: incoming Problems, triage stats (“deflected from call-first habit via tickets”), accept/decline/ignore — no invoice page by default. No claim that we answered their phone.

## Economic machine (no salaries)

| Path | Who pays | Who gets paid |
|---|---|---|
| Solo AI / triage | Nobody (growth burn) or later consumer priority fee | Backbone + Spot as cost |
| Business signup & tooling | **Nobody** (forever-free preference) | — |
| Consumer priority after free uses | Consumer (payee fee) | Infra residual → LLC distributions |
| Optional hosted checkout | Small % on payment rail | Same residual path |
| Investor | Capital in | Preferred return then profit share from residual |

Founders take per-settlement / per-priority operator cuts — not a salary pile.  
**Do not** build the investor model on “we will start billing plumbers and vets in year two” unless that is an explicit, reluctant Plan B.

## Entity

Arizona Clevertree Solutions, LLC was terminated (2022). Form a **new Georgia LLC** named **Clevertree LLC**, new EIN, bank, processor; consumer brand remains **MyProblem** / myproblem.app. Do not revive the Arizona filing.
