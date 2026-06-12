# jankurai Repo Score

- Standard: `jankurai`
- Auditor: `1.6.1`
- Schema: `1.9.0`
- Paper edition: `2026.05-ed8`
- Target stack ID: `rust-ts-vite-react-postgres-bounded-python`
- Target stack: `Rust core + TypeScript/React/Vite + PostgreSQL + generated contracts + exception-only Python AI/data service`
- Repo: `.`
- Run ID: `1781259185`
- Started at: `1781259185`
- Elapsed: `3603` ms
- Scope: `full`
- Raw score: `86`
- Final score: `86`
- Decision: `advisory`
- Minimum score: `85`
- Caps applied: `none`

## Hard Rule Caps

| Rule | Max Score | Applied |
| --- | ---: | --- |
| `no-root-agent-instructions` | 75 | no |
| `no-one-command-setup-or-validation` | 70 | no |
| `no-deterministic-fast-lane` | 65 | no |
| `no-security-lane-on-high-risk-repo` | 60 | no |
| `generated-contracts-or-public-api-drift-untested` | 80 | no |
| `python-direct-product-truth-or-db-ownership` | 72 | no |
| `no-secret-or-dependency-scanning-in-ci` | 78 | no |
| `no-jankurai-audit-lane-in-ci` | 82 | no |
| `jankurai-required-tool-ci-evidence-gap` | 88 | no |
| `non-optimal-product-language-found` | 74 | no |
| `too-much-python-in-product-surface` | 72 | no |
| `boundary-reclassification-evidence-gap` | 72 | no |
| `vibe-placeholders-in-product-code` | 68 | no |
| `fallback-soup-in-product-code` | 70 | no |
| `future-hostile-dead-language-in-product-code` | 64 | no |
| `severe-duplication-in-product-code` | 70 | no |
| `generated-zone-mutation-risk` | 76 | no |
| `direct-db-access-from-wrong-layer` | 66 | no |
| `missing-web-e2e-lane` | 82 | no |
| `missing-rendered-ux-qa-lane` | 84 | no |
| `prompt-injection-risk` | 78 | no |
| `overbroad-agent-agency` | 65 | no |
| `secret-like-content-detected` | 60 | no |
| `false-green-test-risk` | 76 | no |
| `destructive-migration-risk` | 70 | no |
| `authz-or-data-isolation-gap` | 78 | no |
| `input-boundary-gap` | 78 | no |
| `agent-tool-supply-chain-gap` | 78 | no |
| `release-readiness-gap` | 80 | no |
| `missing-rust-property-or-integration-tests` | 82 | no |
| `no-agent-friendly-exception-pattern` | 76 | no |
| `missing-agent-readable-docs` | 80 | no |
| `streaming-runtime-drift` | 78 | no |
| `rust-bad-behavior` | 72 | no |
| `sql-bad-behavior` | 72 | no |
| `typescript-bad-behavior` | 72 | no |
| `docker-bad-behavior` | 72 | no |
| `python-bad-behavior` | 72 | no |
| `ci-bad-behavior` | 70 | no |
| `git-bad-behavior` | 70 | no |
| `gittools-bad-behavior` | 70 | no |
| `release-bad-behavior` | 70 | no |
| `web-security-bad-behavior` | 68 | no |
| `repo-rot-bad-behavior` | 88 | no |
| `comment-hygiene-dangerous-residue` | 72 | no |
| `ci-local-parity` | 70 | no |

## Copy-Code Redundancy

- Status: `review` hard=`0` warning=`11` files=`177`
- Policy: min-lines=`10` min-tokens=`100` max-findings=`50` include-tests=`false` strict=`false`
- Duplicate volume: lines=`46` tokens=`113` bytes=`1114`

- Notes:
  - hard classes are limited to exact active-source file matches and substantial exact same-name units
  - warning classes include same-body different-name units and token/block duplication
  - tests, fixtures, stories, config, Docker, and migrations are omitted unless --include-tests is set

