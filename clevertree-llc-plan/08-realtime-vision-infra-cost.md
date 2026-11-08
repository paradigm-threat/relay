# Rapid image recognition — infra + 2-region $$$

**Goal:** Accept camera / near-video bandwidth, turn it into usable frames + OCR + context in near-realtime, with accuracy approaching today’s OpenAI / Anthropic vision — and where we cannot match them live, call them and **learn**.

**Honesty first:** Running frontier-class multimodal reasoning on **every** frame (or as an on-demand product LLM) is not economically “unlimited free.” MyProblem stance:

1. **Aggressive frame selection** (most bandwidth never becomes a model call)  
2. **Smart-enough realtime** — capture questions / requirements / photos only  
3. **Queue JIRA-like ticket by status** → **batch investigate** ASAP + completely (**region KB git + cache first**)  
4. **Frontier escalate** (Bedrock / Anthropic / OpenAI) **rare**, after KB miss, inside the **Project batch** worker — not live  
5. **Write useful research + escalate residue into the KB** (git Markdown); optional distill for **function/vision** — knowledge compounds in KB, not by stuffing the server model ([`../../myproblem/docs/knowledge-base.md`](../../myproblem/docs/knowledge-base.md))

Canonical product docs: [`../../myproblem/docs/projects.md`](../../myproblem/docs/projects.md) · [`../../myproblem/docs/backbone-escalate.md`](../../myproblem/docs/backbone-escalate.md) · [`../../myproblem/docs/knowledge-base.md`](../../myproblem/docs/knowledge-base.md).

---

## Pipeline (what actually runs)

```
Phone camera (live preview stays on device)
  │  app scores clarity + uniqueness; client token bucket
  │  only clear/unique frames upload (not raw 30 fps)
  ▼
Regional ingest (US + IL/EU)
  │  auth + **server rate limit** (reject/slow extras)
  │  attach to Problem ID — capture coach patches ticket fields
  ▼
Ticket packaged → Project status queue (JIRA-like)
  ▼
Batch investigator (asap + complete, non-redundant)
  │  region KB git + cache (miss → continue)
  │  allowed browser / maps research (honest walls)
  │  HF / open VL+OCR on Spot (+ optional image/diagram gen)
  │  mid LLM / tools → commit public lessons to KB
  │  structured Problem patch + InvestigationUpdate + media Evidence
  │
  ├─ confidence high → done (stay cheap)
  └─ still stuck (math / lang / hard VL / gen / legal gold)
        ▼
     Backbone escalate (Bedrock Sonnet-class) — rare
        ▼
     Write gold onto Problem + answer/media cache + **KB Markdown**
        ▼
     Optional prompt/router tweak for vision/function only
```

**Photo analysis & generation:** prefer **Hugging Face weights on Spot** (or HF/cheap batch Inference) inside Projects — not on-demand Midjourney/Claude-vision UX. Same economics as text escalate: unique miss → cache.

### Feature: on-device frame gate + server rate limit

| Layer | Job |
|---|---|
| App | Live video UX; blur/exposure/motion scores; perceptual dedupe; local upload budget |
| Server | Hard rate limit per user/Problem; fair queue; never accept unbounded streams |
| Together | Feels realtime; costs like sparse photos |

Default policy to bench (tune in Spot Phase 0.5): **2–4 accepts/sec** burst, **≤ ~15 frames / 30s** active scan, daily soft cap for free accounts.

### What “realtime” means here

| Path | Latency target | Content |
|---|---|---|
| Local coaching | &lt; 50–100 ms | “hold still / too blurry” from on-device scores |
| Preview / “point here” | &lt; 200–500 ms | Keep/reject feedback; ROI hints |
| OCR + labels | &lt; 1–2 s | Model numbers, serials, signs |
| Advice patch | &lt; 2–5 s | Cheap VL structured fields |
| Frontier escalate (in Project batch) | minutes→hours OK | Hard unique misses only; not live UX |

