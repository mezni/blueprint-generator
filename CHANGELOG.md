# Changelog

All notable changes to this project are documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/).

## Version History

| Version | Feature Domain | Key Objectives |
| --- | --- | --- |
| 0.1.5 | Project Planner | Add `ProjectPlanner` as the application-logic layer that wires `PromptManager` and `LLMClient` together, keeping the LLM client focused solely on provider communication. Milestone M1 (idea in → project name out) is now a real, reusable service |
| 0.1.4 | Structured Output | Add a `ProjectName` Pydantic model, request JSON from the LLM in the naming prompt, add `LLMClient.structured_output()` that parses and validates responses into the model, and unit test the model without the LLM |
| 0.1.3 | Prompt Management | Move the naming prompt into a versioned YAML template, add `PromptManager` for loading and rendering, wire `PromptManager` to `LLMClient`, and handle reasoning-model responses that leave `content` null |
| 0.1.2 | LLM Client | Add `LLMClient` OpenRouter chat wrapper over `httpx`, wire the API key from `.env`, raise clear errors on transport and malformed-response failures |
| 0.1.1 | Configuration | Add YAML settings and `.env` loading, implement `ConfigLoader`, move code to a `src/` layout |
| 0.1.0 | Project Setup | Initialize `uv` project, add core dependencies (pydantic, httpx, pyyaml, python-dotenv), configure pytest and ruff, add runnable `blueprint_generator` package skeleton |

## [0.1.5] - 2026-09-17

### Added

- `src/project_planner.py` — `ProjectPlanner` with `generate_project_name(project_idea)`: validates the idea is non-empty, builds the system/user messages from `PromptManager`, and returns a validated `ProjectName` via `LLMClient.structured_output()`
- Verified end-to-end through the planner service: idea → blueprints-doc search platform → `DocuSearch AI`

### Notes

- `ProjectPlanner` owns application logic; `LLMClient` only communicates with the LLM provider.
- Milestone M1 (idea in → project name out) of the [Roadmap](docs/ROADMAP.md) is reachable as a service.

## [0.1.4] - 2026-09-17

### Added

- `src/models.py` — `ProjectName` Pydantic model with a `name` field constrained to 2–100 characters
- `LLMClient.structured_output(messages, response_model)` — calls `chat()`, parses the response as JSON, and validates it into the given Pydantic model; raises `RuntimeError` on invalid JSON or failed schema validation
- `tests/test_models.py` — unit tests for `ProjectName` without any LLM call (valid name accepted, empty name rejected)

### Changed

- `prompts/project_name.yaml` now instructs the model to return JSON `{"name": "ProjectName"}`
- Verified end-to-end: idea → JSON response → validated `ProjectName` (`DocuSearch AI`)

### Changed (cleanup)

- `prompt_manager.py` raises `TypeError` (not `ValueError`) for non-mapping prompt files, matching `config_loader.py`

### Notes

- Phase 5 — Structured Output of the [Roadmap](docs/ROADMAP.md).

## [0.1.3] - 2026-09-17

### Added

- `prompts/project_name.yaml` — versioned (`v1`) system/user prompt template for naming projects, rendered via `str.format` with a `{project_idea}` placeholder
- `src/prompt_manager.py` — `PromptManager` with `load()` (reads and validates the YAML prompt mapping), `render_user_prompt(project_idea)`, and `get_system_prompt()`
- `LLMClient.chat()` now falls back to the `reasoning` field when a reasoning model returns `content: null`, and raises `RuntimeError("Unexpected LLM response format.")` if both are absent

### Changed

- Verified the first end-to-end AI workflow: idea in → `PromptManager` renders prompts → `LLMClient` → project name out (`CogniSearch`)

### Notes

- Phase 4 — Prompt Management of the [Roadmap](docs/ROADMAP.md).
- `openrouter/free` occasionally routes to reasoning models that return the answer in `reasoning` with a null `content`; the fallback in `LLMClient` covers this.

## [0.1.2] - 2026-09-17

### Added

- `src/llm_client.py` — `LLMClient.chat(messages) -> str` calling OpenRouter `/chat/completions` with `httpx`
- Transport error handling: `TimeoutException`, `HTTPStatusError`, and `RequestError` are re-raised as `RuntimeError` with clear messages
- Malformed-response handling: missing/invalid `choices[0].message.content` raises `RuntimeError("Unexpected LLM response format.")`

### Changed

- `config/settings.yaml` model set to `openrouter/free` for verification (individual `:free` models returned 429; revert to `openai/gpt-4o-mini` once the account has credits)

### Removed

- Temporary `src/test_llm.py` manual verification script

### Notes

- Phase 3 — LLM Client of the [Roadmap](docs/ROADMAP.md).
- Verified end-to-end against the live API: document-project prompt → `DocuHub`.

## [0.1.1] - 2026-09-17

### Added

- `config/settings.yaml` with `application` and `llm` settings (provider, model, temperature, max_tokens)
- `src/config_loader.py` — `ConfigLoader` that reads and validates a YAML mapping, raising `FileNotFoundError`/`TypeError` on bad input
- `OPENROUTER_API_KEY` entry in `.env` and `.env.example`
- Smoke test wiring for the `src/` layout (`pythonpath = ["src"]`)

### Changed

- Moved `main.py` from the `blueprint_generator/` package into `src/`; removed the package directory
- `tests/test_main.py` now imports `from main import main`

### Removed

- Temporary `src/test_config.py` manual verification script (kept only during validation)

### Notes

- Phase 2 — Project Setup of the [Roadmap](docs/ROADMAP.md).

## [0.1.0] - 2026-09-17

### Added

- `uv`-managed Python project (`pyproject.toml`, `.venv`)
- Runtime dependencies: `pydantic`, `httpx`, `pyyaml`, `python-dotenv`
- Dev dependencies: `pytest`, `pytest-asyncio`, `ruff`
- `blueprint_generator` package with `main()` entry point
- `tests/test_main.py` smoke test
- `pytest` configuration (`testpaths`, `pythonpath`)
- `.gitignore`, `.env`, `.env.example` configuration files

### Notes

- Phase 2 — Project Setup of the [Roadmap](docs/ROADMAP.md).
