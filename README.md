# 🏛️ TrustFlow – Infrastructure for Trustworthy Trade

**_We make trade trustworthy and seamless._**

> A high-performance, type-safe microservices platform built in Rust that powers the complete trade infrastructure. TrustFlow enables secure trade through escrow management, shipment tracking, dispute resolution, and trust scoring.

---

## 🎯 Our Mission & Brand Promise

| **Aspect**             | **Statement**                                                                                                                  |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| **Brand Name**         | TrustFlow                                                                                                                      |
| **Brand Promise**      | _We make trade trustworthy and seamless._                                                                                      |
| **Primary Tagline**    | _We don't sell goods. We make sure sellers and buyers can trust each other._                                                   |
| **Formal Description** | An infrastructure platform that enables secure trade through escrow, shipment tracking, dispute resolution, and trust scoring. |

### 💬 Positioning by Stakeholder

- **For Sellers:** "Guaranteed payment once your buyer confirms."
- **For Buyers:** "Your money is safe until you're satisfied."
- **For Logistics Partners:** "Digital orchestration that drives more jobs."

---

## 🏗️ Architecture Overview

This repository is a **modular microservices platform** built as a Rust monorepo. Each service and library operates as an independent Cargo package under unified build, test, and deployment orchestration.

### 🎯 Core Services (Microservices)

| Service            | Purpose                                                            | Key Technologies         |
| ------------------ | ------------------------------------------------------------------ | ------------------------ |
| **`identity`**     | User authentication, authorization & profile management            | Axum, JWT, SQLx          |
| **`trust`**        | Trust scoring, reputation management & scoring algorithms          | Tokio, PostgreSQL        |
| **`catalog`**      | Product/service catalog, search, and listings                      | Axum, SQLx               |
| **`order`**        | Order creation, lifecycle management, and orchestration            | Axum, SQLx, Tonic        |
| **`escrow`**       | Secure payment holding, release logic & fund management            | Tokio, PostgreSQL, gRPC  |
| **`shipment`**     | Shipment tracking, logistics integration & real-time updates       | Axum, Kafka, SQLx        |
| **`dispute`**      | Dispute resolution workflows, evidence management & decision logic | Axum, SQLx               |
| **`evidence`**     | Evidence collection, storage & validation for disputes             | S3, PostgreSQL           |
| **`notification`** | Multi-channel notifications (email, SMS, push, webhooks)           | Tokio, Redis             |
| **`messaging`**    | Real-time messaging, chat & communication                          | WebSocket, Redis Pub/Sub |
| **`risk`**         | Risk assessment, fraud detection & anomaly scoring                 | ML models, PostgreSQL    |
| **`analytics`**    | Metrics, insights, and platform analytics                          | Kafka, TimescaleDB       |
| **`gateway`**      | API Gateway, routing, rate limiting & request orchestration        | Axum, Redis              |

### 📚 Shared Libraries

| Library         | Purpose                                      |
| --------------- | -------------------------------------------- |
| **`common`**    | Shared utilities, types, and helpers         |
| **`db`**        | Database connections, migrations & pooling   |
| **`config`**    | Configuration management & environment setup |
| **`telemetry`** | Logging, metrics, tracing & observability    |
| **`messaging`** | Event bus abstractions & async communication |
| **`domain`**    | Shared domain models & business logic        |
| **`error`**     | Unified error handling & context propagation |
| **`auth`**      | Authentication/authorization utilities       |
| **`payment`**   | Payment processing abstractions              |
| **`logistics`** | Logistics integration utilities              |

## 🚀 Quick Start

### Prerequisites

- 🦀 **Rust 1.81+** (managed via `rust-toolchain.toml`)
- 🐋 **Docker & Docker Compose** (for dependencies: PostgreSQL, Redis, Kafka)
- 🧩 **Make** (for development workflows)
- 📦 **Protocol Buffers compiler** (for gRPC code generation)

### Development Setup

```bash
# 1. Clone the repository
git clone <repository>
cd trustflow-rust

# 2. Verify Rust toolchain (auto-managed)
rustup show

# 3. Copy environment template
cp ops/.env.example .env

# 4. Start local dependencies (PostgreSQL, Redis, Kafka, etc.)
docker-compose -f ops/docker-compose.yml up -d

# 5. Run database migrations
make db-migrate

# 6. Build all services and libraries
make build

# 7. Run tests to verify setup
make test
```

