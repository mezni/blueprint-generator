# Data Model: CI/CD Pipeline & Delivery Automation

**Phase**: 1 — Design & Contracts
**Date**: 2026-05-31
**Source Spec**: [spec.md](../spec.md)

This document defines the infrastructure entities of the BorneMap CI/CD pipeline.

## Workflow Entities

| Workflow | Trigger | Jobs | Target | Retries |
|----------|---------|------|--------|---------|
| `pr-validation` | pull_request, workflow_dispatch | lint, test, build, contract-validation | CI runner | 0 (fail fast) |
| `integration` | push to main, schedule (nightly) | runtime-boot, health-verify, api-contract-check, cross-service-test | CI runner | 1 (flaky test retry) |
| `build-images` | push to main, version tag | matrix: build-7-images | CI runner | 0 |
| `deploy` | workflow_dispatch (manual) | pull-images, validate-config, rolling-restart, post-deploy-verify | Production host | 0 |
| `release` | version tag push | generate-notes, build-manifest, create-release | CI runner | 0 |
| `security-audit` | schedule (weekly) | cargo-audit, npm-audit, trivy-scan | CI runner | 0 |

## Environment Variables

| Variable | Workflow | Purpose |
|----------|----------|---------|
| `GHCR_TOKEN` | build-images, deploy | Registry authentication |
| `DEPLOY_HOST` | deploy | Production host address |
| `DEPLOY_USER` | deploy | Production SSH user |
| `DEPLOY_KEY` | deploy | Production SSH private key |
| `CARGO_TERM_COLOR` | pr-validation, build-images | CI output formatting |
| `RUSTFLAGS` | pr-validation | Rust compiler flags |
| `IMAGE_TAG` | build-images | Git SHA or version tag |

## Artifact Retention Policy

| Artifact Source | Retention | Storage |
|----------------|-----------|---------|
| PR workflow artifacts | 7 days | GitHub Actions |
| Main build artifacts | 30 days | GitHub Actions |
| Release images | Indefinite | GHCR |
| Release manifests | Indefinite | GitHub Releases |

## Trigger Configuration

| Workflow | Concurrency Group | Cancel In-Progress |
|----------|-------------------|--------------------|
| pr-validation | `pr-${{ github.head_ref }}` | true |
| integration | `integration-main` | true |
| build-images | `build-${{ github.ref }}` | true |
| deploy | `deploy-production` | false (queue) |
| release | `release-${{ github.ref }}` | true |
| security-audit | `audit-weekly` | true (allow skip) |
