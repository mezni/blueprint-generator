# Changelog

All notable changes to this project are documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/).

## Version History

| Version | Feature Domain | Key Objectives |
| --- | --- | --- |
| 0.1.0 | Project Setup | Initialize `uv` project, add core dependencies (pydantic, httpx, pyyaml, python-dotenv), configure pytest and ruff, add runnable `blueprint_generator` package skeleton |

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
