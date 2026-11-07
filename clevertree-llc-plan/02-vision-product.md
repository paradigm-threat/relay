# Vision and product

## Brand

**MyProblem** — product domain **myproblem.app** (registered 2026-11-07; Namecheap → Vercel DNS; live at https://myproblem.app).  
Georgia LLC name: prefer **MyProblem, LLC** or **My Problem LLC** after SOS check; DBA **MyProblem** if the legal string needs a designator tweak.  
Earlier brand lean (**clevertree.ai**) is retired — do not buy it for this product. AZ Clevertree Solutions history is entity-only, not the public name.  
Contact: **ari@asu.edu**. Coming-soon repos: GitLab `clevertree/myproblem` (deploy) · GitHub `paradigm-threat/myproblem` (mirror).

## What MyProblem is

A Georgia LLC that runs a **Relay-native investigation service**: problem-solving AI (image, chat, voice) that helps people get as far as they can alone, then packages a dossier so a **local business** can help **before** any meeting — or so the person never needs to call at all.

**Local businesses sign up at $0 application / $0 subscription.** Preferred stance: **free for businesses forever.** They use MyProblem as a front door for uncertain customers and as AI triage so routine “should I…?” questions never hit the secretary / front desk.

No salaries. Investor return rides residual margin from **consumer-side** monetization after free uses (and optional payment-rail fees), not from invoicing shops.

This is the Relay **vision** (companies without salary machinery), not only Relay **infrastructure**. Relay carries Problem tips. The LLC owns the product, processor, and splits.

## Pitch to local businesses (zero application)

| Promise | Detail |
|---|---|
| **$0 to join** | No application fee, no monthly bill, no lead invoice (default forever-free for the business) |
| **Funnel for the unsure** | People who are not ready to book still get structured advice; when they are ready, the business already has the Problem dossier |
| **Lighter front desk** | Many “call the vet / call the shop” loops are questions AI can triage or redirect better than a phone tree — those people never reach the secretary |
| **Redirect when needed** | Legal stops + “call emergency / see a licensed pro” baked in; MyProblem is not a replacement for licensed care |
| **Ready when it is real** | If the case needs them, they open a tip with photos + AI notes already done — bid, accept, or decline |

Example (vet / clinic / trade): receptionist today fields “is this urgent?” and “what do I do until Monday?” MyProblem absorbs the repeatable layer; staff see fewer tire-kickers and warmer handoffs.

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

**Realtime AI only captures the ticket** (camera coaching + clarifying questions). Deep research is a **Project**: queued, batched server work when Spot/API capacity is free — cheaper than fake realtime at best quality. Merchant submit stays available anytime. Bot, humans, and providers write the **same ticket timeline** (app ± email).

| Mode | Job |
|---|---|
| Realtime | Ticket intake — SEND frames + coach; do not promise a full solve |
| **Project (primary)** | Batch investigate until DIY, need-more-evidence, merchant-ready, or closed |
| Merchant | Optional handoff; updates stay on the Problem |

## Shared local knowledge base

Canonical: [`../../myproblem/docs/knowledge-base.md`](../../myproblem/docs/knowledge-base.md).

Investigations **mint and maintain public docs** for **Georgia and Israel only** — every article must tie to a **local geographic point** in-region. Private ticket data stays private; public advice (and attributed rumors from reliable / identified sources) feeds the shared layer.

**Compete locally, not globally:** outrun ChatGPT/Grok/Google/Yelp on *place-connected, realtime* knowledge — Relay vision — instead of world-scale corpus size.

**Channels (opt-in):** people may project coarse geo or **Wi-Fi locality** (never passwords) so tickets and helpers meet on house / street / town / network channels. More non-sensitive locality → more trust; street address and phone stay optional.

## Core loop

1. Person opens MyProblem (big actions, voice-first, ADA-capable) and lands on **Problems**.
2. **Realtime camera UX** on device; app keeps only clear/unique frames; server **rate-limits** accepts — **intake only**.
3. Ticket packaged → default **start Project** (deferred investigation) and/or submit to merchant.
4. AI may recommend **combine** or **split**; user confirms.
5. Problem-oriented AI only. Legal stops on every update. Timeline is the one-stop shop.
6. Often: Project → `ai_resolved` — never needed the business phone line.
7. When not: local business gets the **Problem tip** and can bid / accept / decline.
8. After handshake, the deal is between them.
9. Hard cases escalate inside the Project worker (frontier / other LLMs / web); answer written onto the Problem.
10. Public lessons → region KB update; optional channel post (place / Wi-Fi locality) without exposing secrets.

## Domains (start narrow, design for more)

| Phase | Domains | Note |
|---|---|---|
| Pilot | Home repair (plumbing / appliance), simple glasses/frame; optionally one clinic/vet-style triage partner | Photo + “should I call?” load |
| Later | Artists, caretakers, other local services | Same dossier pattern |
| Careful | Health / vet / mushrooms | Stop-and-redirect as product; AI lightens phone, does not practice medicine/vet medicine |

## Who pays (and who does not)

### Businesses — preferred: free forever

- **$0 application, $0 subscription, $0 per-lead bill** as the default product promise.
- ROI for them is **time saved** + **warmer bookings**, not a SaaS invoice.
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
- Business dashboard: incoming Problems, triage stats (“deflected from phone”), accept/decline — no invoice page by default.

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

Arizona Clevertree Solutions, LLC was terminated (2022). Form a **new Georgia LLC** under **MyProblem** naming, new EIN, bank, processor. Domain **myproblem.app** is registered and live. Do not revive the Arizona filing; do not reuse Clevertree as the consumer brand.
