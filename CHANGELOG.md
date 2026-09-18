# Changelog

All notable changes to this project are documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/).

## Version History

| Version | Feature Domain | Key Objectives |
| --- | --- | --- |
| 0.1.1 | Configuration | Add YAML settings and `.env` loading, implement `ConfigLoader`, move code to a `src/` layout |
| 0.1.0 | Project Setup | Initialize `uv` project, add core dependencies (pydantic, httpx, pyyaml, python-dotenv), configure pytest and ruff, add runnable `blueprint_generator` package skeleton |

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
