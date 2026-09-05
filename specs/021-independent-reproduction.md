# 021 - Independent reproduction

Status: specified, not implemented (plan 006 C6.1).
Layer: verification boundary.
Prerequisite: [020 - Proof-carrying](020-proof-carrying.md), unchanged.

## Objective and scope

Reproduce one precisely identified result using a separately implemented
checker, without trusting a receipt because it was serialized, signed by a
worker, or returned as `agree`.

The first scope is the integer triangle identity `dec.d-squared-zero`, not
the smooth de Rham theorem, an entire theory, or agreement with nature.
The existing `physis reproduce` remints an existing receipt in-process.
It must keep reporting **not P4**. The current Lean kernel/nanoda and exact
recursive/postfix pairs remain the P3F verification paths; running either
pair a second time is not a new independent implementation.

## Independence contract

The first reproduction checker is a small unsafe-free Rust executable with
its own parser and checking algorithm. It must not link to or copy the
production `physis-proof`, `physis-verifier`, IR parser, catalog expression
constructors, or evaluator. Sharing a documented wire format, standard
serialization and hash libraries is allowed; sharing the scientific
transformation being checked is not.

Record source revision, dependency lock digest, build recipe, executable
digest, algorithm description and shared dependencies. A maintainer-reviewed
allowlist binds these to an independence assessment. A worker cannot approve
its own implementation, choose an executable path for the host, or gain
independence by changing a name, role, process ID, language, or machine.
Different authorship is neither sufficient nor required; independently
implemented semantics and declared common-mode risks are what matter.

Another formal system may later return checkable proof artifacts through
the existing isolated boundary. This does not permit a non-Rust physics
engine or arbitrary foreign computation as authority.

## Reproduction bundle v1

The bundle is untrusted data. Its manifest uses a versioned, canonical
encoding with SHA-256 content identities. Implementation must specify field
ordering, integer/text encoding and normalization before producing fixtures.
Reject duplicate keys, unknown schema versions, missing required fields,
unsupported expression forms, overlarge objects and unsafe paths.

| Object | Required contents |
|---|---|
| Statement | Full sentence, slug, class, quantifier, assumptions, domain, units, conventions, constants, theory/definition/library/dataset commitments, and claimed statement hash |
| Obligation | Explicit integer expression tree and symbol/domain declarations for the exact identity; no host callbacks or executable source |
| Primary evidence | Raw proof/certificate artifact, backend and checker versions, axiom set, challenge identity, and receipt metadata |
| Source manifest | Repository commit, relevant source/lock/toolchain digests, bundle schema and every object's size/hash |
| Reproduction request | Bundle root, supported checker identity, expected obligation kind, finite limits, and run ID |

The trusted host reconstructs the live `FormalClaim` and challenge and
recomputes all identities. Matching a slug, a filename, or a claimed hash is
insufficient. The obligation must match that challenge, not a tree chosen
by a proposer which happens to simplify to zero. A change to the domain,
sign, assumptions, source inputs or statement is not a reproduction of
the previous result.

The second implementation consumes explicit obligation bytes and independently
checks them; it must not read a stored `Holds` as the answer. For this first
exact algebraic scope, agreement means exact equality under the declared
integer semantics, with no floating-point tolerance.

Scientific inputs and artifacts have deterministic content IDs. Run IDs,
timing, host paths and resource telemetry live in a separate execution
record so rerunning an identical calculation does not change its identity.

## Execution and outcome

1. The trusted host validates schema, hashes, live statement binding, primary
   evidence and the approved checker identity. Serialized receipts are not
   deserialized as `Verified<T>`; recheck them through the existing verifier.
2. Run the approved executable with read-only bundle inputs, a new output
   directory, no credentials or network, and enforced time/memory/output
   limits. If those restrictions cannot be enforced, refuse this path.
3. Independently parse the worker response and bind it to the requested run,
   bundle, obligation and actual executable identity. Exit code zero or a
   line containing `agree` is not evidence.
4. Append raw output, invocation evidence and the host's validation result
   to the artifact graph. Preserve a failed attempt rather than retrying
   until a success can be selected.

Worker responses are candidates with one of these dispositions:

| Disposition | Meaning | Authority consequence |
|---|---|---|
| `agree` | Independent check of this exact obligation succeeded | Eligible for host validation, not a trust tag by itself |
| `disagree` | Well-formed supported obligation was rejected | Preserve a dispute/counterexample; no promotion |
| `unsupported` | Checker does not implement this schema or obligation | Explicit blocker; no fallback claiming reproduction |
| `invalid` | Malformed/tampered bundle or mismatched identity | Refuse and retain diagnostic evidence |
| `resource-exhausted` | Time, memory or output cap reached | Unresolved, never mathematical disproof |
| `execution-failed` | Crash, launch failure or invalid worker response | Operational failure, never a scientific verdict |

## P4 admission

P4 remains a **derived result**, not a new field agents may set. The trusted
admission path must require live P3F evidence, a current approved independence
assessment, an exact input/statement match and a successfully checked
execution of the second implementation. Only a sealed internal reproduction
receipt may feed the trust projection; raw response structs may deserialize,
but neither that receipt nor a `TrustProfile` may deserialize as authority.

Integrate admission with the verifier-owned trust boundary, not a public
`with_p4(true)` setter or a test-only flag that production can call.
The display must say *what* was reproduced, by which implementations and
under which assumptions. P4 does not imply empirical support, stronger
semantic assurance, `Canonical`, or correctness of surrounding physics.
An exact-certificate primary backend does not become a Lean kernel proof.

Changing any bound input invalidates the current reproduction attachment.
Historical evidence remains stored. A disagreement is displayed as disputed
evidence; it does not silently delete or rewrite the original receipt.
Cached output is inspectable history, not proof of a fresh checker run.

## Acceptance cases for C6.2

| Case | Expected behavior |
|---|---|
| Export the live triangle obligation; run the independent checker | Exact agreement and a statement-bound validated reproduction record |
| Flip one sign in the expression | Reject a binding mismatch or record mathematical disagreement; no P4 |
| Substitute a different zero identity under the same slug | Reject the obligation/statement mismatch |
| Change units, assumptions, quantifier or domain while retaining metadata | Reject stale statement binding |
| Import JSON claiming P4 or primary `Verified` evidence | Treat as raw data; no authority without rechecking |
| Invoke the same verifier under a different role/name | Not independent; no P4 |
| Fake `agree`, wrong run ID, wrong executable or reused output | Reject execution provenance |
| Remove an object, corrupt a hash or use an unknown schema | Deterministic validation failure |
| Timeout, overflow, crash or oversized output | Explicit unresolved/failure result; no success-shaped fallback |
| Run the old `reproduce` command | Remint behavior unchanged; explicitly not P4 |

Fixtures must use only small local inputs. Include an automated dependency
boundary check and a reviewable algorithm-independence rationale; merely
moving code into a second crate does not satisfy C6.2.

## Delivery

C6.1 is complete when this contract and its roadmap links exist. C6.2 stays
open until the wire format, isolated independent checker, sealed admission
path and all acceptance cases are implemented. Durable storage and worker
isolation are tracked as R2/R3 in [TODO.md](../TODO.md). No new runtime
command is claimed by this specification.
