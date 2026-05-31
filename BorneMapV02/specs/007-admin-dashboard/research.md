# Research: Admin Dashboard

## Technology Decisions

### Decision: React + Vite for Admin Dashboard
- **Rationale**: Constitution Principle II locks React for admin portal. Vite is chosen over Next.js because the spec requires zero backend integration — no SSR, no API routes, no data fetching. Vite provides instant HMR and a simpler configuration surface for a purely client-side mock app.
- **Alternatives considered**: Next.js (overkill for static-only), Create React App (deprecated), plain HTML/JS (loses component architecture)

### Decision: Inline Style Objects (no CSS framework)
- **Rationale**: The reference code from the spec uses explicit `StyleSheet.create`-style inline objects. This keeps dependencies zero and matches the mobile-driver's existing styling approach (React Native StyleSheet). No Tailwind, no CSS modules — the blueprint matrix uses inline styles consistently.
- **Alternatives considered**: Tailwind CSS (adds build complexity for a sandbox), CSS Modules (different pattern from mobile-driver)

### Decision: Monorepo with Independent `apps/` Directories
- **Rationale**: Each app (admin-dashboard, web-driver, mobile-driver) has a different build tool (Vite vs Expo) and platform target. Keeping them independent avoids toolchain conflicts. The blueprint matrix defines layout behavior per app — they share concepts, not code.
- **Alternatives considered**: Monorepo with shared component library (premature — no shared components between admin tables and map views), separate repos (too much overhead)

### Decision: Static Mock Data Inline in Components
- **Rationale**: Spec explicitly requires zero backend integration. Mock arrays live in `src/data/mockData.js` for admin-dashboard and inline within MapPortal/MapScreen for the map apps. No fetch, no async state — data is synchronously available.
- **Alternatives considered**: MSW (mock service worker — adds dependency for no benefit), JSON files (same outcome, more files)

### Decision: Cross-Platform Updates Target Specific Components
- **Rationale**: web-driver `MapPortal.jsx` and mobile-driver `MapScreen.js` are rewritten per the blueprint reference code. The rest of each app (navigation, routing, app shell) remains untouched. This minimizes scope while achieving layout parity.
- **Alternatives considered**: Full app rewrite (unnecessary — only layout and components change, not app structure)

## No Unresolved Clarifications

All [NEEDS CLARIFICATION] markers from the spec were resolved during the `/speckit.clarify` session. No further research dependencies.
