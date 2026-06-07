# Tasks: Driver Mobile App

**Input**: Design documents from `/specs/012-driver-mobile-app/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are NOT included as per feature specification (React Native E2E with Detox would be required for production-ready tests)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US5)
- Include exact file paths in descriptions

## Path Conventions

- **Mobile App**: `apps/driver-mobile/src/` at repository root
- **Tests**: `apps/driver-mobile/tests/` at repository root
- **Contracts**: `specs/012-driver-mobile-app/contracts/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create Expo project structure in apps/driver-mobile/
- [ ] T002 [P] Initialize TypeScript configuration in apps/driver-mobile/tsconfig.json
- [ ] T003 [P] Install core dependencies in apps/driver-mobile/package.json (react-native, expo, react-query, etc.)
- [ ] T004 [P] Initialize Tailwind CSS in apps/driver-mobile/tailwind.config.ts
- [ ] T005 [P] Create project directory structure in apps/driver-mobile/src/ (components, pages, hooks, lib, etc.)
- [ ] T006 [P] Configure environment variables in apps/driver-mobile/.env.example
- [ ] T007 [P] Create base styling setup in apps/driver-mobile/src/styles/index.ts

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T008 Setup API client configuration in apps/driver-mobile/src/lib/api.ts (using existing api-client package)
- [ ] T009 [P] Create authentication context provider in apps/driver-mobile/src/hooks/useAuth.tsx (using existing auth-client package)
- [ ] T010 [P] Implement event emission utility in apps/driver-mobile/src/lib/clickstream.ts (using existing event-taxonomy package)
- [ ] T011 Create reusable UI components in apps/driver-mobile/src/components/ui/ (button, card, input, modal)
- [ ] T012 [P] Setup React Navigation in apps/driver-mobile/src/navigation/AppNavigator.tsx
- [ ] T013 [P] Create ErrorBoundary component in apps/driver-mobile/src/components/ErrorBoundary.tsx
- [ ] T014 [P] Create AuthGate component in apps/driver-mobile/src/components/AuthGate.tsx
- [ ] T015 [P] Configure Secure Store for token storage in apps/driver-mobile/src/lib/storage.ts

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Map Discovery (Priority: P1) 🎯 MVP

**Goal**: A registered driver opens the mobile app and sees stations nearby on a map, with distance indicators and ability to filter by connector type and availability

**Independent Test**: Open the app on a device with GPS enabled, navigate to a location, and verify that stations appear on the map with accurate distances and available filter options

### Implementation for User Story 1

- [ ] T016 [US1] Create dashboard page with map container in apps/driver-mobile/src/pages/DashboardPage.tsx
- [ ] T017 [P] [US1] Implement map marker component in apps/driver-mobile/src/components/ui/map-container.tsx
- [ ] T018 [P] [US1] Create distance calculation utility in apps/driver-mobile/src/utils/format.ts (Haversine formula)
- [ ] T019 [P] [US1] Implement station query hook in apps/driver-mobile/src/hooks/useStations.ts
- [ ] T020 [US1] Integrate React Query cache with map viewport updates in apps/driver-mobile/src/pages/DashboardPage.tsx
- [ ] T021 [P] [US1] Create filter component for connector type in apps/driver-mobile/src/components/Filters.tsx
- [ ] T022 [P] [US1] Create filter component for availability in apps/driver-mobile/src/components/Filters.tsx
- [ ] T023 [US1] Implement map interaction debouncing (300-500ms) in apps/driver-mobile/src/hooks/useStations.ts
- [ ] T024 [US1] Emit station marker click event in apps/driver-mobile/src/components/ui/map-container.tsx
- [ ] T025 [P] [US1] Add map loading skeleton in apps/driver-mobile/src/components/ui/map-container.tsx
- [ ] T026 [US1] Configure map default region (Tunis coordinates) in apps/driver-mobile/src/pages/DashboardPage.tsx
- [ ] T027 [US1] Implement GPS permission handling in apps/driver-mobile/src/hooks/useStations.ts
- [ ] T028 [US1] Handle map error states and retry logic in apps/driver-mobile/src/pages/DashboardPage.tsx

**Checkpoint**: At this point, User Story 1 should be fully functional - map discovery with stations within radius

---

## Phase 4: User Story 2 - Station Details (Priority: P1) 🎯 MVP