| Kind | Severity | Language | Lines | Tokens | Instances | Reason |
| --- | --- | --- | ---: | ---: | --- | --- |
| `ExactUnitSameName` | `Warning` | `rust` | 14 | 37 | `crates/jailgun-cli/src/jailhard/browser.rs:97-111, crates/jailgun-orchestrator/src/agent/accounts.rs:83-97` | `same-name semantic unit copied across multiple files` |
| `ExactUnitSameName` | `Warning` | `rust` | 12 | 46 | `crates/jailgun-cli/src/auth/bridge.rs:30-42, crates/jailgun-cli/src/jailhard/browser.rs:174-186` | `same-name semantic unit copied across multiple files` |
| `ExactUnitSameName` | `Warning` | `rust` | 3 | 2 | `crates/jailgun-deploy/src/fake/ci_tracker.rs:15-18, crates/jailgun-deploy/src/fake/job.rs:15-18, crates/jailgun-deploy/src/fake/upload.rs:14-17` | `same-name semantic unit copied across multiple files` |
| `ExactUnitDifferentName` | `Warning` | `rust` | 2 | 1 | `crates/jailgun-core/src/browser_registry/leases/lock.rs:75-77, crates/jailgun-core/src/browser_registry/leases/lock.rs:80-82, crates/jailgun-core/src/browser_registry/storage.rs:54-56, crates/jailgun-core/src/browser_registry/storage.rs:70-72` | `same body appears under different names across files` |
| `ExactUnitSameName` | `Warning` | `rust` | 4 | 12 | `crates/jailgun-cli/src/auth/mod.rs:138-142, crates/jailgun-orchestrator/src/run/bridge_flow.rs:183-187` | `same-name semantic unit copied across multiple files` |
| `ExactUnitDifferentName` | `Warning` | `rust` | 2 | 1 | `crates/jailgun-core/src/agent/request.rs:243-245, crates/jailgun-core/src/browser_registry/leases.rs:439-441, crates/jailgun-orchestrator/src/bridge/command.rs:45-47` | `same body appears under different names across files` |
| `ExactUnitSameName` | `Warning` | `rust` | 3 | 4 | `crates/jailgun-deploy/src/shell/job.rs:20-23, crates/jailgun-deploy/src/shell/upload.rs:15-18` | `same-name semantic unit copied across multiple files` |
| `ExactUnitSameName` | `Warning` | `rust` | 2 | 5 | `crates/jailgun-deploy/src/deploy/events.rs:10-12, crates/jailgun-orchestrator/src/run/publish.rs:4-6` | `same-name semantic unit copied across multiple files` |
| `ExactUnitDifferentName` | `Warning` | `rust` | 1 | 2 | `crates/domain/src/lib.rs:24-25, crates/domain/src/lib.rs:33-34, crates/domain/src/lib.rs:50-51` | `same body appears under different names across files` |
| `ExactUnitSameName` | `Warning` | `rust` | 2 | 3 | `crates/jailgun-deploy/src/shell.rs:17-19, crates/jailgun-deploy/src/util.rs:6-8` | `same-name semantic unit copied across multiple files` |
| `ExactUnitDifferentName` | `Warning` | `rust` | 1 | 0 | `crates/jailgun-core/src/agent_error.rs:35-36, crates/jailgun-deploy/src/deploy/model.rs:151-152, crates/jailgun-server/src/bus.rs:27-28` | `same body appears under different names across files` |

## Dimensions