Do **not** ship continuous 1080p30 to the cloud. Ship **selected frames** plus metadata.

---

## Accuracy strategy (match OpenAI/Anthropic where it matters)

| Layer | Accuracy role | Learn from frontier? |
|---|---|---|
| Frame picker | Keep sharp, on-subject frames | Yes — frontier says “bad frame / need valve close-up” → train rejector |
| OCR | Text in image | Compare local OCR vs frontier reading on escalate set |
| Cheap VL | Everyday ID / triage | Distill disagreements |
| Frontier | Gold label + user-facing hard answers | Source of truth for training queue |

**Limits of self-host:** tiny engraved text, weird lighting, medical subtlety, rare mushrooms — keep escalate rate honest (pilot **5–15%** of sessions; optimize toward **1–5%**).

**Learning loop (minimal):** store `(problem_brief, frame, cheap_output, frontier_output, user_outcome)` → weekly router/prompt update → later LoRA on open VL if volume justifies GPU training time.

---

## Two regions

| Region | Serves | Why |
|---|---|---|
| **A — US East** (e.g. Virginia / existing Relay US) | Georgia pilot | Low latency to GA; near current fleet |
| **B — EU or Israel-adjacent** (Frankfurt / Tel Aviv cloud) | Israel pilot | Latency + data locality; brother ops |

Each region: ingest + frame factory + brief compiler. Frontier APIs can be global; **training queue** central or dual-write to US.

Relay still holds the **Problem tip**; vision workers write evidence + patches into that Problem.

---

## Unit costs (approx, late 2026 list prices)

Frontier (order of magnitude per **kept** frame + short prompt):

| Call | Ballpark |
|---|---|
| OpenAI GPT-4o-class high-detail ~1K² | ~$0.002 / image input (+ output text) |
| Claude Sonnet-class ~1K² | ~$0.002–0.005 / image |
| Cheap API VL (Nova Lite / Flash / Haiku-class) | ~$0.0001–0.001 / image |
| Local OCR on GPU worker | ~$0 at marginal (in GPU rent) |
| AWS Textract Detect Text | ~$0.0015 / page (use sparingly) |

**Bandwidth:** 1 selected JPEG ~100–300 KB.  
1,000 kept frames/day ≈ 0.1–0.3 GB/day ingest — cheap.  
Raw 1080p30 WebRTC is the expensive mistake (tens of GB/hour/user).

**GPU (frame factory), on-demand-ish:**

| Box | Ballpark |
|---|---|
| 1× L4 / A10G-class (g5.xlarge ~$1/hr AWS; L4 often $0.4–0.8/hr elsewhere) | ~$300–750 / mo always-on |
| Spot / cheaper market | ~40–60% less with interrupt handling |

---

## Capacity scenarios → monthly $$$ (2 regions)

Assumptions per scenario:

- On-device or edge keeps **2 useful frames/sec** max while user is actively scanning, average **30 scan-seconds / session** → ~60 candidate frames → **~8 kept frames** after dedupe/quality  
- Cheap VL on all kept frames  
- Frontier on **10%** of sessions (1–2 frames) early; **3%** when mature  
- 2 regions each run **1 always-on GPU worker** + small CPU ingest (or 1 GPU handles both if tiny — prefer 1/region for latency)  
- Relay / existing mesh **not** re-costed here (you already pay fleet); add only vision-specific

### Fixed (2 regions) — always on

| Item | Low | High |
|---|---|---|
| 2× GPU workers (L4/A10G class) | $500 | $1,500 |
| 2× small ingest/API VMs | $40 | $120 |
| Object temp storage / CDN egress | $20 | $100 |
| Observability | $20 | $50 |
| **Fixed subtotal** | **~$580** | **~$1,770** |

### Variable — by monthly active scan volume