**Goal**: A user taps on a station marker to view detailed information including name, description, charger specifications, real-time availability, and nearby reviews

**Independent Test**: Navigate to a station on the map, tap the marker, and verify all station information displays correctly with accurate availability status

### Implementation for User Story 2

- [ ] T029 [P] [US2] Create station detail page in apps/driver-mobile/src/pages/StationDetailPage.tsx
- [ ] T030 [P] [US2] Create station header component in apps/driver-mobile/src/components/StationHeader.tsx
- [ ] T031 [P] [US2] Create charger list component in apps/driver-mobile/src/components/ChargerList.tsx
- [ ] T032 [P] [US2] Create review list component in apps/driver-mobile/src/components/ReviewList.tsx
- [ ] T033 [US2] Implement station detail query hook in apps/driver-mobile/src/hooks/useStationDetail.ts
- [ ] T034 [US2] Create station detail component with charger display in apps/driver-mobile/src/pages/StationDetailPage.tsx
- [ ] T035 [P] [US2] Implement station loader skeleton in apps/driver-mobile/src/pages/StationDetailPage.tsx
- [ ] T036 [US2] Emit station opened event in apps/driver-mobile/src/pages/StationDetailPage.tsx
- [ ] T037 [US2] Handle station navigation from map marker click in apps/driver-mobile/src/components/ui/map-container.tsx
- [ ] T038 [US2] Implement real-time availability status display in apps/driver-mobile/src/components/StationHeader.tsx
- [ ] T039 [US2] Handle station not found errors in apps/driver-mobile/src/pages/StationDetailPage.tsx
- [ ] T040 [US2] Add station review count display in apps/driver-mobile/src/pages/StationDetailPage.tsx

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - map discovery + station details

---

## Phase 5: User Story 3 - Favorites Management (Priority: P2)

**Goal**: A registered driver can mark stations as favorites for quick access and remove them when no longer needed

**Independent Test**: Login as a registered driver, add a station to favorites, navigate away, return to the map, and verify the favorite appears in the favorites list

### Implementation for User Story 3

- [ ] T041 [P] [US3] Create favorites page in apps/driver-mobile/src/pages/FavoritesPage.tsx
- [ ] T042 [P] [US3] Implement favorites list component in apps/driver-mobile/src/components/FavoritesList.tsx
- [ ] T043 [US3] Create favorites query hook in apps/driver-mobile/src/hooks/useFavorites.ts
- [ ] T044 [US3] Implement favorites toggle hook with optimistic UI in apps/driver-mobile/src/hooks/useFavorites.ts
- [ ] T045 [US3] Create favorite button component in apps/driver-mobile/src/components/FavoriteButton.tsx
- [ ] T046 [P] [US3] Implement favorites cache in apps/driver-mobile/src/lib/storage.ts (AsyncStorage)
- [ ] T047 [US3] Integrate favorites toggle in station detail page in apps/driver-mobile/src/pages/StationDetailPage.tsx
- [ ] T048 [US3] Emit favorite add/remove events in apps/driver-mobile/src/components/FavoriteButton.tsx
- [ ] T049 [US3] Handle favorites sync when network restores in apps/driver-mobile/src/hooks/useFavorites.ts
- [ ] T050 [US3] Remove favorites for soft-deleted stations in apps/driver-mobile/src/hooks/useFavorites.ts
- [ ] T051 [US3] Implement favorites offline mode in apps/driver-mobile/src/pages/FavoritesPage.tsx

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently - map + details + favorites

---

## Phase 6: User Story 4 - Reviews & Ratings (Priority: P2)

**Goal**: A registered driver can submit reviews for stations they have visited and view reviews from other users

**Independent Test**: Login as a registered driver, visit a station, submit a review with rating and comment, then navigate to the station details to view the submitted review

### Implementation for User Story 4