| Dimension | Weight | Score | Weighted | Evidence |
| --- | ---: | ---: | ---: | --- |
| Ownership and navigation surface | 13 | 100 | 13.00 | root `AGENTS.md` present; owner map present |
| Contract and boundary integrity | 13 | 98 | 12.74 | contract surface found; generated contract artifacts found |
| Proof lanes and test routing | 12 | 100 | 12.00 | one-command setup/validation lane found; deterministic fast lane found |
| Security and supply-chain posture | 12 | 86 | 10.32 | lockfile present; secret or dependency scan tooling found |
| Code shape and semantic surface | 12 | 80 | 9.60 | largest authored code file: crates/jailgun-core/src/browser_registry/leases.rs (441 LOC); most code files stay under 300 LOC |
| Data truth and workflow safety | 8 | 85 | 6.80 | database surface present; structured db boundary manifest present |
| Observability and repair evidence | 8 | 88 | 7.04 | observability libraries or patterns found; ops/observability directory present |
| Context economy and agent instructions | 7 | 93 | 6.51 | root `AGENTS.md` present; root `AGENTS.md` stays short |
| Jankurai tool adoption and CI replacement | 7 | 10 | 0.70 | control-plane files present; applicable=18 |
| Python containment and polyglot hygiene | 4 | 100 | 4.00 | no Python files in scope |
| Build speed signals | 4 | 80 | 3.20 | build acceleration markers found; targeted test/build commands found |

## Reference Profile Structure

- Applicable cells: `4` canonical=`4` noncanonical=`0` guidance missing=`0`

| Cell | Status | Canonical | Detected | Aliases | Guidance | Owner | Proof lane | Agent fix |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `web` | `not_applicable` | `apps/web/` | `-` | `frontend/, ui/, packages/web/, packages/ui/` | `not_required` | `apps/web` | `rendered UX / Playwright` | `no action` |
| `api` | `not_applicable` | `apps/api/` | `-` | `api/, server/, backend/` | `not_required` | `apps/api` | `edge handler / contract tests` | `no action` |
| `domain` | `canonical` | `crates/domain/` | `crates/domain` | `domain/, core/` | `present` | `crates/domain` | `unit / property tests` | `keep `crates/domain/AGENTS.md` aligned with owns / forbidden / proof lane guidance` |
| `application` | `not_applicable` | `crates/application/` | `-` | `application/, usecases/, use-cases/` | `not_required` | `crates/application` | `use-case / authz tests` | `no action` |
| `adapters` | `not_applicable` | `crates/adapters/` | `-` | `adapters/, infra/, integrations/` | `not_required` | `crates/adapters` | `adapter integration tests` | `no action` |
| `workers` | `not_applicable` | `crates/workers/` | `-` | `workers/, jobs/, scheduler/, queue/` | `not_required` | `crates/workers` | `workflow / replay tests` | `no action` |
| `contracts` | `canonical` | `contracts/` | `contracts` | `openapi/, protobuf/, json-schema/, generated/` | `present` | `contracts` | `generation / drift checks` | `keep `contracts/AGENTS.md` aligned with owns / forbidden / proof lane guidance` |
| `db` | `canonical` | `db/` | `db` | `migrations/, constraints/, sql/` | `present` | `db` | `migration / constraint tests` | `keep `db/AGENTS.md` aligned with owns / forbidden / proof lane guidance` |
| `python-ai` | `not_applicable` | `python/ai-service/` | `-` | `python/, ai-service/, evals/, embeddings/, model/` | `not_required` | `python/ai-service` | `eval / contract tests` | `no action` |
| `ops` | `canonical` | `ops/` | `.github, .github/workflows, ops` | `.github/, .github/workflows/, ci/, release/, observability/, security/` | `present` | `ops` | `security lane / workflow lint` | `keep `ops/AGENTS.md` aligned with owns / forbidden / proof lane guidance` |

## Rendered UX QA

- Web surface: `true`
- Layered UX lane: `true`
- Missing: `none`

### Ingested UX QA report (`target/jankurai/ux-qa.json`)
- Report count: `2`
- Worst decision: `pass`
- Total violations: `0`
- Summary errors / warnings: `0` / `0`
- Artifact counts: `accessibility=2, aria-snapshot=2, screenshot=2`
- Artifact fingerprints: `6`
- Visual baseline counts: missing=`0` changed=`0` review=`0` block=`0`
- Missing required states: `0` report(s) `none`
- Missing required artifacts: `0` report(s) `none`
- Accessibility violations / incomplete / passes: `0` / `0` / `24`

## Tool Adoption

