# Session Handoff

Context for the next working session. Update this file at the end of each session, or delete it once the project no longer needs a running handoff.

## Snapshot

| Field | Value |
| --- | --- |
| Date | 2026-09-17 |
| Version | 0.1.5 |
| Phase | 5 — Structured Output (complete) |
| Next phase | 6 — Project Planner (full blueprint) |
| Milestone | M1 (idea in → project name out) — reachable as a service, not yet covered by an automated test |

## What is done

- `uv`-managed project with runtime deps (`pydantic`, `httpx`, `pyyaml`, `python-dotenv`) and dev deps (`pytest`, `pytest-asyncio`, `ruff`)
- Configuration layer: `config/settings.yaml` + `.env`, loaded via `ConfigLoader`
- `PromptManager` for versioned YAML prompt templates (load, render, system prompt) — `prompts/project_name.yaml` (`v1`, asks the model for JSON)
- `LLMClient` calling OpenRouter `/chat/completions` with `httpx`, robust error handling (timeouts, HTTP status, connection, malformed responses), plus a `reasoning`-field fallback for reasoning models that return `content: null`
- `LLMClient.structured_output()` — parses the LLM text response as JSON and validates it into a Pydantic model (`RuntimeError` on bad JSON or schema failure)
- `ProjectPlanner` service — owns the application logic (validates idea, builds messages, returns a validated `ProjectName`); `LLMClient` stays focused on provider communication
- Verified end-to-end against the live API: idea → `DocuSearch AI`
- `pytest` and `ruff` both green

## Repo layout

```
blueprint-generator/
├── config/settings.yaml        # application + llm settings
├── prompts/
│   └── project_name.yaml       # versioned naming prompt (v1, JSON output)
├── src/
│   ├── main.py                 # placeholder entry point
│   ├── config_loader.py        # ConfigLoader
│   ├── prompt_manager.py       # PromptManager
│   ├── llm_client.py           # LLMClient (chat + structured_output)
│   ├── models.py               # ProjectName Pydantic model
│   └── project_planner.py      # ProjectPlanner (application logic)
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

1. Commit the pending changes so `0.1.5` is captured.
2. Phase 6 — Project Planner: expand the single naming service into `generate_blueprint()` producing all seven blueprint sections (name, description, business problem, functional requirements, non-functional requirements, technology stack, implementation plan). Add the corresponding `ProjectBlueprint` model and prompt template.
3. Formalize the M1 milestone (idea → project name) as an automated test with a mocked `LLMClient` rather than a manual script.

## Git state at handoff

- Latest commit: `Add LLM client`
- Uncommitted at handoff: `src/llm_client.py` (modified), `src/prompt_manager.py` (new), `src/models.py` (new), `src/project_planner.py` (new), `src/test_planner.py` (untracked), `tests/test_models.py` (new), `prompts/project_name.yaml` (new), `CHANGELOG.md`, `docs/SESSION_HANDOFF.md` (modified)