- [ ] T052 [P] [US4] Create review form component in apps/driver-mobile/src/components/ReviewForm.tsx
- [ ] T053 [P] [US4] Create review rating selector component in apps/driver-mobile/src/components/RatingSelector.tsx
- [ ] T054 [P] [US4] Implement review mutation hook in apps/driver-mobile/src/hooks/useReviews.ts
- [ ] T055 [US4] Integrate review form in station detail page in apps/driver-mobile/src/pages/StationDetailPage.tsx
- [ ] T056 [US4] Implement review submission with optimistic UI in apps/driver-mobile/src/hooks/useReviews.ts
- [ ] T057 [P] [US4] Implement review validation in apps/driver-mobile/src/utils/validation.ts
- [ ] T058 [US4] Emit review submission event in apps/driver-mobile/src/hooks/useReviews.ts
- [ ] T059 [US4] Handle review validation errors in apps/driver-mobile/src/components/ReviewForm.tsx
- [ ] T060 [US4] Display user's own reviews first in apps/driver-mobile/src/components/ReviewList.tsx
- [ ] T061 [US4] Implement review load error handling in apps/driver-mobile/src/pages/StationDetailPage.tsx

**Checkpoint**: At this point, User Stories 1, 2, 3, AND 4 should all work independently - map + details + favorites + reviews

---

## Phase 7: User Story 5 - Login Flow (Priority: P1)

**Goal**: A user can authenticate with Keycloak via OAuth2, supporting both password-based login and social login providers

**Independent Test**: Navigate to a gated action (favorites/reviews) without login, verify login modal appears, complete authentication, and return to the previous action

### Implementation for User Story 5

- [ ] T062 [P] [US5] Implement Keycloak OAuth2 integration in apps/driver-mobile/src/hooks/useAuth.tsx (using existing auth-client)
- [ ] T063 [US5] Create login modal component in apps/driver-mobile/src/components/LoginModal.tsx
- [ ] T064 [P] [US5] Implement password login form in apps/driver-mobile/src/components/PasswordLoginForm.tsx
- [ ] T065 [P] [US5] Implement social login buttons (Google/Facebook) in apps/driver-mobile/src/components/SocialLoginButtons.tsx
- [ ] T066 [US5] Handle OAuth redirect callback in apps/driver-mobile/src/main.tsx
- [ ] T067 [US5] Integrate AuthGate in AppNavigator in apps/driver-mobile/src/navigation/AppNavigator.tsx
- [ ] T068 [P] [US5] Implement token refresh logic in apps/driver-mobile/src/hooks/useAuth.tsx
- [ ] T069 [US5] Emit auth started/succeeded/failed events in apps/driver-mobile/src/hooks/useAuth.tsx
- [ ] T070 [US5] Handle auth state persistence across app restarts in apps/driver-mobile/src/hooks/useAuth.tsx
- [ ] T071 [US5] Handle logout functionality in apps/driver-mobile/src/components/LogoutButton.tsx
- [ ] T072 [US5] Handle token expiration and re-authentication in apps/driver-mobile/src/hooks/useAuth.tsx

**Checkpoint**: At this point, User Stories 1, 2, 3, 4, AND 5 should all work independently - map + details + favorites + reviews + login

---

## Phase 8: User Story 6 - RTL Support (Priority: P3)

**Goal**: The mobile app supports Right-to-Left (RTL) languages, primarily Arabic, with full layout flipping and proper text direction

**Independent Test**: Switch the app to Arabic language, verify all UI elements flip layout direction, text renders correctly, and interactive elements remain accessible

### Implementation for User Story 6

- [ ] T073 [P] [US6] Implement locale detection in apps/driver-mobile/src/utils/i18n.ts
- [ ] T074 [US6] Configure RTL support in apps/driver-mobile/src/navigation/AppNavigator.tsx (react-native-locale-identify)
- [ ] T075 [P] [US6] Create RTL-aware text component in apps/driver-mobile/src/components/RtlText.tsx
- [ ] T076 [P] [US6] Update all components to use RTL text alignment in apps/driver-mobile/src/components/ui/
- [ ] T077 [US6] Test Arabic language support in apps/driver-mobile/src/pages/StationDetailPage.tsx
- [ ] T078 [US6] Test Arabic language support in apps/driver-mobile/src/pages/DashboardPage.tsx
- [ ] T079 [US6] Test Arabic language support in apps/driver-mobile/src/components/FavoriteButton.tsx
- [ ] T080 [US6] Update language selector component in apps/driver-mobile/src/components/LanguageSelector.tsx
- [ ] T081 [US6] Test RTL layout flipping for all map markers in apps/driver-mobile/src/components/ui/map-container.tsx
- [ ] T082 [US6] Handle RTL-specific styling in apps/driver-mobile/src/styles/index.ts
- [ ] T082.5 [P] [US6] Create RTL validation tests in apps/driver-mobile/src/__tests__/rtl.test.tsx (verify Arabic text direction, layout flipping, text alignment)
- [ ] T082.6 [US6] Test RTL layout for all major screens in apps/driver-mobile/src/__tests__/rtl.test.tsx (station detail, favorites, map, profiles)
- [ ] T082.7 [US6] Test RTL text rendering with RTL-aware text components in apps/driver-mobile/src/__tests__/rtl.test.tsx
- [ ] T082.8 [US6] Verify RTL layout doesn't break interactive elements in apps/driver-mobile/src/__tests__/rtl.test.tsx (buttons, inputs, markers)