- Control plane present: `true`
- Applicable tools: `18`
- Configured: `0`
- CI evidence: `0`
- Artifact verified: `0`
- Replaced count: `0`
- Missing CI evidence: `audit-ci, proof-routing, proofbind, proofmark-rust, copy-code, security, ci-bad-behavior, git-bad-behavior, release-bad-behavior, ux-qa, db-migration-analyze, contract-drift, rust-witness, authz-matrix, input-boundary, agent-tool-supply, release-readiness, cost-budget`

| Tool | Category | Mode | Status | Replaced | Artifacts |
| --- | --- | --- | --- | --- | --- |
| `audit-ci` | `audit` | `auto` | `missing` | `manual repo scoring, ad hoc score gates` | `.jankurai/repo-score.json, .jankurai/repo-score.md` |
| `proof-routing` | `proof` | `auto` | `missing` | `ad hoc proof lane selection, manual proof receipts` | `.jankurai/repo-score.json, .jankurai/repo-score.md, target/jankurai/repair-queue.jsonl` |
| `proofbind` | `proof` | `auto` | `missing` | `manual changed-surface routing, ad hoc proof obligation lists` | `target/jankurai/proofbind/surface-witness.json, target/jankurai/proofbind/obligations.json` |
| `proofmark-rust` | `proof` | `auto` | `missing` | `line-only coverage review, manual in-diff mutation review` | `target/jankurai/proofmark/proofmark-receipt.json, target/jankurai/proofmark/proof-receipt.json` |
| `copy-code` | `audit` | `auto` | `missing` | `ad hoc copy-code review, manual duplication triage` | `target/jankurai/copy-code.json, target/jankurai/copy-code.md` |
| `security` | `security` | `auto` | `missing` | `gitleaks, dependency review, SBOM/provenance` | `target/jankurai/security/evidence.json` |
| `ci-bad-behavior` | `security` | `auto` | `missing` | `mutable workflow refs, secret echo/debug workflow checks, non-blocking security scans` | `target/jankurai/language-bad-behavior.log` |
| `git-bad-behavior` | `audit` | `auto` | `missing` | `destructive git automation, force-push release scripts, hidden stash-based state` | `target/jankurai/language-bad-behavior.log` |
| `release-bad-behavior` | `release` | `auto` | `missing` | `manual release checklist, ad hoc tag and artifact review, manual provenance review` | `target/jankurai/language-bad-behavior.log` |
| `ux-qa` | `ux` | `auto` | `missing` | `playwright, axe-core, visual baselines` | `target/jankurai/ux-qa.json` |
| `db-migration-analyze` | `db` | `auto` | `missing` | `manual migration review` | `target/jankurai/migration-report.json` |
| `contract-drift` | `contract` | `auto` | `missing` | `handwritten contract drift checks, openapi diff` | `.jankurai/repo-score.json, .jankurai/repo-score.md` |
| `rust-witness` | `rust` | `auto` | `missing` | `manual witness graphing` | `target/jankurai/rust/witness-graph.json` |
| `vibe-coverage` | `audit` | `auto` | `not_applicable` | `manual vibe-coding coverage spreadsheet` | `target/jankurai/vibe-coverage.json, target/jankurai/vibe-coverage.md` |
| `coverage-evidence` | `proof` | `auto` | `not_applicable` | `manual coverage report review, ad hoc mutation survivor review` | `target/jankurai/coverage/coverage-audit.json, target/jankurai/coverage/coverage-audit.md` |
| `authz-matrix` | `security` | `auto` | `missing` | `manual authz matrix review` | `.jankurai/repo-score.json, .jankurai/repo-score.md` |
| `input-boundary` | `security` | `auto` | `missing` | `manual unsafe sink review` | `.jankurai/repo-score.json, .jankurai/repo-score.md` |
| `agent-tool-supply` | `security` | `auto` | `missing` | `manual MCP/tool trust review` | `.jankurai/repo-score.json, .jankurai/repo-score.md` |
| `release-readiness` | `release` | `auto` | `missing` | `manual launch checklist` | `.jankurai/repo-score.json, .jankurai/repo-score.md` |
| `cost-budget` | `release` | `auto` | `missing` | `manual spend review` | `.jankurai/repo-score.json, .jankurai/repo-score.md` |

