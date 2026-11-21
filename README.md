# Webhook Aggregator System (wagg / WACDP)
## High-Throughput, Multi-Threaded Event Ingestion, Buffering & Reliable Dispatching Middleware

**Course:** Software Engineering (Mini-Project)  
**Project Code / Topic:** BPS49 — Webhook Aggregator Core Development Pipeline  
**Team Members:**  
- **Sumedh Girish** (SRN: `PES1UG24CS480`)  
- **Subramani B M** (SRN: `PES1UG24CS473`)  
- **Sujan S Halanannavar** (SRN: `PES1UG24CS477`)  
- **Skanda Shyam Nadig** (SRN: `PES1UG24CS458`)  
**Institution:** Department of Computer Science & Engineering, PES University  
**Baseline Version:** 1.0  
**Status:** Sprint 2 Final — Verified & Documented (see [Deviations from Specification](docs/5_Deviations_from_Specification.md))  

---

## Executive Overview & Engineering Specifications

This repository contains the complete, production-grade implementation and rigorous engineering documentation for the **Webhook Aggregator System (`wagg` / `WACDP`)**.

All documents follow formal **IEEE Engineering Standards** (IEEE Std 830, IEEE Std 829, IEEE Std 1016), incorporate agile sprint backlogs from Jira (`SCRUM_SumedhGirish_PES1UG24CS480_BPS49WebhookAggregator.pdf`), and integrate comprehensive system architecture models and empirical performance benchmarks.

### Engineering Specifications Index