**Checkpoint**: RTL support implemented and tested for all major user journeys

---

## Phase 9: User Story 7 - Offline-Safe UI (Priority: P3)

**Goal**: The mobile app maintains a functional UI even when network connectivity is unavailable, with appropriate loading states and error handling

**Independent Test**: Enable airplane mode in the app, navigate through different screens, verify appropriate offline UI states, and test reconnecting to network

### Implementation for User Story 7

- [ ] T083 [P] [US7] Implement offline queue in apps/driver-mobile/src/lib/offlineQueue.ts
- [ ] T084 [P] [US7] Create network status hook in apps/driver-mobile/src/hooks/useNetworkStatus.ts
- [ ] T085 [US7] Implement offline state management in apps/driver-mobile/src/components/OfflineBanner.tsx
- [ ] T086 [US7] Integrate offline queue with favorites in apps/driver-mobile/src/hooks/useFavorites.ts
- [ ] T087 [US7] Integrate offline queue with reviews in apps/driver-mobile/src/hooks/useReviews.ts
- [ ] T088 [P] [US7] Create offline loading skeleton in apps/driver-mobile/src/components/ui/OfflineSkeleton.tsx
- [ ] T089 [US7] Implement background sync for queued operations in apps/driver-mobile/src/hooks/useOfflineSync.ts
- [ ] T090 [US7] Handle network connectivity changes in apps/driver-mobile/src/hooks/useNetworkStatus.ts
- [ ] T091 [US7] Implement retry logic for failed requests in apps/driver-mobile/src/lib/api.ts
- [ ] T092 [US7] Preserve navigation state during offline mode in apps/driver-mobile/src/navigation/AppNavigator.tsx
- [ ] T093 [US7] Test offline state persistence across app restarts in apps/driver-mobile/src/hooks/useOfflineSync.ts
- [ ] T094 [US7] Display appropriate offline error messages in apps/driver-mobile/src/components/OfflineError.tsx

**Checkpoint**: All user stories work offline with proper state preservation and sync

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T095 [P] Create user profile page in apps/driver-mobile/src/pages/ProfilePage.tsx
- [ ] T096 [P] Implement PIN/biometric lock in apps/driver-mobile/src/lib/storage.ts (Secure Store)
- [ ] T097 [P] Add performance monitoring and error telemetry in apps/driver-mobile/src/lib/monitoring.ts
- [ ] T098 [P] Update documentation with component usage examples in apps/driver-mobile/README.md
- [ ] T099 [P] Add code comments and documentation in apps/driver-mobile/src/
- [ ] T100 [P] Implement code splitting and lazy loading for heavy components in apps/driver-mobile/src/
- [ ] T101 [P] Add performance optimizations (virtualized lists, memoization) in apps/driver-mobile/src/
- [ ] T102 [P] Test accessibility (WCAG 2.1 AA) across all screens in apps/driver-mobile/src/
- [ ] T103 Implement QR code scanning for station codes in apps/driver-mobile/src/components/StationQRScanner.tsx
- [ ] T104 [P] Add push notification service in apps/driver-mobile/src/services/notification.ts
- [ ] T105 [P] Create RTL language selector component in apps/driver-mobile/src/components/LanguageSelector.tsx
- [ ] T106 [P] Implement share station feature in apps/driver-mobile/src/components/StationShare.tsx
- [ ] T107 Run quickstart.md validation and documentation review
- [ ] T107.5 [P] Implement remote logging service integration in apps/driver-mobile/src/lib/monitoring.ts (error telemetry with request context, retry logic for failed uploads)
- [ ] T107.6 [P] Add performance monitoring hooks in apps/driver-mobile/src/hooks/usePerformanceMetrics.ts (app launch time, interaction latency, memory usage tracking)
- [ ] T107.7 [P] Create error boundary with logging integration in apps/driver-mobile/src/components/ErrorBoundary.tsx (log errors to remote service before crash)
- [ ] T107.8 [P] Implement error telemetry in apps/driver-mobile/src/lib/monitoring.ts (structured JSON logging with request IDs, stack traces, user context)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-9)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 10)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 5 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 6 (P3)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 7 (P3)**: Can start after Foundational (Phase 2) - No dependencies on other stories

