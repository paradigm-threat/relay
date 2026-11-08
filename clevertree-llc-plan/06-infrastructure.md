# Infrastructure options

Build and deploy ourselves (Cursor). Prefer **Relay** for problem distribution; pay commercial AI backbones for vision/LLM; keep the product thin.

## Context model: Problem is the unit

We do **not** design infra around chat history or max context windows as the product boundary.

| Store | Role |
|---|---|
| **Problem** | Source of truth: status, title, domain, legal stops, structured findings, links to evidence, provider bids |
| **Evidence** | Photos, clips, OCR text, voice transcripts attached to a Problem (or moved on split/combine) |
| **Event log** | Append-only attempts (“asked for valve photo”, “escalated to B”) — for audit/learn, **not** fed wholesale to the model |
| **Compiled brief** | Deterministic or lightly summarized view of the Problem shipped to the backbone each turn |

**Combine:** merge evidence + findings into one Problem; close or redirect the other IDs.  
**Split:** fork evidence subsets into a new Problem; keep cross-links (“related to PROB-123”).

Token budget is an implementation detail of “how we compile the brief,” not the user’s mental model.

## Layers

```
[ Phone / web — Problem list / detail, ADA, voice, camera ]
        │
[ Edge preprocess — resize, OCR, blur, redact → Evidence ]
        │
[ Problem store on Relay — one tip (or repo) per Problem ]
        │
[ Brief compiler — Problem + selected Evidence → model input ]
        │
[ Orchestrator — policies, combine/split suggestions, stops ]
        │
[ Backbone A — cheap/fast vision + structured update ]
[ Backbone B — escalate; write result onto Problem ]
        │
[ Match / notify — providers open Problem tip ]
[ Payments — provider settlement only ]
```

## Option A — Relay-first (preferred)

| Piece | Choice |
|---|---|
| Problem file | One Relay tip/repo per Problem (`ticket_uuid`); tip = dossier SoT providers open |
| Combine/split | Git-friendly: move evidence paths, write `related:` links, close superseded Problem |
| Hosting UI | Park / SSR on relay-gateway or thin client-web app talking to gateway |
| Auth | **Anon by default** (UUID + device claim / APK local backup); optional **free** secret recover email or free register; phone OTP when needed — see myproblem `docs/anonymous-tickets.md` |
| Media | Audio + stills in **cheap object store** (Bunny prefer; S3-compatible OK); tip holds URI + hash only — **no video files**; do not pin multi-MB audio on every node |
| Queue | Thin index (Neon/Redis) for Project / ASR job leases — not a second dossier |
| Chat/voice | Ephemeral input → Evidence/event; **not** a parallel source of truth; first ASR + LLM retranscription gate |
| Billing | **Outside** Relay (Stripe etc.) — Relay has no card billing today; recover/register stay $0 |

**Pros:** Matches vision; portable dossier; context = Problem; we already operate fleet.  
**Cons:** We own uptime; must design photo size and brief-compiler carefully.

## Option B — Managed cloud app + Relay dossier export

| Piece | Choice |
|---|---|
| API / agent | Vercel or similar for orchestrator only |
| Cases | Still write through to Relay tip for provider open  
| AI | Bedrock / OpenAI / Gemini API |

**Pros:** Faster iterate on HTTP APIs.  
**Cons:** Two planes; easier to accidentally become “normal SaaS.”

## Option C — Fully managed (not preferred)

Firebase/Supabase + only commercial AI. Easiest hire-ware path; fights the no-salary / Relay vision. Use only as a spike, not the investor architecture.

---

## AI backbones

| Role | Candidates | Notes |
|---|---|---|
| Default vision + short advice | Amazon Bedrock (multi-model), Gemini API, OpenAI | Pick on $/quality for **problem** prompts, not arena chat scores |
| Escalate | Stronger / larger model when confidence low | Log outcome into case; distill later |
| Image generation | Separate endpoint; explain-only prompts | Hard budget; not the free unlimited surface |
| Speech | Device STT/TTS first; cloud STT if needed | Prefer on-device for cost + privacy |

**Product rule:** system prompts and tools are **service investigation only**. Refuse general LLM entertainment features so “unlimited” stays affordable. Model output must be a **structured Problem patch** (fields + optional user-facing summary), not an unbounded chat reply as state.

## Image pipeline (hard lift before the LLM)

### Realtime video without uploading video

