# Research: Monorepo + Tooling Foundation

## 1. Toolchain Version Decisions

### Decision: Rust edition 2024

**Rationale**: Rust edition 2024 is the latest stable edition, providing modern language features (anonymous lifetimes, `impl Trait` in closures, `unsafe` attributes in extern blocks) that reduce boilerplate across the 5 backend services. Edition 2024 is forward-compatible with all future releases and is the recommended baseline for new Rust projects started in 2026.

**Alternatives considered**:
- **Edition 2021**: Stable and proven but missing ergonomic improvements that reduce code volume in service crates. No reason to start a new project on an older edition.
- **Edition 2018/2015**: Far too old; missing fundamental features like `impl Trait`, `const generics`, and let-else statements.

### Decision: Node.js 22 LTS

**Rationale**: Node.js 22 LTS is the current Active LTS release (April 2026), providing stable npm workspace support, built-in `--experimental-require-module` for ESM/CJS interop, and the latest V8 engine. It will receive security updates through April 2027.

**Alternatives considered**:
- **Node.js 20 LTS**: Previous LTS, still active but older V8 engine and npm version. No advantage over 22 for a new project.
- **Node.js 23/24 (current)**: Not LTS; risk of breaking changes during the project lifecycle.

### Decision: npm workspaces

**Rationale**: npm is bundled with Node.js (no extra install), uses the widely understood `package-lock.json` format, and its workspace support handles the shared-package linking needs of this monorepo. For the scale of this project (6 packages, 4 apps), the performance difference between npm and pnpm is negligible, and npm eliminates a toolchain dependency.

**Alternatives considered**:
- **pnpm**: Faster installs, strict dependency isolation. Adds another tool to the CI/CD pipeline and developer environment. Worth migrating to if the monorepo grows beyond ~20 packages.
- **yarn**: Mature workspace support but requires separate installation and has a different lockfile format.

## 2. Build Tooling Choices

### Decision: Cargo workspace for Rust

**Rationale**: Rust's built-in workspace feature is the standard way to manage multi-crate repositories. It provides shared dependency resolution, a single `Cargo.lock`, and workspace-level commands (`cargo build --workspace`). No alternative is viable for Rust.

### Decision: Vite for web apps

**Rationale**: Vite is the de-facto standard React bundler, providing fast HMR, TypeScript support out of the box, and a simple configuration model. It is the explicit choice in the Constitution ("React + Vite, no Next.js"). Follows the project's REST-only architecture principle (SSR not needed for a map-first SPA).

**Alternatives considered**:
- **CRA (Create React App)**: Deprecated by the React team. Not viable for new projects.
- **Next.js**: Adds SSR/SSG complexity not needed for this project. Constitution explicitly forbids it.

### Decision: React Native Expo for mobile

**Rationale**: Expo provides a managed workflow that simplifies React Native development, with built-in support for over-the-air updates, EAS Build, and Expo Go for testing. For a map-centric app, Expo's `expo-location` and map module integrations reduce boilerplate.

## 3. Infrastructure Approach

### Decision: Docker Compose skeleton in Sprint 1

**Rationale**: Creating the Compose skeleton early (stub services, internal networking, Traefik routing) validates the deployment topology before real code is written. This reduces the risk of networking/configuration issues in Sprint 2 when the full infrastructure goes live. The skeleton is intentionally non-functional (empty health endpoints) to keep Sprint 1 focused on build tooling.

## 4. Shared Contract Modeling

### Decision: TypeScript-first, mirrored in Rust

**Rationale**: All shared contracts (API envelopes, event taxonomy, error codes) are authored as TypeScript types in the shared packages (`api-contracts`, `event-taxonomy`). These serve as the canonical definitions. Rust equivalents in `common-types` are manually mirrored (or generated via codegen when complexity grows). This dual-source approach is pragmatic for a solo-dev team — the TypeScript types are the source of truth, and Rust types are kept in sync manually.

**Alternatives considered**:
- **Single source of truth in JSON Schema**: More tooling overhead; adds a compilation step. Not justified for the current contract complexity.
- **Single source in Rust + codegen to TypeScript**: Unnatural for a project where the majority of API consumers are TypeScript frontends.