## Boundary manifest (ingested)

- Path: `agent/boundaries.toml`
- Stack: `rust-split-family-child` · version: `1.0.0`
- Queue path counts — adapter: `0`, event_contract: `2`, generated_type: `1`, client_marker: `7`, streaming_exception: `1`
- Content fingerprint: `sha256:6a3023594b57b4e533c8a7152c821d36738b1db3c89630c4b315e99e62fa60b1`

## Boundary Reclassifications

No audited runtime boundary reclassifications declared.

## Findings

1. `medium` `shape` `.`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:shape` `soft` confidence `0.76`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: `Code shape and semantic surface` scored 80 below the standard floor of 85
   Fix: split large or ambiguous authored code into smaller semantic modules with focused tests
   Rerun: `just fast`
   Fingerprint: `sha256:d3464f5131306ae32b0470db781e558575a3108f20fe67563cb3380701b730cf`
   Evidence: largest authored code file: crates/jailgun-core/src/browser_registry/leases.rs (441 LOC), most code files stay under 300 LOC, copy-code advisory classes found: 11 (advisory only, no score impact), rust bad-behavior advisory signals: 425
2. `medium` `governance` `Cargo.lock`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `workspace`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `Cargo.lock` has an uncommitted hand-edit at `Cargo.lock` instead of a regeneration
   Fix: revert the in-place edit to `Cargo.lock` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:6bdf6f31ab054c7a86dde856885fef44c0ca739360d260e05a5196da7624ee57`
   Evidence: `Cargo.lock` was hand-edited inside declared generated zone `Cargo.lock`
3. `medium` `proof` `Justfile`
   Rule: `HLT-018-PERF-CONCURRENCY-DRIFT`
   Check: `HLT-018-PERF-CONCURRENCY-DRIFT:proof` `soft` confidence `0.76`
   Route: TLR `Verification`, lane `fast`, owner `workspace`
   Docs: `docs/testing.md`
   Reason: `Build speed signals` scored 80 below the standard floor of 85
   Fix: add fast deterministic build/test targets, caches, and narrow proof lanes for agent iteration
   Rerun: `just fast`
   Fingerprint: `sha256:2f2531223d7f7036c20d44b58cd52e64aa53ffd6cb85e01e541c1feff0c09cb2`
   Evidence: build acceleration markers found, targeted test/build commands found, locked dependency graph present, CI cache hint found
4. `medium` `governance` `contracts/fixtures/events/auth-action-needed.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/auth-action-needed.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/auth-action-needed.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:b1d8556d0eb75b09b9100e5abda76c8c358495adda6024c3994c25763b371e42`
   Evidence: `contracts/fixtures/events/auth-action-needed.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
5. `medium` `governance` `contracts/fixtures/events/auth-code-requested.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/auth-code-requested.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/auth-code-requested.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:67ee9ddaa0a0075b3a5fc7f6bd8debffe6bdbc83f47d0e8188d323c0830a5fea`
   Evidence: `contracts/fixtures/events/auth-code-requested.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
6. `medium` `governance` `contracts/fixtures/events/auth-code-submitted.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/auth-code-submitted.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/auth-code-submitted.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:5215eb36621f3f77ec6258a004b9739a6978a178620de9c20775628a216c0a6d`
   Evidence: `contracts/fixtures/events/auth-code-submitted.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
7. `medium` `governance` `contracts/fixtures/events/auth-complete.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/auth-complete.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/auth-complete.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:b6cff5a1cb1938ad2412a5c72bc41cdd9a2793e352966c1c94c3886ea0b49f29`
   Evidence: `contracts/fixtures/events/auth-complete.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