The user sees a **live camera**. The network sees **sparse Evidence frames**.

**On device (required lift):**

1. Run the camera preview locally (full fps for UX).  
2. Score frames: sharpness / blur, exposure, subject motion, optional face/plate blur.  
3. **Uniqueness:** perceptual hash / embedding distance vs last N sent — skip near-duplicates.  
4. **Clarity gate:** only enqueue if score ≥ threshold (or user taps “capture anyway”).  
5. Downscale / JPEG / WebP; attach Problem ID + timestamp + scores.  
6. Respect **client token bucket** mirroring server policy (don’t even try to upload over budget).  
7. Show UI coaching from local scores (“hold still”, “move closer”) *before* a server round-trip when possible.

**On server (rate limit + process):**

1. Per-account / per-Problem **rate limit** (examples to bench): e.g. max **2–4 accepts/sec** burst, **~8–15 kept frames / 30s scan**, soft daily caps for free tier.  
2. Over limit → `429` + `Retry-After` / “preview only”; never queue unbounded video.  
3. Accepted frames → OCR / cheap VL → **Problem patch** (realtime updates to the ticket).  
4. Low confidence → frontier escalate (still one frame, not a stream).  

Realtime means **Problem fields update while you point the camera**, not that 30 fps hits the model.

### Steps (summary)

1. Live preview on device  
2. Clarity + uniqueness filter → sparse uploads  
3. Server rate limit + quality re-check  
4. OCR / VL → structured Problem patch  
5. UI checklist updates; combine/split suggestions as today  

This is how we claim better “real-time scanning” than “3 uploads/day to a chatbot,” and why context length and bandwidth stop dominating product design.

## Provider open path

- Link or app opens **Problem tip** on Relay (read-only dossier)  
- Bid / decline recorded on the Problem + notified to consumer  
- After accept, contact channel can be phone/SMS/in-app — platform steps back from the job itself  

## Payments

- Stripe Connect (or similar) for provider charges  
- Automatic split: processing fee → reserve → LLC residual → distributions / per-case bounties  
- No consumer card for free investigation path  

## Privacy / retention

- Case photos are sensitive (homes, bodies, glasses).  
- Retention policy in terms: delete or lock after N days unless both parties keep.  
- Health-adjacent: minimize; prefer redirect over storing clinical chat.  
- Israel/EU users: expect stricter rules — consult before scale.

## Capacity pools: free → Spot, paid → reliable

```
                    ┌─ free_pool ─── Spot GPU/CPU (bid market, reclaim OK)
ingest + rate limit ┤
                    └─ paid_pool ─── on-demand / reserved / local colo (sticky)
```

| Rule | Behavior |
|---|---|
| Route by account tier | Free jobs only schedule on `free_pool`; paid on `paid_pool` |
| Spot reclaim / no capacity | Free: return clear **capacity unavailable** (not a silent hang). Paid: unaffected if reliable pool healthy |
| Overflow | Never steal paid capacity for free by default; optional “donate spare paid headroom to free” is a flag, off in scarce times |
| API escalate | Can stay shared with stricter free quotas; or paid-only for frontier |

UI copy (product, not ops jargon): “Free scanners are busy right now. Try again in a few minutes, or unlock priority scanning.”

## Cost control switches

| Switch | Effect |
|---|---|
| Daily scan budget per account | Soft cap after free depth; still no card — queue or “come back tomorrow” beats surprise bill |
| Free → Spot only | Keeps free burn tied to cheap interruptible $/hr |
| Paid → reliable pool | Predictable latency; justifies subscription or provider fee |
| Escalate rate limit | Most cases stay on cheap model |
| Explain-image gen off by default | On only when user taps “show me” |
| Provider free-tool quota | Mirrors consumer free uses |

## Build inventory (Relay / Cursor)

Reuse where possible:

- relay-gateway park/SSR and static case presentation  
- relay-client-web patterns for accessible UI  
- Existing catalog / mesh publish for app shells  
- Do **not** block pilot on full Android CleverScript parity — ship web or thin native shell first if faster  

## Pilot minimum viable infra

1. Web app (PWA) with camera + voice  
2. One Bedrock (or equivalent) path + escalate  
3. Case storage on Relay or Relay-writable object store with tip link  
4. Phone verify  
5. Manual provider onboarding spreadsheet → notify link (automate after first settled jobs)  
6. Stripe test mode until first real fee  
