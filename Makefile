# Makefile - Trust Platform (Klippa Rust Microservices)

# Configuration
CARGO := cargo
PROJECT := trust-platform
SERVICES := identity trust catalog order escrow shipment dispute notification messaging risk evidence analytics gateway
LIBS := common db config telemetry messaging domain error auth payment logistics
PROTO_DIR := proto

# Colors for pretty output
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
BLUE := \033[0;34m
NC := \033[0m # No Color

.PHONY: help all dev deps build test fmt clippy clean proto docker compose

## Help - show available commands
help:
	@echo "$(GREEN)╔════════════════════════════════════════════════════════════╗$(NC)"
	@echo "$(GREEN)║             Trust Platform Microservices                    ║$(NC)"
	@echo "$(GREEN)╚════════════════════════════════════════════════════════════╝$(NC)"
	@echo ""
	@echo "$(YELLOW)Development Environment:$(NC)"
	@echo "  make dev         - Start complete development environment"
	@echo "  make deps        - Start dependencies (DB, Redis, Kafka)"
	@echo "  make compose     - Start with docker-compose"
	@echo ""
	@echo "$(YELLOW)Building & Compilation:$(NC)"
	@echo "  make build       - Build all services and libraries"
	@echo "  make build-release - Build optimized release"
	@echo "  make check       - Quick compilation check (all services)"
	@echo "  make check-SERVICE - Check specific service"
	@echo ""
	@echo "$(YELLOW)Code Quality & Testing:$(NC)"
	@echo "  make test        - Run all tests"
	@echo "  make test-e2e    - Run end-to-end tests"
	@echo "  make test-contract - Run contract tests"
	@echo "  make fmt         - Format all code"
	@echo "  make clippy      - Run clippy linter"
	@echo "  make quality     - Run all quality checks (fmt + clippy + test)"
	@echo ""
	@echo "$(YELLOW)Service Management:$(NC)"
	@echo "  make run-SERVICE - Run specific service"
	@echo "  make run-all     - Run all services"
	@echo "  make stop        - Stop all running services"
	@echo ""
	@echo "$(YELLOW)Protobuf & Code Generation:$(NC)"
	@echo "  make proto       - Generate gRPC code from .proto files"
	@echo "  make proto-watch - Watch for .proto changes and regenerate"
	@echo ""
	@echo "$(YELLOW)Database:$(NC)"
	@echo "  make migrate     - Run all database migrations"
	@echo "  make migrate-SERVICE - Run migrations for specific service"
	@echo "  make db-reset    - Reset all databases (development only)"
	@echo ""
	@echo "$(YELLOW)Monitoring & Observability:$(NC)"
	@echo "  make monitor     - Start monitoring stack (Prometheus + Grafana)"
	@echo "  make logs        - View service logs"
	@echo ""
	@echo "$(YELLOW)Cleanup:$(NC)"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make nuke        - Clean everything including dependencies"
	@echo ""
	@echo "$(BLUE)Available Services:$(NC)"
	@for svc in $(SERVICES); do \
		echo "  • $$svc"; \
	done

## Development Environment
all: deps proto build
	@echo "$(GREEN)✅ Complete environment ready$(NC)"

dev: deps proto build
	@echo "$(GREEN)🚀 Starting development environment...$(NC)"
	@make run-all

deps:
	@echo "$(GREEN)📦 Starting dependencies...$(NC)"
	docker-compose -f ops/docker-compose.yml up -d postgres redis kafka zookeeper

compose:
	@echo "$(GREEN)🐳 Starting with docker-compose...$(NC)"
	docker-compose -f ops/docker-compose.yml up --build

## Building
build:
	@echo "$(GREEN)🔨 Building workspace...$(NC)"
	$(CARGO) build --workspace

build-release:
	@echo "$(GREEN)🔨 Building for release...$(NC)"
	$(CARGO) build --workspace --release

## Check Commands
check:
	@echo "$(GREEN)✅ Checking all code...$(NC)"
	$(CARGO) check --workspace

# Generate check commands for all services dynamically
define CHECK_TEMPLATE
check-$(1):
	@echo "$(GREEN)✅ Checking $(1) service...$(NC)"
	$(CARGO) check -p $(1)
endef

$(foreach svc,$(SERVICES),$(eval $(call CHECK_TEMPLATE,$(svc))))

## Testing
test:
	@echo "$(GREEN)🧪 Running all tests...$(NC)"
	$(CARGO) test --workspace

test-e2e:
	@echo "$(GREEN)🧪 Running end-to-end tests...$(NC)"
	$(CARGO) test -p tests-e2e

test-contract:
	@echo "$(GREEN)🧪 Running contract tests...$(NC)"
	$(CARGO) test -p tests-contract

test-smoke:
	@echo "$(GREEN)🧪 Running smoke tests...$(NC)"
	$(CARGO) test -p tests-smoke

test-load:
	@echo "$(GREEN)📊 Running load tests...$(NC)"
	cargo run -p tests-performance -- load

test-stress:
	@echo "$(GREEN)📊 Running stress tests...$(NC)"
	cargo run -p tests-performance -- stress

test-watch:
	@echo "$(GREEN)👀 Running tests in watch mode...$(NC)"
	$(CARGO) watch -x test

## Code Quality
fmt:
	@echo "$(GREEN)🎨 Formatting code...$(NC)"
	$(CARGO) fmt --all

clippy:
	@echo "$(GREEN)🔍 Running clippy...$(NC)"
	$(CARGO) clippy --workspace -- -D warnings

audit:
	@echo "$(GREEN)🔒 Security audit...$(NC)"
	$(CARGO) audit

quality: fmt clippy test
	@echo "$(GREEN)✨ All quality checks passed!$(NC)"

## Protobuf Code Generation
proto:
	@echo "$(GREEN)📦 Generating gRPC code from .proto files...$(NC)"
	$(CARGO) build -p proto
	@# Generate for each service that has proto definitions
	@for proto_file in $(PROTO_DIR)/*.proto; do \
		if [ -f "$$proto_file" ]; then \
			echo "Generating for $$(basename $$proto_file)"; \
		fi \
	done

proto-watch:
	@echo "$(GREEN)👀 Watching for .proto changes...$(NC)"
	@while true; do \
		inotifywait -e modify $(PROTO_DIR)/*.proto; \
		make proto; \
	done

## Service Management
# Generate run commands for all services dynamically
define RUN_TEMPLATE
run-$(1):
	@echo "$(GREEN)🚀 Starting $(1) service...$(NC)"
	$(CARGO) run -p $(1)
endef

$(foreach svc,$(SERVICES),$(eval $(call RUN_TEMPLATE,$(svc))))

run-all:
	@echo "$(GREEN)🚀 Starting all services...$(NC)"
	@for service in $(SERVICES); do \
		echo "$(BLUE)▶ Starting $$service...$(NC)"; \
		$(CARGO) run -p $$service & \
	done
	@echo "$(GREEN)✅ All services started!$(NC)"
	@echo "$(YELLOW)Services running in background.$(NC)"
	@echo "$(YELLOW)To stop: 'make stop' or 'pkill -f \"cargo run\"'$(NC)"
	@echo "$(YELLOW)To see output, check each service's terminal$(NC)"
	@wait  # Keeps the make command running

stop:
	@echo "$(RED)🛑 Stopping all services...$(NC)"
	@pkill -f "cargo run" 2>/dev/null || true
	@echo "$(GREEN)✅ All services stopped.$(NC)"

## Database
migrate:
	@echo "$(GREEN)🗃️ Running all migrations...$(NC)"
	@for service in $(SERVICES); do \
		if [ -f "services/$$service/migrations" ] || [ -d "services/$$service/migrations" ]; then \
			echo "Migrating $$service..."; \
			$(CARGO) run -p $$service -- migrate 2>/dev/null || echo "No migrate command for $$service"; \
		fi \
	done

# Generate migrate commands for all services dynamically
define MIGRATE_TEMPLATE
migrate-$(1):
	@echo "$(GREEN)🗃️ Running migrations for $(1)...$(NC)"
	@if [ -f "services/$(1)/migrations" ] || [ -d "services/$(1)/migrations" ]; then \
		$(CARGO) run -p $(1) -- migrate; \
	else \
		echo "No migrations found for $(1)"; \
	fi
endef

$(foreach svc,$(SERVICES),$(eval $(call MIGRATE_TEMPLATE,$(svc))))

db-reset:
	@echo "$(RED)⚠️  Resetting all databases (development only)...$(NC)"
	@read -p "Are you sure? This will delete all data. (y/N): " confirm; \
	if [ "$$confirm" = "y" ] || [ "$$confirm" = "Y" ]; then \
		docker-compose -f ops/docker-compose.yml down -v; \
		docker-compose -f ops/docker-compose.yml up -d postgres; \
		sleep 5; \
		make migrate; \
		echo "$(GREEN)✅ Databases reset complete.$(NC)"; \
	else \
		echo "$(YELLOW)❌ Operation cancelled.$(NC)"; \
	fi

## Monitoring & Observability
monitor:
	@echo "$(GREEN)📊 Starting monitoring stack...$(NC)"
	docker-compose -f ops/docker-compose.yml up -d prometheus grafana

logs:
	@echo "$(GREEN)📋 Service Logs:$(NC)"
	@if [ -d "logs" ]; then \
		for logfile in logs/*.log; do \
			service=$$(basename $$logfile .log); \
			echo "$(YELLOW)=== $$service ===$(NC)"; \
			tail -20 $$logfile 2>/dev/null || echo "No logs yet"; \
			echo ""; \
		done; \
	else \
		echo "No logs available. Run services first."; \
	fi

logs-follow:
	@echo "$(GREEN)📋 Following service logs (Ctrl+C to stop)...$(NC)"
	@if [ -d "logs" ]; then \
		tail -f logs/*.log; \
	else \
		echo "No logs available. Run services first."; \
	fi

logs-$(1):
	@echo "$(GREEN)📋 Following $(1) logs...$(NC)"
	@if [ -f "logs/$(1).log" ]; then \
		tail -f logs/$(1).log; \
	else \
		echo "No logs available for $(1)."; \
	fi

## Documentation
docs:
	@echo "$(GREEN)📚 Building documentation...$(NC)"
	$(CARGO) doc --workspace --no-deps --open

docs-internal:
	@echo "$(GREEN)📚 Building internal documentation...$(NC)"
	$(CARGO) doc --workspace --document-private-items --no-deps

## Cleanup
clean:
	@echo "$(GREEN)🧹 Cleaning build artifacts...$(NC)"
	$(CARGO) clean
	@rm -rf logs pids target

nuke: clean
	@echo "$(RED)💥 Stopping and removing all dependencies...$(NC)"
	docker-compose -f ops/docker-compose.yml down -v --remove-orphans
	@docker system prune -f

## Docker Operations
docker-build:
	@echo "$(GREEN)🐳 Building Docker images...$(NC)"
	@for service in $(SERVICES); do \
		if [ -f "services/$$service/Dockerfile" ]; then \
			echo "Building $$service..."; \
			docker build -t trust-platform/$$service:latest services/$$service; \
		fi \
	done

docker-push:
	@echo "$(GREEN)📤 Pushing Docker images...$(NC)"
	@for service in $(SERVICES); do \
		if [ -f "services/$$service/Dockerfile" ]; then \
			echo "Pushing $$service..."; \
			docker push trust-platform/$$service:latest; \
		fi \
	done

## Kubernetes
k8s-apply:
	@echo "$(GREEN)☸️  Applying Kubernetes manifests...$(NC)"
	kubectl apply -f ops/k8s/

k8s-delete:
	@echo "$(RED)☸️  Deleting Kubernetes resources...$(NC)"
	kubectl delete -f ops/k8s/

## Backup & Restore
backup:
	@echo "$(GREEN)💾 Creating backup...$(NC)"
	@timestamp=$$(date +%Y%m%d_%H%M%S); \
	mkdir -p backups/$$timestamp; \
	docker-compose -f ops/docker-compose.yml exec -T postgres pg_dumpall -U postgres > backups/$$timestamp/full_backup.sql; \
	echo "$(GREEN)✅ Backup saved to backups/$$timestamp/$(NC)"

restore:
	@echo "$(YELLOW)💾 Restoring from backup...$(NC)"
	@latest=$$(ls -td backups/*/ | head -1); \
	if [ -z "$$latest" ]; then \
		echo "$(RED)No backups found$(NC)"; \
		exit 1; \
	fi; \
	echo "Restoring from $$latest"; \
	docker-compose -f ops/docker-compose.yml exec -T postgres psql -U postgres -f /backup.sql < $$latest/full_backup.sql; \
	echo "$(GREEN)✅ Restore complete$(NC)"

## Setup directories
setup:
	@echo "$(GREEN)📁 Setting up directories...$(NC)"
	@mkdir -p logs pids backups
	@echo "$(GREEN)✅ Directory structure ready$(NC)"

## Health Check
health:
	@echo "$(GREEN)🏥 Health Check:$(NC)"
	@echo "1. Checking cargo..."
	@which cargo > /dev/null && echo "$(GREEN)  ✓ Cargo installed$(NC)" || echo "$(RED)  ✗ Cargo not found$(NC)"
	@echo "2. Checking docker..."
	@which docker > /dev/null && echo "$(GREEN)  ✓ Docker installed$(NC)" || echo "$(RED)  ✗ Docker not found$(NC)"
	@echo "3. Checking docker-compose..."
	@which docker-compose > /dev/null && echo "$(GREEN)  ✓ Docker Compose installed$(NC)" || echo "$(RED)  ✗ Docker Compose not found$(NC)"
	@echo "4. Checking required ports..."
	@for port in 5432 6379 9090 3000; do \
		netstat -tuln | grep -q ":$$port " && echo "$(RED)  ✗ Port $$port in use$(NC)" || echo "$(GREEN)  ✓ Port $$port available$(NC)"; \
	done
	@echo "$(GREEN)✅ Health check complete$(NC)"

## Quick Start for New Developers
onboard:
	@echo "$(GREEN)👋 Welcome to Trust Platform!$(NC)"
	@echo ""
	@echo "Setting up your development environment..."
	@echo ""
	@make health
	@echo ""
	@echo "1. Installing dependencies..."
	make deps
	@echo ""
	@echo "2. Generating protocol buffers..."
	make proto
	@echo ""
	@echo "3. Building workspace..."
	make build
	@echo ""
	@echo "4. Running migrations..."
	make migrate
	@echo ""
	@echo "$(GREEN)✅ Setup complete!$(NC)"
	@echo ""
	@echo "$(YELLOW)Next steps:$(NC)"
	@echo "  • Run 'make dev' to start all services"
	@echo "  • Run 'make test' to run tests"
	@echo "  • Run 'make logs' to view service logs"
	@echo "  • Visit http://localhost:3000 for the API Gateway"