### Running Services

```bash
# Run all services
make run-all

# Run specific service
make run-order
make run-escrow
make run-shipment
make run-identity

# Stop all services
make stop
```

### Development Commands

```bash
# Code quality checks
make fmt      # Format code
make clippy   # Lint & analyze
make quality  # All quality checks

# Testing
make test           # Run all tests
make test-e2e       # End-to-end tests
make test-contract  # Service contract tests
make test-perf      # Performance tests

# Building
make build          # Build all (debug)
make build-release  # Optimized build
make proto          # Generate gRPC code

# Useful utilities
make help           # Show all available commands
make clean          # Clean build artifacts
make check          # Quick compilation check
```

## 📁 Project Structure

```
trustflow-rust/
├── 🏠 Root Configuration
│   ├── Cargo.toml              # Workspace manifest (all services & libs)
│   ├── rust-toolchain.toml     # Pinned Rust version (1.81+)
│   ├── Makefile                # Development commands & workflows
│   └── Cargo.lock              # Lock file for reproducible builds
│
├── 🔧 Microservices (services/)
│   ├── identity/               # Authentication & user management
│   ├── trust/                  # Trust scoring & reputation
│   ├── catalog/                # Product/service listings
│   ├── order/                  # Order lifecycle management
│   ├── escrow/                 # Payment escrow & fund management
│   ├── shipment/               # Logistics tracking & orchestration
│   ├── dispute/                # Dispute resolution workflows
│   ├── evidence/               # Evidence storage & validation
│   ├── notification/           # Multi-channel notifications
│   ├── messaging/              # Real-time messaging & chat
│   ├── risk/                   # Risk assessment & fraud detection
│   ├── analytics/              # Platform metrics & insights
│   └── gateway/                # API gateway & routing
│
├── 📚 Shared Libraries (libs/)
│   ├── common/                 # Shared utilities & types
│   ├── db/                     # Database layer & migrations
│   ├── config/                 # Configuration management
│   ├── telemetry/              # Logging, metrics & tracing
│   ├── messaging/              # Event bus & async comms
│   ├── domain/                 # Business domain models
│   ├── error/                  # Error handling framework
│   ├── auth/                   # Auth utilities
│   ├── payment/                # Payment abstractions
│   └── logistics/              # Logistics integrations
│
├── 🚀 Operations (ops/)
│   ├── docker-compose.yml      # Local dev environment
│   ├── prometheus/             # Metrics collection
│   ├── grafana/                # Monitoring dashboards
│   └── .env.example            # Environment template
│
├── 📡 API Contracts (proto/)
│   ├── *.proto                 # gRPC service definitions
│   └── build.rs                # Protobuf code generation
│
├── 📖 Documentation (docs/)
│   ├── adr/                    # Architecture Decision Records
│   ├── api/                    # API documentation
│   └── runbooks/               # Operational procedures
│
├── 📋 Scripts (scripts/)
│   ├── build_all.sh            # Build all services
│   ├── test_all.sh             # Run all tests
│   ├── lint_all.sh             # Code quality checks
│   └── db_migrate.sh           # Database migrations
│
└── 🧪 Testing (tests/)
    ├── contract/               # Service contract tests
    ├── e2e/                    # End-to-end integration tests
    ├── performance/            # Load & stress testing
    └── smoke/                  # Health checks & smoke tests
```

## ⚙️ Configuration Management

### Environment Configuration

Services use a hierarchical configuration system:

```
config/
├── base.yaml           # Common settings (all environments)
├── development.yaml    # Development overrides
├── production.yaml     # Production settings
└── .env                # Local secrets (gitignored)
```

### Configuration Priority (Highest to Lowest)

1. **Environment variables** (`DATABASE_URL`, `REDIS_URL`, `JWT_SECRET`, etc.)
2. **Environment-specific YAML** (`development.yaml`, `production.yaml`)
3. **Base configuration** (`base.yaml`)

### Setup Example (.env)