8. `medium` `governance` `contracts/fixtures/events/auth-failed.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/auth-failed.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/auth-failed.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:02f19886dc768a15c62cbedd4700de98c9de75af5ba8fe1e4d5b21ce9264fb85`
   Evidence: `contracts/fixtures/events/auth-failed.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
9. `medium` `governance` `contracts/fixtures/events/auth-state.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/auth-state.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/auth-state.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:d9cf7693f0ae2dafd0ef9d51ee809477d7f7f3f324892e4915f5ca63aa0a3f00`
   Evidence: `contracts/fixtures/events/auth-state.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
10. `medium` `governance` `contracts/fixtures/events/browser-log.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/browser-log.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/browser-log.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:a61fb177077ebb87be7d709dff8f55ef1c99a625938c0917011e0a1eb8e52e5e`
   Evidence: `contracts/fixtures/events/browser-log.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
11. `medium` `governance` `contracts/fixtures/events/deploy-finished.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/deploy-finished.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/deploy-finished.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:4949315a2249c65d777c1379101854522008a27924ba8be88bb36ca6195535f0`
   Evidence: `contracts/fixtures/events/deploy-finished.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
12. `medium` `governance` `contracts/fixtures/events/deploy-queued.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/deploy-queued.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/deploy-queued.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:8c8ecabe70735058b56fee594c4671bb1703e41dd644dacdc286e73356eda053`
   Evidence: `contracts/fixtures/events/deploy-queued.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
13. `medium` `governance` `contracts/fixtures/events/download-receipt.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/download-receipt.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/download-receipt.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:18237e7c06e5e4a8fb5affe38cdd1f5fc73794fc5ccc3b6e10a0df7bc985e7cc`
   Evidence: `contracts/fixtures/events/download-receipt.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
14. `medium` `governance` `contracts/fixtures/events/error.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/error.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/error.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:f1e917743656b7290f8d3a1d0599dd99a6376fd47b0aaf85f24d91ad65732083`
   Evidence: `contracts/fixtures/events/error.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
15. `medium` `governance` `contracts/fixtures/events/prompt-policy-deny.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/prompt-policy-deny.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/prompt-policy-deny.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:ade8146fa8850eb2420ce701ca33fbc3e7864e383bdf0342dadd495c4e1e8b0c`
   Evidence: `contracts/fixtures/events/prompt-policy-deny.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
16. `medium` `governance` `contracts/fixtures/events/prompt-submitted.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/prompt-submitted.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/prompt-submitted.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:1c9844bc41641518759a25982516c3aa73edf56b025dadb19cfaca5c8d9e8471`
   Evidence: `contracts/fixtures/events/prompt-submitted.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
17. `medium` `governance` `contracts/fixtures/events/rate-limit-detected.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/rate-limit-detected.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/rate-limit-detected.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:c7be4299eebff7899e54b91395efa1ad00eca8b88887f1ea40912f686a82a608`
   Evidence: `contracts/fixtures/events/rate-limit-detected.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
18. `medium` `governance` `contracts/fixtures/events/remote-safety.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/remote-safety.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/remote-safety.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:b5604477353559f19ad9357e668c0355f5d279ed9f68c40038085fd0fe989f30`
   Evidence: `contracts/fixtures/events/remote-safety.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
19. `medium` `governance` `contracts/fixtures/events/run-started.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/run-started.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/run-started.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:436e89c605093d1377a1e9020fb39e76f708c51dffd776f07807ce548cf65424`
   Evidence: `contracts/fixtures/events/run-started.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
