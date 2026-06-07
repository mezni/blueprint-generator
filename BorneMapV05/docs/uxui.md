UX/UI Specification (v1.0) — Bornemap
1. Purpose

This document defines the system-level UX/UI behavior rules for the Bornemap platform.

It is NOT a visual design guide.

It defines:

Interaction behavior
Layout logic
Cross-app UX consistency rules
Map interaction model
Authentication flow behavior
RTL / multilingual constraints
Accessibility baseline
State management UX rules
2. UX Design Philosophy
2.1 Map-first product

The primary interface is always:

Map → Stations → Actions

Everything else is secondary.

2.2 Progressive disclosure

Users should never be overwhelmed.

Information hierarchy:

Map
Stations list / markers
Station details
Actions (favorite, review, manage)
2.3 No login friction (critical rule)
Public browsing is always allowed
Authentication is triggered ONLY on action

Example triggers:

Favorite station
Submit review
Partner dashboard access
Admin actions
2.4 System consistency

All apps MUST behave identically for:

Station cards
Map markers
Availability badges
Review rendering
Error states
3. Application UX Model
3.1 Driver Apps (Web + Mobile)

Modes:

Public mode (anonymous)
Registered mode (authenticated)

Capabilities:

Feature	Public	Registered
Map browsing	✅	✅
Station view	✅	✅
Search/filter	✅	✅
Favorites	❌	✅
Reviews	Read-only	CRUD
Profile	❌	✅
3.2 Partner Dashboard

Purpose:

Operational control panel

Focus:

speed
density
efficiency

NOT aesthetics.

3.3 Admin Dashboard

Purpose:

System-wide control interface

Focus:

clarity
governance
moderation
analytics
4. Layout System
4.1 Global structure

All web apps use:

Sidebar + Main Content + Context Panel
4.2 Sidebar (fixed)
Width: 260px
Collapsible sections
Persistent navigation

Sections:

Dashboard
Stations
Users
Reports
Settings
4.3 Main Content
Scroll isolated (overflow-y)
Grid-based layout
Responsive container
4.4 Context Panel

Used for:

Station details
Filters
Review forms
Editing forms

Behavior:

slides in (desktop)
bottom sheet (mobile)
5. Map UX System (CORE)
5.1 Map is the default state

On load:

map always visible
stations loaded immediately
5.2 Map state machine
State 1 — Idle
Default center (user location or Tunisia fallback)
Minimal markers
State 2 — Viewport Active

Triggered when user moves map:

bbox query triggered
markers refresh
clustering enabled
State 3 — Station Selected
station pinned
details panel opens
map recenters optionally
surrounding markers dimmed
6. Station Interaction Model
6.1 Station Card

Must display:

name
status badge
distance
availability
quick action (favorite if logged in)
6.2 Station Detail Flow

Flow:

Map click → Card expand → Detail panel

Detail includes:

chargers
reviews
availability
actions
6.3 Availability Badge Rules
Status	Meaning
available	fully operational
limited	partial availability
unavailable	offline
7. Authentication UX (Progressive Auth)
7.1 Principle

No login screen unless necessary.

7.2 Flow
User action → Check auth → If missing → Login modal → Resume action
7.3 Gated actions
favorite station
submit review
partner access
admin access
8. State Management UX Rules
8.1 Loading states

Must always use:

skeletons (NOT spinners)
8.2 Empty states

Must include:

explanation
retry action (if applicable)
no dead ends
8.3 Error states

Must include:

retry button
non-blocking UI fallback
no full screen failure unless critical
9. RTL + Multilingual System
9.1 Supported languages
French (LTR default)
Arabic (RTL)
9.2 RTL rules

When Arabic is active:

layout direction flips
sidebar moves to right
icons mirror directionality
text alignment switches automatically
9.3 Map exception rule

Map itself:

NEVER mirrored
only UI overlays adapt to RTL
10. Accessibility (WCAG 2.1 AA)

Mandatory:

keyboard navigation everywhere
visible focus states
ARIA labels on all controls
contrast compliance
semantic HTML structure
11. Interaction Rules
11.1 Map triggers UI updates
movement → refetch stations
zoom → cluster adjustment
click → station focus
11.2 Filtering behavior

Filters:

always applied to viewport query
always debounce requests
always reset pagination
11.3 Mutation behavior

Any mutation MUST:

update backend
invalidate React Query cache
update UI optimistically (if safe)
12. Cross-App Consistency Rules

All apps MUST share:

station card behavior
badge logic
error handling patterns
loading skeletons
review rendering rules
13. Design System Contract
13.1 Layer hierarchy
Layer	Responsibility
Tokens	raw values
Tailwind theme	mapping
shadcn/ui	base components
Domain components	business UI
13.2 Forbidden rules
no inline hex colors
no arbitrary spacing values
no duplicate UI components across apps
14. Performance UX Rules
Map markers must remain smooth under 1000+ points
UI transitions must remain under 150ms perceived latency
API debounce for map movement: 300–500ms
Skeletons used instead of spinners for perceived performance
15. Notifications UX

Types:

success → green accent
warning → amber
error → red
info → indigo

Rules:

non-blocking unless critical
auto-dismiss after timeout
grouped when repeated
16. Security UX Constraints
no sensitive data in UI state logs
no role exposure in frontend except UI gating
admin UI must clearly isolate destructive actions
17. UX Anti-Patterns (Forbidden)
login wall before browsing
full page reloads on map movement
blocking modals for non-critical actions
inconsistent station card formats
UI-specific business logic duplicated per app
18. System Summary

This UX/UI system enforces:

map-first interaction model
progressive authentication
strict cross-app consistency
RTL-ready architecture
accessibility compliance
state-driven UI design
no UI-business logic entanglement