```bash
# Database
DATABASE_URL=postgres://user:password@localhost:5432/trustflow_dev
DATABASE_POOL_SIZE=20

# Redis
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=

# Kafka
KAFKA_BROKERS=localhost:9092
KAFKA_GROUP_ID=trustflow-dev

# Observability
LOG_LEVEL=debug
OTEL_ENDPOINT=http://localhost:4317

# Security
JWT_SECRET=your_super_secret_jwt_key_here
ENCRYPTION_KEY=your_encryption_key

# Services
IDENTITY_SERVICE_URL=http://localhost:3001
ORDER_SERVICE_URL=http://localhost:3002
ESCROW_SERVICE_URL=http://localhost:3003
```

---

## 🏛️ Service Overview

### Core Trade Flow

```
User Registration
    ↓
identity service → Creates account, authentication
    ↓
trust service → Establishes initial trust score
    ↓
Product Listing
    ↓
catalog service → Lists products/services
    ↓
Order Creation
    ↓
order service → Initiates order, coordinates workflow
    ↓
Secure Payment
    ↓
escrow service → Holds funds securely
    ↓
Shipment
    ↓
shipment service → Tracks logistics, real-time updates
    ↓
Delivery & Verification
    ↓
evidence service → Collects proof of delivery
    ↓
Payment Release
    ↓
escrow service → Releases funds to seller
    ↓
Trust Update
    ↓
trust service → Updates trust scores based on transaction
```

### Service Descriptions

**Identity Service** – User authentication, JWT tokens, OAuth integration

**Trust Service** – Reputation scoring, trust metrics, behavioral analysis

**Catalog Service** – Product/service listings, search, categories

**Order Service** – Order lifecycle, state machine, orchestration

**Escrow Service** – Secure fund holding, smart escrow logic, payment release

**Shipment Service** – Real-time tracking, logistics integration, EDI

**Dispute Service** – Conflict resolution, escalation workflows, arbitration

**Evidence Service** – Proof collection, storage, validation

**Notification Service** – Email, SMS, push, webhooks, in-app notifications

**Messaging Service** – Real-time chat, file sharing, communication

**Risk Service** – Fraud detection, anomaly scoring, ML-based assessment

**Analytics Service** – Metrics, insights, reporting, dashboards

**Gateway Service** – API gateway, routing, rate limiting, request validation

---

## 📊 Technology Stack

| Layer              | Technology                                   |
| ------------------ | -------------------------------------------- |
| **Framework**      | Axum (async web), Tonic (gRPC)               |
| **Runtime**        | Tokio (async runtime)                        |
| **Database**       | PostgreSQL + SQLx (compile-time checked SQL) |
| **Caching**        | Redis (sessions, cache, pub/sub)             |
| **Message Queue**  | Kafka (event streaming, log aggregation)     |
| **RPC**            | gRPC + Protocol Buffers                      |
| **Serialization**  | serde + serde_json                           |
| **Error Handling** | thiserror (ergonomic errors)                 |
| **Observability**  | OpenTelemetry, Prometheus, Grafana, Jaeger   |
| **Testing**        | tokio-test, proptest, mockito                |
| **Code Quality**   | Clippy, rustfmt, cargo-audit                 |

---

## 🔐 Security Considerations

✅ **JWT Authentication** – Secure token-based auth across services

✅ **Escrow Smart Logic** – Prevents fund misuse through business rules

✅ **Dispute Arbitration** – Fair resolution mechanisms with evidence

✅ **Encryption** – TLS/SSL for transit, encrypted at rest for sensitive data

✅ **Rate Limiting** – Protection against abuse via gateway service

✅ **Input Validation** – Schema validation on all API endpoints

✅ **Audit Logging** – Immutable transaction logs for compliance

✅ **Trust Scoring** – Behavioral analysis to detect fraudulent actors

---

## 📈 Monitoring & Observability

All services export metrics to **Prometheus** and logs to **OpenTelemetry**.

### View Dashboards

```bash
# Access Grafana
open http://localhost:3000
# Default: admin / admin

# Access Prometheus
open http://localhost:9090

# Access Jaeger (Distributed Tracing)
open http://localhost:6831
```

---

## 🛠️ Development Workflow

### Common Tasks

```bash
# Build everything
make build

# Run tests
make test

# Start specific service
make run-order
make run-escrow
make run-shipment
make run-identity

# Code quality checks
make fmt          # Format code
make clippy       # Lint code
make quality      # Run all quality checks

# Clean up
make clean        # Remove build artifacts
make nuke         # Reset everything including dependencies
```