| Scale | Sessions / mo | Kept frames | Cheap VL $ | Frontier $ (10%→3%) | Bandwidth $ | **Variable ~** |
|---|---|---|---|---|---|---|
| **Pilot** | 2,000 | ~16k | $5–40 | $40–150 | &lt;$10 | **$50–200** |
| **Early growth** | 20,000 | ~160k | $50–400 | $200–800 | $20–50 | **$300–1,200** |
| **Serious free campaign** | 200,000 | ~1.6M | $500–4,000 | $500–3,000* | $100–400 | **$1.5k–7k** |

\*Frontier drops if escalate rate falls to ~3% and images stay downscaled.

### All-in monthly (2 regions)

| Scale | **Monthly burn (vision stack)** |
|---|---|
| Pilot (GA + Israel) | **~$700 – $2,000** |
| Early growth | **~$1,000 – $3,000** |
| Aggressive free | **~$2,500 – $9,000** (spikes if escalate stays high or people stream video) |

### One-time / setup

| Item | $ |
|---|---|
| Wiring WebRTC or upload path, frame factory, Problem writer | Mostly Cursor time ($0 salary) |
| Buffer GPU credits / API prepaid | **$500 – $2,000** |
| If you skip self-GPU and go **100% API** for pilot | Fixed drops to ~$100–300; variable rises (see below) |

### API-only pilot alternative (no regional GPU)

Skip self-host GPU; every kept frame → cheap API VL; 10% → frontier.

| Scale | Monthly |
|---|---|
| 2k sessions | **~$100 – $400** (+ tiny ingest) |
| 20k | **~$800 – $3,000** |
| 200k | **~$5k – $20k+** (why you add regional GPU later) |

**Recommendation:** start **API-only + on-device keyframes** for weeks 1–4 (~**$150–500/mo**), then stand up **2× GPU regions** when kept frames/day make self-host cheaper than APIs or latency demands it.

---

## What would blow the budget

1. Uploading continuous video instead of keyframes  
2. Sending every frame to Claude/GPT at high detail  
3. Always-on H100s “for accuracy”  
4. Textract on every frame  
5. Image-gen explain cards on by default  

## What keeps accuracy high without H100s

1. Better **capture coaching** (UI: “move closer”) — free accuracy  
2. Multi-frame consensus on device (3 stills → 1 brief)  
3. Frontier escalate + distillation  
4. Domain adapters per vertical (plumbing labels ≠ rash)  
5. Human provider outcomes as weak labels when jobs settle  

---

## Suggested buy order (cash)

| Week | Spend |
|---|---|
| **0.5** | **Rackspace Spot prototype** (see `09`): IAD CPU + SJC **A30 GPU** Spot (~$0.15/hr) for realtime feed benches — sprint **~$50–150** + API **~$50–150** |
| 0 | $0–100: on-device picker + OpenAI/Anthropic/Bedrock keys, log escalate diffs |
| 1–4 | **$150–500/mo** API-only, 2 regions = just API geo + Relay tips |
| 5+ | Add **~$600–1,500/mo** for 2× L4/A10G frame factories when volume or latency hurts |
| Campaign | Cap frontier escalate %; budget **$2.5k–9k/mo** vision all-in at high free volume |

Spot GPU hours are for **proving** the realtime frame→OCR→VL path **and**, in production, the **free-tier pool** (interruptible; show “busy” when demand/reclaim hits). **Paid** traffic should sit on reliable capacity (on-demand / reserved / local colo) so monetized users are not competing with Spot reclaim.

| Tier | Infra $ shape |
|---|---|
| Free | Spot market — scales with free usage; fails soft when market tight |
| Paid | Steady reliable burn — smaller user count, higher willingness to pay |

---

## Tie-back to product

- Evidence + OCR + VL patches land on the **Problem**, not a chat log.  
- Realtime UX is “Problem updating,” not “streaming tokens of prose.”  
- Providers still open the Relay Problem tip; they never need the video pipe.