20. `medium` `governance` `contracts/fixtures/events/session-expired.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/session-expired.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/session-expired.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:2137152ab75c91ee9c8e58b644eba1ab13e65a4f060bfe1f6c392e817ac51897`
   Evidence: `contracts/fixtures/events/session-expired.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
21. `medium` `governance` `contracts/fixtures/events/tab-opened.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/tab-opened.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/tab-opened.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:8f413cbc8923cbdd30910e566e6e291d7a3f80b56b0d1b054909d5367aecb326`
   Evidence: `contracts/fixtures/events/tab-opened.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`
22. `medium` `governance` `contracts/fixtures/events/tar-discovered.json`
   Rule: `HLT-045-GENERATED-ZONE-GOVERNANCE`
   Check: `HLT-045-GENERATED-ZONE-GOVERNANCE:governance` `soft` confidence `0.76`
   Route: TLR `Contracts/data`, lane `contract`, owner `contracts`
   Docs: `agent/JANKURAI_STANDARD.md#generated-zones`
   Reason: generated zone `contracts/fixtures/events/` has an uncommitted hand-edit at `contracts/fixtures/events/tar-discovered.json` instead of a regeneration
   Fix: revert the in-place edit to `contracts/fixtures/events/tar-discovered.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Rerun: `just fast`
   Fingerprint: `sha256:645182cd89f82d6f6d63abb87ca9eff8dc26bfed37ad94ad6ff893b1afb7b5c4`
   Evidence: `contracts/fixtures/events/tar-discovered.json` was hand-edited inside declared generated zone `contracts/fixtures/events/`

## Policy

- Policy file: `./agent/audit-policy.toml`
- Minimum score: `85`
- Fail on: `critical, high`

## Agent Fix Queue

1. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `Cargo.lock` - revert the in-place edit to `Cargo.lock` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
2. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/auth-action-needed.json` - revert the in-place edit to `contracts/fixtures/events/auth-action-needed.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
3. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/auth-code-requested.json` - revert the in-place edit to `contracts/fixtures/events/auth-code-requested.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
4. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/auth-code-submitted.json` - revert the in-place edit to `contracts/fixtures/events/auth-code-submitted.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
5. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/auth-complete.json` - revert the in-place edit to `contracts/fixtures/events/auth-complete.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
6. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/auth-failed.json` - revert the in-place edit to `contracts/fixtures/events/auth-failed.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
7. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/auth-state.json` - revert the in-place edit to `contracts/fixtures/events/auth-state.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
8. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/browser-log.json` - revert the in-place edit to `contracts/fixtures/events/browser-log.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
9. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/deploy-finished.json` - revert the in-place edit to `contracts/fixtures/events/deploy-finished.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
10. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/deploy-queued.json` - revert the in-place edit to `contracts/fixtures/events/deploy-queued.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
11. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/download-receipt.json` - revert the in-place edit to `contracts/fixtures/events/download-receipt.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
12. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/error.json` - revert the in-place edit to `contracts/fixtures/events/error.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
13. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/prompt-policy-deny.json` - revert the in-place edit to `contracts/fixtures/events/prompt-policy-deny.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
14. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/prompt-submitted.json` - revert the in-place edit to `contracts/fixtures/events/prompt-submitted.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
15. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/rate-limit-detected.json` - revert the in-place edit to `contracts/fixtures/events/rate-limit-detected.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
16. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/remote-safety.json` - revert the in-place edit to `contracts/fixtures/events/remote-safety.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
17. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/run-started.json` - revert the in-place edit to `contracts/fixtures/events/run-started.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
18. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/session-expired.json` - revert the in-place edit to `contracts/fixtures/events/session-expired.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
19. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/tab-opened.json` - revert the in-place edit to `contracts/fixtures/events/tab-opened.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
20. `medium` `HLT-045-GENERATED-ZONE-GOVERNANCE` `contracts/fixtures/events/tar-discovered.json` - revert the in-place edit to `contracts/fixtures/events/tar-discovered.json` and regenerate it from the declared source/command in `agent/generated-zones.toml`; do not patch generated output by hand
   Route: `Contracts/data`/`contract`
21. `medium` `HLT-018-PERF-CONCURRENCY-DRIFT` `Justfile` - add fast deterministic build/test targets, caches, and narrow proof lanes for agent iteration
   Route: `Verification`/`fast`
22. `medium` `HLT-001-DEAD-MARKER` `.` - split large or ambiguous authored code into smaller semantic modules with focused tests
   Route: `Entropy`/`fast`
