Testing Strategy (v3.0)
1. Purpose

This document defines the end-to-end testing strategy for the Bornemap platform.

It ensures:

functional correctness
authorization safety
data integrity across services
event-driven reliability
GIS consistency
analytics accuracy
production stability under load

It is the verification layer for the entire architecture, not an implementation detail.

2. Testing Objectives
2.1 Business correctness

Validate core product flows:

station discovery
station detail view
favorites lifecycle
review lifecycle
partner station management
admin moderation workflows
2.2 Authorization correctness (CRITICAL)

Enforce strict validation of:

public vs authenticated access
registered driver permissions
partner tenant isolation
admin global scope
2.3 Data correctness

Validate consistency across:

platform_db (source of truth)
analytics_db (derived state)
GIS projections (derived spatial layer)
2.4 Event correctness

Ensure:

clickstream events are valid
event schema is enforced
deduplication by event_id
RabbitMQ delivery integrity
2.5 System reliability

Validate behavior under:

service restart
queue backpressure
partial DB failure
delayed GIS sync
retry storms
2.6 UX correctness

Ensure:

mobile-first experience works end-to-end
web parity is maintained
RTL Arabic rendering is correct
French LTR consistency
WCAG 2.1 AA compliance
2.7 Performance correctness

Validate:

GIS bbox queries remain bounded
station discovery scales under viewport movement
event ingestion supports sustained load (<100 events/sec baseline)
analytics aggregation does not block ingestion
2.8 Security correctness

Validate:

RBAC enforcement is consistent
tenant isolation cannot be bypassed
input validation resilience
event tampering prevention
API abuse resistance
3. Scope

Testing applies to:

Backend Services
Driver Service
Admin Service
Clickstream Service
GIS Sync Worker
Analytics Writer
Infrastructure
Keycloak
RabbitMQ
PostgreSQL (all DBs)
Frontends
Driver Web App
Driver Mobile App
Partner Dashboard
Admin Dashboard
4. Test Levels

The system uses 6 structured levels:

Unit Tests
Integration Tests
Contract/API Tests
End-to-End (E2E) Tests
Performance Tests
Security Tests

Plus:

Smoke Tests (post-deploy)
Operational Tests (system health validation)
5. Unit Testing Strategy
5.1 Goal

Validate isolated logic with deterministic inputs.

5.2 Backend coverage
Driver Service
station filtering logic (bbox / radius)
is_live + soft delete rules
favorites logic
review ownership enforcement
geo fallback logic (Tunisia center)
Admin Service
partner scoping enforcement
station lifecycle rules
charger lifecycle rules
moderation state transitions
soft delete logic
GIS Worker
outbox event parsing
idempotency logic
retry/backoff rules
state transitions (pending → processed → failed)
Clickstream Service
event schema validation
event_id uniqueness enforcement
anonymous vs authenticated handling
Analytics Writer
deduplication logic
aggregation correctness
partition routing logic
5.3 Frontend unit tests
route protection logic
auth gating behavior
map state transitions
filter state updates
RTL rendering behavior
6. Integration Testing Strategy
6.1 Goal

Validate real interactions between services and infrastructure.

6.2 Backend integration coverage
Driver Service
PostgreSQL station queries
favorites persistence
reviews persistence
authentication enforcement
Admin Service
partner isolation enforcement
CRUD operations correctness
reporting queries
GIS Worker
outbox consumption
retry correctness
idempotent updates
partial failure recovery
Clickstream Service
RabbitMQ publishing
ingestion validation
invalid event rejection
Analytics Writer
queue consumption
DB writes correctness
deduplication behavior
6.3 Database integration

Must use:

real PostgreSQL schema
migrations applied
isolated test datasets
full reset between runs
7. Contract / API Testing
7.1 Goal

Guarantee API stability across:

4 frontend apps
multiple roles
evolving backend services
7.2 Coverage
Driver API
station discovery
station detail
favorites
reviews
/me
Admin API
partner management
station CRUD
moderation endpoints
reporting endpoints
Clickstream API
event ingestion
batch ingestion
error handling
7.3 Contract validation rules

Each endpoint must validate:

response schema
error schema
status codes
auth requirements
pagination structure
filtering correctness
8. End-to-End Testing Strategy
8.1 Goal

Validate full user journeys across system boundaries.

8.2 Critical E2E flows
Public Driver
map load
station browsing
station detail view
Registered Driver
login
favorites lifecycle
review lifecycle
Partner
station creation
charger management
availability updates
tenant isolation enforcement
Admin
partner management
moderation flow
system reporting
System flows
GIS sync lifecycle
clickstream ingestion pipeline
analytics processing pipeline
8.3 E2E categories
A. UI-critical flows
station discovery
map interaction
favorites
reviews
B. System orchestration flows
GIS sync
event pipeline
analytics ingestion
partner lifecycle
9. Performance Testing Strategy
9.1 Goal

Ensure system stability under expected and peak load.

9.2 Coverage
GIS
bbox query latency
map viewport refresh cost
clustering performance
Clickstream
event ingestion throughput
burst handling behavior
queue backpressure resilience
Analytics
ingestion rate stability
aggregation performance
partition write throughput
9.3 Test types
load testing
stress testing
soak testing
regression benchmarking
10. Security Testing Strategy
10.1 Authorization testing (CRITICAL)
public endpoint isolation
partner tenant isolation
admin global access validation
10.2 Input security
SQL injection attempts
malformed JSON payloads
oversized payloads
schema violations
10.3 Event security
event replay attacks
event tampering
duplicate event injection
invalid identity spoofing
10.4 Abuse protection
rate limiting validation
endpoint flooding
authentication brute force resistance
11. GIS Testing Strategy
11.1 Idempotency
duplicate outbox events
replayed events
out-of-order updates
11.2 Failure simulation
worker crash mid-sync
partial updates
retry recovery behavior
11.3 Performance
ensure bbox queries remain indexed
validate spatial query plans
ensure no full-table scans
12. Analytics Testing Strategy
duplicate event ingestion handling
delayed event ordering
partition correctness
aggregation accuracy
13. Localization, RTL & Accessibility Testing
13.1 RTL testing (Arabic)
layout mirroring
icon direction inversion
map overlay correctness
13.2 French (LTR)
layout consistency
text expansion handling
13.3 Accessibility (WCAG 2.1 AA)
keyboard navigation
focus visibility
ARIA correctness
contrast compliance
14. Cross-Browser & Device Testing
Chrome / Firefox / Safari
mobile browsers
tablet layouts
React Native iOS / Android parity
15. CI Testing Requirements
15.1 Backend CI
lint
unit tests
integration tests
contract tests
security tests
performance smoke tests
15.2 Frontend CI
lint
type check
unit tests
build validation
accessibility checks
15.3 CI architecture
isolated DB containers per test suite
isolated RabbitMQ instances
test Keycloak realm
parallel execution per service
16. Pre-Release Gates

A release is blocked unless:

all unit tests pass
all integration tests pass
all contract tests pass
E2E critical flows pass
no performance regression detected
security checks pass
17. Post-Deployment Validation

Mandatory checks:

API health endpoints
authentication flow validation
GIS sync sanity check
clickstream ingestion check
analytics pipeline verification
18. Definition of Done (STRICT)

A feature is NOT complete unless:

unit tests exist
integration coverage exists
authorization tested
data correctness validated
performance impact understood
security implications reviewed
19. Summary

This testing strategy enforces:

strict correctness across distributed services
strong authorization guarantees
reliable event-driven architecture validation
GIS correctness under real-world conditions
analytics integrity and resilience
production-grade stability and observability
