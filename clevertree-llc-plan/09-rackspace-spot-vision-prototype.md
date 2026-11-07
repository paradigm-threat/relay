# Rackspace Spot prototype — realtime video → Problem context

**Goal:** Before paying for always-on production GPUs, rent **Rackspace Spot** high-power boxes (especially GPU) and benchmark the realtime path: video/camera feed → usable frames → OCR / context → Problem patch, with optional frontier escalate-to-learn.

Org already on Spot: **paradigmthreat** (`org_wb51srHT8GccEYeV`).

---

## What Spot can give us (checked 2026-11-07)

| Class | Region | Spec (listed) | Spot ~$/hr | On-demand ~$/hr | Role |
|---|---|---|---|---|---|
| **gpu.vs2.xlargeplusplus-sjc** | `us-west-sjc-1` | **GPU A30**, 24 vCPU, 64 GB | **$0.15** | $7.25 | Primary vision bench (VL + OCR GPU) |
| ch.vs1.2xlarge-iad | `us-east-iad-1` | 16 vCPU, 32 GB | ~$0.01 | $0.26 | East ingest / ffmpeg / WebRTC SFU |
| gp.vs1.2xlarge-iad | `us-east-iad-1` | 16 vCPU, 64 GB | ~$0.01 | $0.30 | Parallel CPU OCR / brief compiler |
| mh.vs1.2xlarge-iad | `us-east-iad-1` | 16 vCPU, 128 GB | ~$0.02 | $0.43 | Heavy CPU models / batch distill |
| ch.vs1.2xlarge-lon | `uk-lon-1` | 16 vCPU, 32 GB | ~$0.01 | $0.26 | EU-ish latency probe (Israel users) |

**Note:** As of this check, **A30 GPU Spot inventory showed in San Jose only** among listed regions. IAD/DFW/LON are strong for **CPU frame pipeline** benchmarks; put the heavy VL on SJC GPU Spot (or reclaim when market moves).

Spot is interruptible — fine for benchmarks; checkpoint results to NAS / Relay often.

---

## Prototype architecture (bench, not product)

```
Laptop / phone camera
    │  WebRTC or RTSP / file replay (recorded leak/glasses clips)
    ▼
[Region A] Spot CPU  — ingest, keyframe extract (ffmpeg), quality gate
    │  selected JPEGs + timestamps
    ▼
[SJC] Spot GPU A30 — OCR (Paddle/Rapid), fast VL (Florence-2 / similar), structured JSON
    │
    ├─ write Problem patch locally + push to Relay tip (optional)
    └─ on low confidence → OpenAI/Anthropic/Bedrock frontier (log gold)
```

Two-region **bench** (matches later prod story):

1. **iad** (or dfw): stream terminate + frame factory CPU  
2. **sjc**: GPU recognition  
3. Optional **lon**: same CPU pipeline for EU/IL RTT numbers  

---

## Benchmarks to run (pass/fail metrics)

Record every run in `clevertree-llc-plan/bench/` (or NAS sibling) as CSV + short markdown.

### B0 — On-device gate + server rate limit (product feature)

Simulate app logic on the bench host or a phone build:

| Metric | Target |
|---|---|
| Clarity filter drops blurry frames | Manual spot-check: junk rarely uploaded |
| Uniqueness dedupe | Near-duplicate burst → ≤ 1 upload |
| Server rate limit | Excess frames get 429; Problem still updates from accepted set |
| User-perceived realtime | Live preview uninterrupted while uploads stay sparse |

### B1 — Frame factory throughput (CPU Spot)

| Metric | Target (pilot-grade) |
|---|---|
| 720p / 1080p ingest → keyframes/sec | ≥ 5 kept decisions/sec sustained |
| Motion/blur reject accuracy (manual spot-check) | Obvious junk &lt; 10% false keep |
| End-to-end frame→disk JPEG | p95 &lt; 100 ms after decode |

### B2 — Realtime context on GPU A30