| Specification Document | Required Format / Standard | Mandatory Elements Covered | Status | File Location |
| :--- | :--- | :--- | :---: | :--- |
| **1. Software Requirements Specification (SRS)** | **IEEE Std 830-1998** / ISO/IEC/IEEE 29148 | • Introduction, Scope, Perspective, Constraints<br/>• FRs & NFRs using exact specification method<br/>• UML Use Case Diagram + Detailed Specifications<br/>• Security section with ≥ 2 Objectives & ≥ 2 Requirements | **100% Compliant** | [PDF Version (19 pages)](docs/1_Software_Requirements_Specification_SRS.pdf)<br/>[Markdown](docs/1_Software_Requirements_Specification_SRS.md) |
| **2. Software Test Plan (STP)** | **IEEE Std 829-2008** / ISO/IEC/IEEE 29119 | • Test Plan Identifier, Intro & References<br/>• **Section 3** (Test Items & Risk Issues)<br/>• **Section 4** (Features to be Tested / Not Tested)<br/>• **Section 5** (Approach & Strategy)<br/>• **Section 5.1** (Security Validation Strategy)<br/>• Requirements Traceability Matrix (SRS ↔ Test Plan) | **100% Compliant** | [PDF Version (24 pages)](docs/2_Software_Test_Plan_and_Test_Cases.pdf)<br/>[Markdown](docs/2_Software_Test_Plan_and_Test_Cases.md) |
| **3. Software Architecture & Design Specification (SADD)** | **IEEE Std 1016-2009** | • **Architecture:** Component Diagram & 9 Descriptions, Architectural Patterns, Requirements Traceability, Security Architecture<br/>• **Design:** ≥ 2 UML Sequence Diagrams, Multi-Swimlane Activity Diagram, REST API Design, Error Handling & Backoff Math | **100% Compliant** | [PDF Version (17 pages)](docs/3_Software_Architecture_and_Design_Specification_SADD.pdf)<br/>[Markdown](docs/3_Software_Architecture_and_Design_Specification_SADD.md) |
| **4. Test Cases Suite (Test Plan Update)** | IEEE Std 829 Test Case Specifications | • Sufficient coverage of both FRs and NFRs<br/>• Comprehensive suite (Includes **28 fully elaborated test cases** across Functional, Non-Functional, Security, and E2E) | **100% Compliant** | Included in [docs/2_Software_Test_Plan_and_Test_Cases.pdf](docs/2_Software_Test_Plan_and_Test_Cases.pdf) & [Markdown](docs/2_Software_Test_Plan_and_Test_Cases.md#10-comprehensive-test-cases-functional-non-functional--security) |
| **5. Performance Benchmark Datasheet** | System Performance & Verification Report | • Universal Scalability Law (USL) fitting (12.6k req/s)<br/>• Ingestion latency percentiles ($p99 < 15$ ms)<br/>• Little's Law dispatch capacity & memory profiles<br/>• CPU FlameGraphs & database fault recovery | **100% Verified** | [PDF Version (12 pages)](docs/4_Performance_Benchmark_Datasheet_Report.pdf) |
| **6. Deviations from Specification** | Change record against SRS / STP / SADD v1.0 | • 20 deviations with rationale and status<br/>• Updated requirement traceability<br/>• Follow-up actions | **100% Compliant** | [PDF Version](docs/5_Deviations_from_Specification.pdf)<br/>[Markdown](docs/5_Deviations_from_Specification.md) |
| **7. Quality Assurance & Test Execution Reports** | Generated Reports (docs folder) | • Unit Test Execution Report (28 Test Cases)<br/>• Code & Line Coverage Report (86.4% Coverage)<br/>• Branch Coverage Report (82.1% Decision Coverage)<br/>• System Validation & E2E Verification Report | **100% Compliant** | [Unit Test](docs/Unit_Test_Report.md)<br/>[Code Coverage](docs/Code_Coverage_Report.md)<br/>[Branch Coverage](docs/Branch_Coverage_Report.md)<br/>[System Validation](docs/System_Validation_Report.md) |

---

## Directory & File Structure

```text
WebhookAggregator/
├── README.md                                             # System Overview & Architecture Index
├── SCRUM_SumedhGirish_PES1UG24CS480_BPS49WebhookAggregator.pdf # Jira Sprint Backlog & Epics
├── report.pdf                                            # Performance Datasheet (Root Copy)
├── SE_sprint_video.mp4                                   # Combined Sprint Demo Video (Root Copy)
├── Cargo.toml                                            # Rust Package Configuration
├── Cargo.lock                                            # Cargo Dependency Lockfile
├── .github/workflows/
│   ├── ci.yml                                            # Build, Clippy & MariaDB Smoke Test
│   └── pr_automation.yml                                 # PR & Commit Automation
├── src/                                                  # Production Rust Codebase (wagg)
│   ├── main.rs / lib.rs                                  # Entry Point & Library Root
│   ├── listener/                                         # Ingestion Gateway & Signature Verification
│   ├── dispatcher/                                       # Per-Service Claimers, Retry & Backoff
│   └── db/                                               # MariaDB Queue, Config & Dispatch State
├── tests/                                                # Integration Test Suites
│   ├── ingestion_gateway.rs                              # Ingestion & Signature Verification Tests
│   ├── outbound_http_dispatcher.rs                       # Dispatcher & HTTP Sink Tests
│   ├── exponential_backoff_retry_engine.rs               # Backoff & Retry Sequence Tests
│   ├── dead_letter_persistence.rs                        # DLQ Persistence & Crash Recovery Tests
│   └── concurrency_stress_suite.rs                       # High-Concurrency Stress Tests
├── demo/                                                 # Sprint Demo Videos
│   └── SE_sprint_video.mp4                               # Working Product Demo (Sprint 1 + Sprint 2)
└── docs/
    ├── 1_Software_Requirements_Specification_SRS.md      # IEEE 830 SRS Document (Markdown)
    ├── 1_Software_Requirements_Specification_SRS.pdf     # IEEE 830 SRS Document (Presentation PDF)
    ├── 2_Software_Test_Plan_and_Test_Cases.md            # IEEE 829 Test Plan & 28 Test Cases (Markdown)
    ├── 2_Software_Test_Plan_and_Test_Cases.pdf           # IEEE 829 Test Plan & 28 Test Cases (Presentation PDF)
    ├── 3_Software_Architecture_and_Design_Specification_SADD.md # IEEE 1016 SADD Document (Markdown)
    ├── 3_Software_Architecture_and_Design_Specification_SADD.pdf # IEEE 1016 SADD Document (Presentation PDF)
    ├── 4_Performance_Benchmark_Datasheet_Report.pdf      # Performance Datasheet & USL Report (PDF)
    ├── 5_Deviations_from_Specification.md                # Deviations from SRS / STP / SADD (Markdown)
    ├── 5_Deviations_from_Specification.pdf               # Deviations from SRS / STP / SADD (PDF)
    ├── Unit_Test_Report.md                               # Unit Test Execution Results
    ├── Code_Coverage_Report.md                           # Line Coverage Report (86.4%)
    ├── Branch_Coverage_Report.md                         # Branch Coverage Report (82.1%)
    ├── System_Validation_Report.md                       # System Validation Results
    └── assets/
        ├── uml_use_case_diagram.jpeg                     # Webhook Aggregator Use Case Model
        └── activity_workflow_diagram.jpeg                # Webhook Aggregator 5-Swimlane Workflow
```

---

## Agile Planning, Project Board & Sprint Tracking

- **GitHub Project Board / Milestones:**
  - [Sprint 1 Milestone (19th–23rd Oct 2026)](https://github.com/skandanadig/WebhookAggregator/milestone/1)
  - [Sprint 2 Milestone (26th–30th Oct 2026)](https://github.com/skandanadig/WebhookAggregator/milestone/2)
- **Original Scrum / Jira Backlog Export:** [`SCRUM_SumedhGirish_PES1UG24CS480_BPS49WebhookAggregator.pdf`](SCRUM_SumedhGirish_PES1UG24CS480_BPS49WebhookAggregator.pdf)
- **CI / CD Workflows:** GitHub Actions configured for automated PR checks, MariaDB smoke testing, and linting in [`.github/workflows/`](.github/workflows/).

### Sprint Deliverables & Video Demonstrations

| Sprint | Dates | Backlog & Milestones | Scope & Deliverables | Demo Video |
| :--- | :--- | :--- | :--- | :--- |
| **Sprint 1** | 19th–23rd Oct 2026 | [Sprint 1 Milestone](https://github.com/skandanadig/WebhookAggregator/milestone/1) | Ingestion Gateway, Ed25519/HMAC-SHA256 Auth, MariaDB FIFO queue buffering, claimer loop, CI/CD pipeline | [demo/SE_sprint_video.mp4](demo/SE_sprint_video.mp4) *(Part 1)* |
| **Sprint 2** | 26th–30th Oct 2026 | [Sprint 2 Milestone](https://github.com/skandanadig/WebhookAggregator/milestone/2) | Capacity-bounded dispatching, exponential backoff retries, DLQ persistence, stress suite, performance benchmarks | [demo/SE_sprint_video.mp4](demo/SE_sprint_video.mp4) *(Part 2)* |

> **Note on Demo Video:** The product demonstration video [`SE_sprint_video.mp4`](SE_sprint_video.mp4) (also in [`demo/SE_sprint_video.mp4`](demo/SE_sprint_video.mp4)) provides a comprehensive end-to-end walkthrough covering both Sprint 1 (core ingestion, queueing, auth) and Sprint 2 (dispatcher, retry mechanism, dead-letter storage, failure simulation).

Backlog items not delivered in this release (recurring schedules, status/health, and DLQ redrive APIs) remain documented as formal deviations with rationale in [docs/5_Deviations_from_Specification.md](docs/5_Deviations_from_Specification.md).

---

## Project Evaluation Checklist Mapping

This table maps the repository deliverables directly against the **Software Engineering Evaluation Checklist**:

| Evaluation Item | Description & Coverage | Repository Evidence / Artifact |
| :--- | :--- | :--- |
| **Part-1: SRS** | Standard IEEE 830 format, intro/descriptive sections, FRs/NFRs clearly specified, UML Use Case diagram, Security (≥2 objectives & ≥2 requirements). | [SRS (MD)](docs/1_Software_Requirements_Specification_SRS.md) & [SRS (PDF)](docs/1_Software_Requirements_Specification_SRS.pdf) |
| **Part-1: Test Plan** | Standard IEEE 829 format, Sections 3, 4, 5 filled, Section 5.1 security validation, SRS traceability matrix, 28 test cases covering FRs & NFRs. | [Test Plan (MD)](docs/2_Software_Test_Plan_and_Test_Cases.md) & [Test Plan (PDF)](docs/2_Software_Test_Plan_and_Test_Cases.pdf) |
| **Part-1: SAD Specification** | Standard IEEE 1016 format, 9 component descriptions, EDA/Worker Pool patterns, traceability, security arch, 2 UML sequence diagrams, 5-swimlane activity diagram, REST APIs, backoff error protocols. | [SADD (MD)](docs/3_Software_Architecture_and_Design_Specification_SADD.md) & [SADD (PDF)](docs/3_Software_Architecture_and_Design_Specification_SADD.pdf) |
| **Part-2: Agile & Backlog** | GitHub Milestones & Issues created using FRs/NFRs, story points assigned, assignees distributed, Sprint 1 & 2 planned. | [Sprint 1](https://github.com/skandanadig/WebhookAggregator/milestone/1) & [Sprint 2](https://github.com/skandanadig/WebhookAggregator/milestone/2), [Jira Export](SCRUM_SumedhGirish_PES1UG24CS480_BPS49WebhookAggregator.pdf) |
| **Part-2: Sprint 1 Execution** | GitHub Actions CI/PR workflows enabled, code implemented in Rust (`wagg`), PRs raised and reviewed. 1-minute working demo. | [ci.yml](.github/workflows/ci.yml), [pr_automation.yml](.github/workflows/pr_automation.yml), [SE_sprint_video.mp4](SE_sprint_video.mp4) |
| **Part-2: Sprint 2 Execution** | Development activities frozen, formal deviations document ([docs/5_Deviations_from_Specification.md](docs/5_Deviations_from_Specification.md)), 2-minute working demo. | [Deviations (MD)](docs/5_Deviations_from_Specification.md) & [Deviations (PDF)](docs/5_Deviations_from_Specification.pdf), [SE_sprint_video.mp4](SE_sprint_video.mp4) |
| **Part-3: Testing & Reports** | Test cases updated for sprint changes, integration tests executed, test and coverage reports generated. | [Unit Test](docs/Unit_Test_Report.md), [Code Coverage](docs/Code_Coverage_Report.md), [Branch Coverage](docs/Branch_Coverage_Report.md), [System Validation](docs/System_Validation_Report.md) |
| **Part-4: Final Evaluation** | Core use cases fulfilled, deviations documented, clean version control, architecture and performance justified via USL benchmarks. | [Performance Datasheet](docs/4_Performance_Benchmark_Datasheet_Report.pdf), [wagg source](src/) |

---

## Core Specifications Summary

### 1. Software Requirements Specification (SRS) — IEEE Std 830-1998
- **Scope & Perspective:** High-throughput, multi-threaded event ingestion, buffering, and dispatching middleware decoupling upstream microservices from downstream receivers.
- **Exact Method of Specification:**
  - Every functional requirement defines: Requirement ID, Jira Epic mapping, Description, Inputs, Processing Logic, Outputs, Acceptance Criteria, and MoSCoW Priority.
  - Covers 11 Functional Requirements (`FR-01` through `FR-11`) spanning ingestion, non-blocking buffering, task categorization, recurring scheduling, dispatching, backoff retries, and dead-letter queueing.
  - Covers 12 Non-Functional Requirements (`NFR-01` through `NFR-12`) including p99 latency < 15 ms, 5,000 req/s throughput, timer precision $\pm 10$ ms, zero silent data loss, 99.95% availability over 30-day window, backoff sequence fidelity with $\le \pm 100$ ms jitter, queue memory bounding ($\text{RSS} \le 500$ MB), 4-to-64 worker pool elasticity, and ThreadSanitizer-verified thread safety (< 5% contention).
- **UML Use Case Model:**
  - Includes both embedded graphic and detailed specifications for all 7 use cases: *Submit One-Shot Request*, *Configure Recurring Request («extend»)*, *Service Request («include»)*, *Perform Computation*, *Deliver Webhook Payload*, *View Failed Requests*, and *View System & Hook Status*.
- **Security Section:**
  - 3 Formal Security Objectives (`SEC-OBJ-01` to `SEC-OBJ-03`).
  - 4 Formal Security Requirements (`SEC-REQ-01` to `SEC-REQ-04`): HMAC-SHA256 signature verification, Bearer token auth & rate limiting, TLS 1.3 transport security, and JSON-aware recursive secret redaction.

### 2. Software Test Plan & Test Cases — IEEE Std 829-2008
- **Section 3 (Test Items & Risks):** Identifies 8 software modules under test and analyzes 5 core concurrency/network risks with mitigation strategies.
- **Section 4 (Features Under Test):** Delineates features tested vs. out-of-scope boundaries.
- **Section 5 (Approach & Strategy):** Details testing across Unit, Integration, Concurrency (TSan), Load, and Pass/Fail criteria.
- **Section 5.1 (Security Validation):** Dedicated procedures for validating HMAC-SHA256 signatures, replay attack prevention (timestamp tolerances), API token rejections, DoS throttling, and log sanitization.
- **Requirements Traceability Matrix (RTM):** Bidirectional matrix mapping all SRS FRs, NFRs, and Security Requirements directly to test cases with explicit measurement metrics and quantitative acceptance thresholds.
- **Test Cases Suite (28 Comprehensive Cases):**
  - Functional Suite: `TC-FR-01` to `TC-FR-11` (covering boundary 64 KB validation, FIFO buffering, categorizer, recurring timers, keep-alive dispatch, exponential backoff with 6 attempts, DLQ persistence, status queries, redrive).
  - Non-Functional Suite: `TC-NFR-01` to `TC-NFR-12` (latency percentiles, 5k req/s throughput, timer drift, 30-day availability, bounded jitter, RSS $\le 500$ MB, TSan race analysis, modularity, POSIX portability, RFC compliance).
  - Security Suite: `TC-SEC-01` to `TC-SEC-04` (HMAC signatures, rate limits, TLS 1.3 enforcement, JSON-aware recursive secret redaction).
  - Resilience Suite: `TC-E2E-01` (full end-to-end lifecycle under target failure and manual redrive).

### 3. Software Architecture & Design Specification (SADD) — IEEE Std 1016-2009
- **Component Architecture:**
  - Architectural Component Diagram detailing 9 core subsystems: Ingestion Gateway, `service_queue`, Worker Pool Manager, Scheduler & Timers, `dispatch_queue`, Dispatcher Thread, Exponential Backoff Engine, Persistent Storage (DLQ), and Management APIs.
- **Architectural Patterns:**
  - Event-Driven Architecture (EDA) & Reactive Buffering
  - Multi-Queue Producer-Consumer Pattern
  - Worker Thread Pool & Shared Concurrent Task Queue Pattern
  - Exponential Backoff & Circuit Protection Pattern ($T_{\text{wait}} = 2^n$)
  - Dead Letter Channel & Persistent Audit Trail Pattern
- **UML Sequence Diagrams:**
  - **Sequence Diagram 1:** *One-Shot Webhook Ingestion, Buffering, Worker Processing, and Successful Dispatch*
  - **Sequence Diagram 2:** *Scheduled Recurring Execution, Downstream Target Outage, Exponential Backoff Retry Loop, and DLQ Persistence*
- **Activity & Workflow Diagram:**
  - Complete analysis of the 5-swimlane execution flow: `Event Loop`, `Worker Thread Pool`, `Scheduler / Timer`, `Dispatcher Thread`, and `Persistent Storage & Logger`.
- **API Design:**
  - Complete RESTful API specifications with JSON request/response contracts for Ingestion, Recurring Scheduling, Status Query, Failed Queue Inspection, Manual Redrive, and System Health.
- **Error Handling & Data Models:**
  - Comprehensive fault taxonomy, backoff algorithmic formulas, `WebhookTask` with `PayloadReference` memory bounding, `recurring_schedules` persistent SQL schema, and startup recovery flow.

### 4. Performance Benchmark Datasheet Report (wagg Webhook Aggregator)
- **Universal Scalability Law (USL):** System throughput modeled up to $12,639\text{ req/s}$ peak ceiling with $\sigma = 0.057$ serial contention.
- **Latency Percentiles Under Load:** Ingestion latency evaluated via wrk2, k6, and Gatling, verifying $p50 \le 0.95\text{ ms}$ and $p99 \le 12.3\text{ ms}$ at 64 concurrent clients.
- **Little's Law Dispatching:** Empirical validation of $7,470\text{ deliveries/s}$ capacity-bounded fan-out.
- **Fault Injection & Recovery:** 0 stranded and 0 lost events across 113,029 burst requests and database connection cuts.
- **System Footprint & Profiling:** Detailed heap allocation profiles ($12.5\text{ MiB}$ peak heap), CPU FlameGraphs, and InnoDB fsync latency distributions.

---

## Accessing System Documentation

### Option 1: PDF Format (Architectural Documents)
- [1_Software_Requirements_Specification_SRS.pdf](docs/1_Software_Requirements_Specification_SRS.pdf)
- [2_Software_Test_Plan_and_Test_Cases.pdf](docs/2_Software_Test_Plan_and_Test_Cases.pdf)
- [3_Software_Architecture_and_Design_Specification_SADD.pdf](docs/3_Software_Architecture_and_Design_Specification_SADD.pdf)
- [4_Performance_Benchmark_Datasheet_Report.pdf](docs/4_Performance_Benchmark_Datasheet_Report.pdf) *(or root [report.pdf](report.pdf))*
- [5_Deviations_from_Specification.pdf](docs/5_Deviations_from_Specification.pdf)

### Option 2: Markdown Format (GitHub Native)
- [1_Software_Requirements_Specification_SRS.md](docs/1_Software_Requirements_Specification_SRS.md)
- [2_Software_Test_Plan_and_Test_Cases.md](docs/2_Software_Test_Plan_and_Test_Cases.md)
- [3_Software_Architecture_and_Design_Specification_SADD.md](docs/3_Software_Architecture_and_Design_Specification_SADD.md)
- [5_Deviations_from_Specification.md](docs/5_Deviations_from_Specification.md)
- [Unit Test Report](docs/Unit_Test_Report.md)
- [Code Coverage Report](docs/Code_Coverage_Report.md)
- [Branch Coverage Report](docs/Branch_Coverage_Report.md)
- [System Validation Report](docs/System_Validation_Report.md)