### Service Development

```bash
# Work on a specific service
cargo check -p booking
cargo test -p payment
cargo run -p chat

# Add dependencies to workspace
# Edit [workspace.dependencies] in root Cargo.toml
# Then in service: cargo add tokio --workspace
```

### Database Operations

```bash
# Start dependencies (PostgreSQL, Redis, Kafka)
make deps

# Run migrations
make migrate
```

---

## 🔍 Observability & Monitoring

### Local Development

```bash
# Start monitoring stack
docker-compose -f ops/docker-compose.yml up prometheus grafana

# Access dashboards
# Prometheus: http://localhost:9090
# Grafana:    http://localhost:3000
```

### Metrics & Logging

All services automatically:

- Export Prometheus metrics at `/metrics`
- Output structured JSON logs
- Support distributed tracing with OpenTelemetry
- Include health checks at `/health`

---

## 🐳 Deployment

### Local Development

```bash
# Full local setup
make dev

# Or step by step
make deps        # Start dependencies
make build       # Build services
make run-all     # Start all services
```

### Production Build

```bash
# Build optimized binaries
make build-release

```

---

## 🧪 Testing Strategy

| Test Type             | Location             | Purpose                      |
| --------------------- | -------------------- | ---------------------------- |
| **Unit Tests**        | `services/*/src/`    | Individual function testing  |
| **Integration Tests** | `services/*/tests/`  | Service + dependencies       |
| **Contract Tests**    | `tests/contract/`    | Service API compatibility    |
| **End-to-End Tests**  | `tests/e2e/`         | Full user journey validation |
| **Performance Tests** | `tests/performance/` | Load and stress testing      |
| **Benchmarks**        | `benches/`           | Code-level performance       |

Run all tests: `make test`

---

## 🛡️ Security & Compliance

### Security Features

- **Memory Safety**: All crates use `#![forbid(unsafe_code)]`
- **Dependency Scanning**: Regular `cargo audit` runs
- **Secure Defaults**: All services follow security best practices
- **Compliance**: NDPR/GDPR compliant audit trails
- **Secrets Management**: Environment variables for sensitive data

### Security Scanning

```bash
# Audit dependencies
cargo audit

# Check for vulnerable dependencies
cargo deny check advisories

# Security-focused linting
cargo clippy -- -D security
```

---

## 🔄 CI/CD Pipeline

The GitHub Actions workflow in `.github/workflows/` provides:

- **Continuous Integration**:
  - Format checking (`cargo fmt --check`)
  - Linting (`cargo clippy`)
  - Unit and integration tests
  - Security scanning (`cargo audit`)

- **Continuous Deployment**:
  - Docker image building and publishing
  - Kubernetes deployment manifests
  - Environment-specific configurations

---

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines:

1. **Fork & Branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Develop & Test**

   ```bash
   make quality  # Run all quality checks
   make test     # Ensure all tests pass
   ```

3. **Submit PR**
   - Ensure CI passes
   - Update documentation as needed
   - Follow conventional commits

### Code Standards

- **Formatting**: `cargo fmt` before committing
- **Linting**: No `cargo clippy` warnings
- **Testing**: Maintain high test coverage
- **Documentation**: Update relevant README files

---

## 📚 Documentation

| Resource                   | Purpose                       | Location               |
| -------------------------- | ----------------------------- | ---------------------- |
| **Service Documentation**  | Service-specific setup & APIs | `services/*/README.md` |
| **Architecture Decisions** | Technical decision records    | `docs/adr/`            |
| **API Reference**          | Service API documentation     | `docs/api/`            |
| **Runbooks**               | Operational procedures        | `docs/runbooks/`       |

---

## 🏢 Related Projects

| Repository             | Purpose                     |
| ---------------------- | --------------------------- |
| **trustflow-frontend** | React-based web application |
| **trustflow-mobile**   | Mobile client applications  |

---

## 📄 License

© 2024 TrustFlow Inc. – Proprietary and confidential. All rights reserved.

_For internal use only._

---

> 🦀 **Built with Rust for safety, performance, and reliability at scale.**
>
> _"If it compiles, it works."_
