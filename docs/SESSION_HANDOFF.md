# Session Handoff

Context for the next working session. Update this file at the end of each session, or delete it once the project no longer needs a running handoff.

## Snapshot

| Field | Value |
| --- | --- |
| Date | 2026-09-18 |
| Version | 0.1.8 |
| Phase | 8 — Tools (in progress) |
| Next phase | 8 — finish tools (tool-aware prompt wiring + more tools); then 9 — Memory |
| Milestone | M3 (multi-turn agent with planning) done; M4 (agent with tools) started |

## What is done

- Phase 7 — Agent Loop is **complete**: the loop selects an action, executes it, and feeds the outcome back into `state` so the agent can observe it and select `finish`; the `max_iterations` guard is no longer the only termination path.
- Phase 8 — Tools is **started**:
  - `src/tools.py` — `Tool` (name, description, `input_model`, callable `function`), `ToolRegistry` (`register`/`get`/`execute`), and `create_blueprint_tool(planner)`
  - Tool arguments are schema-validated: `Tool.execute(arguments)` validates the raw dict with Pydantic before calling the function
  - `GenerateBlueprintInput` model (`project_idea`, 10–5000 chars) governs the blueprint tool's arguments
- `AgentAction` now carries `arguments: dict[str, object]` (default `{}`), so the LLM supplies structured tool arguments
- `prompts/agent.yaml` bumped to **v2**: the system prompt documents each action's required argument contract and the full JSON shape including `arguments`
- `AgentRunner.run` executes `self.tools.execute(action.action, action.arguments)` — no hard-coded arguments
- `LLMClient._extract_json()` — strips Markdown code fences and falls back to the first `{`…last `}` span, so reasoning-model responses that wrap JSON in prose still parse (fixes the `json.loads` "Expecting value: line 1 column 1" crash seen during a live run)
- `src/test_tools.py` (temporary) — verifies `GenerateBlueprintInput` accepts a long idea and rejects a too-short one
- Phase 6 planner + Phase 7 agent still verified end-to-end against the live API (blueprint → `DocSearch AI Platform` / `Internal TechDoc AI Search Platform` seeds)
- `pytest` and `ruff` both green

## Repo layout

```
blueprint-generator/
├── config/settings.yaml        # application + llm settings
├── prompts/
│   ├── project_name.yaml       # versioned naming prompt (v1, JSON output)
│   ├── project_blueprint.yaml  # versioned blueprint prompt (v1, JSON output)
│   └── agent.yaml              # versioned agent action-selection prompt (v2, includes arguments contract)
├── src/
│   ├── main.py                 # placeholder entry point
│   ├── config_loader.py        # ConfigLoader
│   ├── prompt_manager.py       # PromptManager
│   ├── llm_client.py           # LLMClient (chat + structured_output + _extract_json)
│   ├── models.py               # ProjectName, Technology, ProjectBlueprint, GenerateBlueprintInput, AgentAction
│   ├── project_planner.py      # ProjectPlanner.generate_blueprint()
│   ├── agent.py                # ProjectPlannerAgent.decide()
│   ├── agent_runner.py         # AgentRunner.run() loop (executes tools, feeds state)
│   ├── tools.py                # Tool, ToolRegistry, create_blueprint_tool
│   ├── test_agent.py           # (temporary) live end-to-end agent + tools script
│   └── test_tools.py           # (temporary) GenerateBlueprintInput validation script
├── tests/
│   ├── test_main.py
│   └── test_models.py          # ProjectName validation tests
├── docs/PRD.md, docs/ROADMAP.md, docs/SESSION_HANDOFF.md
├── CHANGELOG.md, README.md
└── pyproject.toml, uv.lock, .env, .env.example, .gitignore
```

## How to verify

```bash
uv run pytest
uv run ruff check
uv run python src/test_tools.py   # local: GenerateBlueprintInput validation
uv run python src/test_agent.py   # live API: full agent loop with tool execution
```

There is no CLI yet; services are exercised via scripts run from inside `src/` (`pythonpath = ["src"]`). Temporary verification scripts are deleted after use.

## Environment notes / gotchas

- `.env` holds a real `OPENROUTER_API_KEY` and is gitignored — never commit it.
- `config/settings.yaml` currently uses `model: openrouter/free`. Individual `:free` models were returning `429`; the free router auto-selects a live one. Revert to `openai/gpt-4o-mini` once the OpenRouter account has credits.
- `openrouter/free` occasionally routes to reasoning models that return `content: null` with the answer in `reasoning`; the fallback in `LLMClient.chat()` covers that, and `_extract_json` handles reasoning prose that wraps the JSON.
- The `VIRTUAL_ENV=.../ccarf2/.venv` warning comes from an unrelated active venv in the shell; it is harmless (`uv` uses this project's `.venv`). Run `unset VIRTUAL_ENV` to silence it.
- The agent sometimes re-selects `generate_blueprint` instead of `finish` after the state is set — that is the LLM's choice, not a crash; `max_iterations` still catches runaway loops.

## Open items / next steps

1. Verify the live flow with the new `arguments` contract (`src/test_agent.py`): the agent should emit `arguments: {"project_idea": ...}` and the tool should consume them.
2. Finish Phase 8 — Tools: wire the registered tool list into the agent prompt so the LLM only sees available tools + signatures, handle bad tool args / missing required arguments, and add failure-mode handling (tools returning junk, agent fabricating results).
3. Retrofit automated tests for `ProjectPlanner`/`AgentRunner`/`ToolRegistry` with a mocked `LLMClient`; add real unit tests for `Tool` argument validation.
4. Update the PRD/README evolution ladder: the agent is now at the `Tool-using Agent` (L4) rung.
5. Phase 9 — Memory after tools are solid.

## Git state at handoff

- Latest commit: `6d6667b Add tools schemas`
- Uncommitted at handoff: `prompts/agent.yaml` (v2 arguments contract), `src/models.py` (`AgentAction.arguments`), `src/agent_runner.py` (execute via `action.action` + `action.arguments`), plus this docs update. `src/test_agent.py` / `src/test_tools.py` remain temporary/untracked.