| Metric | Target |
|---|---|
| OCR on model-number / serial crops | p95 &lt; 300 ms |
| Cheap VL structured patch (what/uncertainty/next shot) | p95 &lt; 1.5 s |
| Concurrent streams on one A30 | Find knee (1 / 2 / 4) before p95 doubles |
| GPU mem / watt not required; **$/hour × latency** is |

### B3 — Video-feed → Problem brief (full path)

| Metric | Target |
|---|---|
| Open Problem + 30s guided scan → first useful patch | &lt; 5 s to first patch; continuous updates ≤ 2 s apart |
| Kept frames / 30s scan | ≤ 15 after dedupe (prove we are not shipping every frame) |
| Frontier escalate rate on fixture set | Log %; aim understandability, not &lt;X% yet |

### B4 — Frontier learn loop (small)

| Metric | Target |
|---|---|
| N hard frames with cheap vs Claude/GPT | ≥ 50 labeled |
| Agreement rate / error taxonomy | Written; feeds router rules |

### B5 — Cost extrapolation

From Spot hours + API $:

- Cost per 30s scan  
- Cost per kept frame  
- Projected monthly at 2k / 20k sessions (update `08-realtime-vision-infra-cost.md`)

---

## Suggested Spot shapes for a 3–5 day sprint

| Day | Nodes | Bid strategy | Est. $ |
|---|---|---|---|
| 1–2 | 1× `ch.vs1.2xlarge-iad` + 1× `gpu.vs2.xlargeplusplus-sjc` | Spot at/above market ($0.01 + $0.15) | ~$4–8 if ~24h on; or **$2–4** for 8h/day |
| 3 | Add `ch.vs1.2xlarge-lon` for RTT | Same | +~$1–2 |
| 4–5 | Re-run best config; frontier batch | GPU only when needed | API $20–100 |

**Sprint budget cap:** **$50–150** Spot + **$50–150** frontier APIs unless extended.

On-demand GPU ($7.25/hr) only if Spot reclaim blocks the sprint — avoid for multi-day always-on.

---

## Work items (Cursor build on Spot)

1. VM Cloudspace or K8s Cloudspace in org; SSH key; pull a thin bench repo  
2. Docker: `ffmpeg`, Python, CUDA stack on A30 image  
3. Script: `ingest_webrtc_or_file.py` → keyframes → queue  
4. Script: `recognize_gpu.py` → OCR + VL → `problem_patch.json`  
5. Script: `bench_report.py` → CSV + p50/p95  
6. Optional: write patches into a Relay Problem tip for end-to-end demo  
7. Tear down Spot pools when idle (market meter never sleeps if you leave them)

---

## Success → next plan step

| Outcome | Action |
|---|---|
| A30 meets B2/B3 targets | Keep Spot GPU as **burst prototype**; production still 2-region plan in `08` |
| CPU-only almost enough for OCR+keyframes | Shrink GPU role; more on-device + API VL |
| Latency SJC↔GA too painful for live coach | Prefer IAD CPU + API VL for GA; GPU Spot for offline distill / batch |
| Spot reclaim too frequent | Short on-demand windows or different GPU region when listed |

---

## Production note (after prototype)

Keep Spot as the **free plan** vision pool: when Spot is reclaimed or at capacity, the app says free scanning is unavailable due to demand (camera preview may continue; uploads wait). Route **paid** accounts to reliable workers so revenue users are not on interruptible iron.

Prototype should still exercise a fake `free_pool` vs `paid_pool` flag so the unavailable path is tested early.

## Explicit non-goals for this prototype

- Production SLA / multi-AZ  
- Consumer app polish  
- Unlimited free campaign traffic  
- Training a full LoRA (optional stretch after B4)

---

## Decision log

| Date | Note |
|---|---|
| 2026-11-07 | Add Spot prototype phase; SJC A30 GPU ~$0.15/hr spot; IAD/LON CPU for ingest/RTT |
