# IT Project Planner Agent — Product Requirements Document (PRD)

## 1. Product Overview

**Name:** IT Project Planner Agent

A person has an IT project idea but doesn't know how to turn that idea into a structured technical plan. The IT Project Planner Agent transforms a rough idea into a structured project blueprint.

### Example

Input:

> "I want to build an AI chatbot for company documents."

Output:

| Section | Content |
| --- | --- |
| Project Name | DocumentAI Assistant |
| Business Problem | Employees need to find information in company documents. |
| Requirements | - Upload documents<br>- Search documents<br>- Ask questions<br>- Generate grounded answers |
| Technology Stack | - Python<br>- FastAPI<br>- PostgreSQL<br>- pgvector<br>- LLM<br>- Docker |
| Implementation Plan | 1. Define requirements<br>2. Build ingestion pipeline<br>3. Build retrieval<br>4. Build generation<br>5. Add evaluation<br>6. Deploy |

## 2. Problem Statement

An IT project idea alone is not actionable. Non-technical stakeholders and engineers alike struggle to move from a vague idea to a concrete plan that names the requirements, the technology stack, and the order of work. This application removes that gap by generating a structured blueprint.

## 3. User Flow

```mermaid
flowchart TD
    U[User] -->|project idea| P[IT Project Planner]
    P --> A[Analyze idea]
    A --> G[Generate blueprint]
    G --> R[Return structured result]
```

The first version is deliberately a single, linear flow: idea in, blueprint out.

## 4. MVP Scope

The MVP is **not** an autonomous super-agent. It is an LLM application that maps a project idea to a project blueprint.

```mermaid
flowchart LR
    Idea[Project Idea] --> LLM[LLM] --> Blueprint[Project Blueprint]
```

The blueprint must contain exactly:

1. **Project name**
2. **Project description**
3. **Business problem**
4. **Functional requirements**
5. **Non-functional requirements**
6. **Technology stack**
7. **Implementation plan**

### First milestone

An even smaller stepping stone comes first:

```mermaid
flowchart TD
    User[User enters project idea]
    App[Python application]
    OR[OpenRouter]
    LLM[LLM]
    Name[Project name]

    User --> App --> OR --> LLM --> Name
```

Example:

- Input: `"Build an AI system for employee knowledge."`
- Output: `"KnowledgeHub AI"`

We build this first, verify it, then expand section by section until the full blueprint is produced.

## 5. What Makes It an Agent (and What Doesn't, Yet)

Initially it is an **LLM application**, not an agent. We build it that way on purpose so the fundamental mechanics are understood before any framework or agentic behavior is layered on.

The intended evolution:

```mermaid
flowchart TD
    L1[LLM Application]
    L2[Structured LLM Application]
    L3[Agent]
    L4[Tool-using Agent]
    L5[Memory-enabled Agent]
    L6[RAG Agent]
    L7[Evaluated Agent]
    L8[Observable Agent]
    L9[Production Agent]

    L1 --> L2 --> L3 --> L4 --> L5 --> L6 --> L7 --> L8 --> L9
```

## 6. Architecture

### Phase 1 architecture

```mermaid
flowchart TD
    U[User] --> Idea[Project Idea]
    Idea --> P[Project Planner]
    P --> M[LLM]
    M --> B[Project Blueprint]
```

Not built yet, but where we are heading:

```mermaid
flowchart TD
    U[User] --> API[API]
    API --> AR[Agent Runtime]

    AR --> LLM[LLM]
    AR --> Tools[Tools]
    AR --> Mem[Memory]

    Tools --> RAG[RAG]
    Tools --> Web[Web]
    Tools --> Files[Files]
    Mem --> DB[(DB)]

    LLM --> G[Guardrails]
    Tools --> G
    Mem --> G

    G --> R[Response]
    R --> E[Evaluation]
    R --> O[Observability]

    classDef tool fill:#f0f4ff,stroke:#4a7,color:#123;
    class RAG,Web,Files,DB tool;
```

## 7. Technology Decisions

| Component | Technology | When |
| --- | --- | --- |
| Language | Python | Now |
| Package manager | uv | Now |
| Validation | Pydantic | Now |
| LLM | OpenRouter | Now |
| HTTP | httpx | Now |
| Testing | pytest | Now |
| Configuration | YAML + .env | Now |
| UI | TBD | Later |
| API | TBD | Later |
| Database | TBD | Later |
| RAG | rag-system | Later |
| Agent framework | None initially | Later |

### Key decision: no agent framework at the start

We implement the fundamental mechanics ourselves. This is how we will understand what agent frameworks actually do when we eventually use one.

## 8. Success Criteria

- [x] Idea in → structured blueprint out, with all seven required sections
- [x] First milestone reachable: idea in → project name out (as a `ProjectPlanner` service)
- [x] The system validates output structure with Pydantic (`ProjectName`; full `ProjectBlueprint` later)
- [x] The system is testable with pytest
- [x] Everything runs with `uv` and configuration from YAML + `.env`