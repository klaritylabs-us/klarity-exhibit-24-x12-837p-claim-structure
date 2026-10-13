# 837P / professional Claim structure — Rust

Small library and CLI that sanity-check synthetic **837P** segment sketches and **FHIR Claim** JSON before clearinghouse handoff — ST/CLM/SV1 patterns and claim **patient** + **item** rows.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

**RCM / Claims edition** — separate from the US Core exhibit series (synthetic EDI sketches + FHIR where noted).

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 24** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [FHIR Claim](http://hl7.org/fhir/R4/claim.html)
- X12 837P (structural sketch only)

## What it checks

- FHIR **Claim** with **patient** and **item[]**
- 837P sketch: **ST**, **CLM**, warning if **SV1** missing

| Problem | What breaks |
|--------|-------------|
| **Claim without items** | Professional claim rejected |
| **837P without CLM** | Transaction set not a claim |
| **Wrong resource** | Mapper pointed at wrong profile |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-837p -- examples/valid-professional-claim.json
cargo run --bin validate-837p -- examples/invalid-professional-claim.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or input.

## Sample output

```bash
cargo run --bin validate-837p -- examples/valid-professional-claim.json
```

**Pass** — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-professional-claim.json`) — exit code `1`:

```json
[
  {
    "severity": "error",
    "path": "item",
    "rule_id": "..."
  }
]
```

## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken fixtures for tests and demos

## GitHub topics

Paste in repository **About** (also: `fhir`, `hl7-fhir`, `healthcare-interoperability`, `rust`):

`x12-edi, 837p, claims, fhir-claim, fhir, hl7-fhir, healthcare-interoperability, rust`

## Inferno vs this repo

Not a FHIR exhibit. This is **claims pipe shape** next to your FHIR programs.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