### Within Each User Story

- Core components before services
- Services before UI integration
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Within each user story, tasks marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1 (Map Discovery)

```bash
# Phase 2 foundation complete - start User Story 1 in parallel:

# Task T017 (map marker component):
Task: "Implement map marker component in apps/driver-mobile/src/components/ui/map-container.tsx"

# Task T018 (distance calculation utility):
Task: "Create distance calculation utility in apps/driver-mobile/src/utils/format.ts (Haversine formula)"

# Task T021 (connector type filter):
Task: "Create filter component for connector type in apps/driver-mobile/src/components/Filters.tsx"

# Task T022 (availability filter):
Task: "Create filter component for availability in apps/driver-mobile/src/components/Filters.tsx"
```

---

## Parallel Example: User Story 3 (Favorites)

```bash
# Within Phase 5, favorite functionality can be parallelized:

# Task T041 (favorites page):
Task: "Create favorites page in apps/driver-mobile/src/pages/FavoritesPage.tsx"

# Task T042 (favorites list component):
Task: "Implement favorites list component in apps/driver-mobile/src/components/FavoritesList.tsx"

# Task T043 (favorites query hook):
Task: "Create favorites query hook in apps/driver-mobile/src/hooks/useFavorites.ts"

# Task T046 (favorites cache):
Task: "Implement favorites cache in apps/driver-mobile/src/lib/storage.ts (AsyncStorage)"
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 + 5)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 - Map Discovery
4. Complete Phase 4: User Story 2 - Station Details
5. Complete Phase 7: User Story 5 - Login Flow
6. **STOP and VALIDATE**: Test User Stories 1, 2, AND 5 independently
7. Deploy/demo if ready - Core journey (discovery + details + login) is complete!

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (Map Discovery!)
3. Add User Story 2 → Test independently → Deploy/Demo (Station Details!)
4. Add User Story 5 → Test independently → Deploy/Demo (Login Flow!)
5. Add User Story 3 → Test independently → Deploy/Demo (Favorites!)
6. Add User Story 4 → Test independently → Deploy/Demo (Reviews!)
7. Add User Story 6 → Test independently → Deploy/Demo (RTL!)
8. Add User Story 7 → Test independently → Deploy/Demo (Offline!)
9. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Map Discovery)
   - Developer B: User Story 2 (Station Details)
   - Developer C: User Story 5 (Login Flow)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- No test tasks included (E2E testing would require Detox and emulator/simulator setup)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

---

## Task Summary

**Total Tasks**: 121
**Total User Stories**: 7 (US1, US2, US3, US4, US5, US6, US7)
**Parallelizable Tasks**: 57 marked with [P]

**Story Breakdown**:
- Setup (Phase 1): 7 tasks
- Foundational (Phase 2): 8 tasks (blocking)
- User Story 1 (P1): 12 tasks
- User Story 2 (P1): 12 tasks
- User Story 3 (P2): 11 tasks
- User Story 4 (P2): 10 tasks
- User Story 5 (P1): 11 tasks
- User Story 6 (P3): 14 tasks (including RTL testing)
- User Story 7 (P3): 12 tasks
- Polish (Phase 10): 17 tasks (including remote logging)

**MVP Scope** (Recommended):
- Phase 1: Setup (7 tasks)
- Phase 2: Foundational (8 tasks)
- Phase 3: User Story 1 (12 tasks)
- Phase 4: User Story 2 (12 tasks)
- Phase 7: User Story 5 (11 tasks)
- **Total**: 50 tasks for MVP

**Expected Development Time** (Solo Developer):
- MVP: 2-3 weeks (50 tasks)
- Full Feature Set: 4-5 weeks (121 tasks)

---

## Next Steps

1. Review and approve the task breakdown
2. Start with Phase 1 (Setup)
3. Follow the priority order for user stories
4. Implement tasks in parallel where marked [P]
5. Test each story independently at checkpoints
6. Commit after completing logical groups
