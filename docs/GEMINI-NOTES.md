# Gemini notes from the organizer guide (Oct 25)

Complements docs/PLAN.md section 6. Interactions API only.

## Models and reasoning

- `gemini-4-argon` (lead) and `gemini-3.8-flash` (leaf modules, baseline comparison).
- `thinking_level`: `"low"` | `"medium"` (default) | `"high"`, set in `generation_config`.
- Our choice: lead planner uses `high`. Repair loops use `medium` for speed in the live demo.
- SDK: `google-genai` 2.33.0 is locked (the skill now asks for >= 2.30.1).

## Long output and state

- Default output cap is 65,536 tokens. To go past it: `background=True` plus `generation_config={"max_output_tokens": N}` with N > 65536.
- Poll `client.interactions.get(id=...)` while `status` is `"in_progress"` or `"queued"`.
- `previous_interaction_id` gives server-side conversation state.
- Interactions are stored by default. `store=False` disables both `background` and `previous_interaction_id`, so leave it on.
- `tools`, `system_instruction` and `generation_config` are per interaction: re-send them every turn.
- HTTP timeouts 15 to 20 minutes for `high`, or use `background=True`.

## Cost

- Implicit caching is automatic: 95% discount on cached input tokens. Do not call `cached_content`.
- $2 / 1M input, $10 / 1M output through Nov 30, 2026.
- Usage fields: `interaction.usage.total_input_tokens`, `total_output_tokens`, `total_thought_tokens`, `total_cached_tokens`.

## Remove legacy params

`thinking_budget`, `temperature`, `top_p`, `top_k`, `cached_content`.

## Managed agents

- `agent="antigravity-preview-09-2026"`, `agent_config={"type": "antigravity", "model": "gemini-4-argon"}`.
- `environment="remote"` works as a string. Use the dict form `{"type": "remote", "network": {"allowlist": [...]}}` when we need an allowlist (PR step).
- Secrets go through `client.credentials.create`; they are injected at the proxy and never enter the model context.

## Pitch and demo hooks

- Google's own pattern 2 (give Argon a verification loop) and pattern 4 (Argon lead plus Flash leaf subagents) are exactly what Theseus does. Say so.
- Before the demo, ask Argon for an interactive HTML architecture diagram of the pipeline.

Key check: `cd backend && uv run python ../scripts/gemini_check.py`
