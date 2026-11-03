# Webhook Aggregator System (wagg / WACDP)
## High-Throughput, Multi-Threaded Event Ingestion, Buffering & Reliable Dispatching Middleware

**Project Code / Topic:** BPS49 — Webhook Aggregator Core Development Pipeline  
**Team Members:**  
- **Sumedh Girish** (SRN: `PES1UG24CS480`)  
- **Subramani B M** (SRN: `PES1UG24CS473`)  
- **Sujan S Halanannavar** (SRN: `PES1UG24CS477`)  
- **Skanda Shyam Nadig** (SRN: `PES1UG24CS458`)  
**Institution:** Department of Computer Science & Engineering, PES University  
**Baseline Version:** 1.0  
**Status:** Production-Ready & Verified  

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
| **6. Deviations from Specification** | Change record against SRS / STP / SADD v1.0 | • 20 deviations with rationale and status<br/>• Updated requirement traceability<br/>• Follow-up actions | **Current** | [Markdown](docs/5_Deviations_from_Specification.md) |

---

## Directory & File Structure

```text
WebhookAggregator/
├── README.md                                             # System Overview & Architecture Index
├── SCRUM_SumedhGirish_PES1UG24CS480_BPS49WebhookAggregator.pdf # Jira Sprint Backlog & Epics
├── report.pdf                                            # Performance Datasheet (Root Copy)
├── Cargo.toml                                            # Rust Package Configuration
├── Cargo.lock                                            # Cargo Dependency Lockfile
├── src/                                                  # Production Rust Codebase (wagg)
└── docs/
    ├── 1_Software_Requirements_Specification_SRS.md      # IEEE 830 SRS Document (Markdown)
    ├── 1_Software_Requirements_Specification_SRS.pdf     # IEEE 830 SRS Document (Presentation PDF)
    ├── 2_Software_Test_Plan_and_Test_Cases.md            # IEEE 829 Test Plan & 28 Test Cases (Markdown)
    ├── 2_Software_Test_Plan_and_Test_Cases.pdf           # IEEE 829 Test Plan & 28 Test Cases (Presentation PDF)
    ├── 3_Software_Architecture_and_Design_Specification_SADD.md # IEEE 1016 SADD Document (Markdown)
    ├── 3_Software_Architecture_and_Design_Specification_SADD.pdf # IEEE 1016 SADD Document (Presentation PDF)
    ├── 4_Performance_Benchmark_Datasheet_Report.pdf      # Performance Datasheet & USL Report (PDF)
    └── assets/
        ├── uml_use_case_diagram.jpeg                     # Webhook Aggregator Use Case Model
        └── activity_workflow_diagram.jpeg                # Webhook Aggregator 5-Swimlane Workflow
```

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

### Option 2: Markdown Format (GitHub Native)
- [1_Software_Requirements_Specification_SRS.md](docs/1_Software_Requirements_Specification_SRS.md)
- [2_Software_Test_Plan_and_Test_Cases.md](docs/2_Software_Test_Plan_and_Test_Cases.md)
- [3_Software_Architecture_and_Design_Specification_SADD.md](docs/3_Software_Architecture_and_Design_Specification_SADD.md)
