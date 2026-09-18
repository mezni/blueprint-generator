# Session Handoff

Context for the next working session. Update this file at the end of each session, or delete it once the project no longer needs a running handoff.

## Snapshot

| Field | Value |
| --- | --- |
| Date | 2026-09-17 |
| Version | 0.1.7 |
| Phase | 7 — Agent Loop (in progress) |
| Next phase | 7 — finish the loop (execute action + state feedback); then 8 — Tools |
| Milestone | M2 (idea in → full structured blueprint out) reachable; M3 (multi-turn agent with planning) started |

## What is done

- `uv`-managed project with runtime deps (`pydantic`, `httpx`, `pyyaml`, `python-dotenv`) and dev deps (`pytest`, `pytest-asyncio`, `ruff`)
- `ConfigLoader` for `config/settings.yaml` + `.env`
- `PromptManager` for versioned YAML prompt templates (load, render, system prompt) — `prompts/project_name.yaml` (`v1`) and `prompts/project_blueprint.yaml` (`v1`, requests the full seven-section blueprint as JSON)
- `LLMClient` calling OpenRouter `/chat/completions` with `httpx`, robust error handling (timeouts, HTTP status, connection, malformed responses), plus a `reasoning`-field fallback for reasoning models that return `content: null`
- `LLMClient.structured_output()` — parses the LLM text response as JSON and validates it into a Pydantic model (`RuntimeError` on bad JSON or schema failure)
- `ProjectPlanner.generate_blueprint()` — owns the application logic (validates idea, builds messages, returns a validated `ProjectBlueprint`); `LLMClient` stays focused on provider communication
- `models.py` — `ProjectName`, `Technology`, `ProjectBlueprint` (all seven sections), `AgentAction` (`generate_blueprint` | `finish`)
- `PromptManager.render_user_prompt` accepts an optional `state` (default `""`)
- `ProjectPlannerAgent.decide()` — returns a validated `AgentAction`
- `AgentRunner.run()` — agent loop with max-iteration guard
- Verified end-to-end against the live API: full blueprint → `DocSearch AI Platform`; agent loop selects `generate_blueprint` and is caught by the `max_iterations` guard
- `pytest` and `ruff` both green

## Repo layout

```
blueprint-generator/
├── config/settings.yaml        # application + llm settings
├── prompts/
│   ├── project_name.yaml       # versioned naming prompt (v1, JSON output)
│   ├── project_blueprint.yaml  # versioned blueprint prompt (v1, JSON output)
│   └── agent.yaml              # versioned agent action-selection prompt (v1)
├── src/
│   ├── main.py                 # placeholder entry point
│   ├── config_loader.py        # ConfigLoader
│   ├── prompt_manager.py       # PromptManager
│   ├── llm_client.py           # LLMClient (chat + structured_output)
│   ├── models.py               # ProjectName, Technology, ProjectBlueprint, AgentAction
│   ├── project_planner.py      # ProjectPlanner.generate_blueprint()
│   ├── agent.py                # ProjectPlannerAgent.decide()
│   └── agent_runner.py         # AgentRunner.run() loop
├── tests/
│   ├── test_main.py
│   └── test_models.py          # ProjectName validation tests
├── docs/PRD.md, docs/ROADMAP.md
├── CHANGELOG.md, README.md
└── pyproject.toml, uv.lock, .env, .env.example, .gitignore
```

## How to verify

```bash
uv run pytest
uv run ruff check
```

There is no CLI yet; services are exercised via short scripts run under `src/` with `uv run python src/<script>.py` (see `src/` on `pythonpath` in `pyproject.toml`). Temporary verification scripts are deleted after use.

## Environment notes / gotchas

- `.env` holds a real `OPENROUTER_API_KEY` and is gitignored — never commit it.
- `config/settings.yaml` currently uses `model: openrouter/free`. Individual `:free` models were returning `429`; the free router auto-selects a live one. Revert to `openai/gpt-4o-mini` once the OpenRouter account has credits.
- `openrouter/free` occasionally routes to reasoning models that return `content: null` with the answer in `reasoning` — handled by the fallback in `LLMClient.chat()`.
- The `VIRTUAL_ENV=.../ccarf2/.venv` warning comes from an unrelated active venv in the shell; it is harmless (`uv` uses this project's `.venv`). Run `unset VIRTUAL_ENV` to silence it.

## Open items / next steps

1. Commit the pending changes so `0.1.7` is captured.
2. Finish Phase 7 — Agent Loop: execute the selected action inside the loop (call `ProjectPlanner.generate_blueprint()`) and write the outcome back into `state` so the agent can select `finish` and terminate. Watch the documented failure modes: infinite loops, runaway token spend, no termination criteria (currently the `max_iterations` guard catches it).
3. Retrofit automated tests for `ProjectPlanner`/`AgentRunner` with a mocked `LLMClient` (currently verified via live-API scripts).
4. Phase 8 — Tools: give the agent tools (web search, files), structured tool requests.

## Git state at handoff

- Latest commit: `Add LLM client`
- Uncommitted at handoff: `src/llm_client.py` (modified), `src/prompt_manager.py` (new), `src/models.py` (new), `src/project_planner.py` (new), `src/agent.py` (new), `src/agent_runner.py` (new), `src/test_agent.py` (untracked, temporary), `tests/test_models.py` (new), `prompts/project_name.yaml` (new), `prompts/project_blueprint.yaml` (new), `prompts/agent.yaml` (new), `CHANGELOG.md`, `README.md`, `docs/ROADMAP.md`, `docs/PRD.md`, `docs/SESSION_HANDOFF.md` (modified)