# Contributor Recognition System

This document defines how the Soroban Cookbook recognises and rewards
contributors.  All criteria, tier levels, and rewards described here apply to
the open-source repository at
[Soroban-Cookbook/Soroban-Cookbook-](https://github.com/Soroban-Cookbook/Soroban-Cookbook-).

---

## Table of Contents

1. [Recognition Criteria](#recognition-criteria)
2. [Contribution Tiers](#contribution-tiers)
3. [Rewards](#rewards)
4. [Automation Plan](#automation-plan)
5. [Nomination and Review Process](#nomination-and-review-process)

---

## Recognition Criteria

A contribution is eligible for recognition when it meets **all** of the
following quality gates:

| Gate | Requirement |
|------|-------------|
| Tests pass | CI green on the PR (build, clippy, tests) |
| Follows style guide | `cargo fmt` clean; follows [`docs/style-guide.md`](./style-guide.md) |
| Acceptance criteria met | Issue's acceptance criteria are all checked |
| Reviewed and merged | PR reviewed by at least one maintainer and merged to `main` |

### Contribution types counted

- **Code**: new example contracts, bug fixes, refactors
- **Documentation**: new guides, corrections, translations
- **Testing**: additional unit tests, integration tests, coverage improvements
- **Community**: issue triage, PR reviews, answering questions in Discussions

---

## Contribution Tiers

Tiers accumulate permanently — reaching a higher tier does not remove lower-
tier benefits.

### Tier 1 — Contributor

**Threshold:** 1 merged PR of any size.

**Recognition:**
- Name listed in the **Contributors** section of [`README.md`](../README.md)
- `contributor` label applied to your GitHub profile in the repo's member list

---

### Tier 2 — Cookbook Author

**Threshold:** 3 or more merged PRs **or** 1 substantial contribution
(a complete, tested example crate with README).

**Recognition:**
- Name listed under **Authors** in [`README.md`](../README.md)
- Mention in the quarterly project update (GitHub Discussion)
- `author` label on GitHub

---

### Tier 3 — Core Contributor

**Threshold:** 10 or more merged PRs **or** sustained engagement over at least
2 consecutive months (code + review + community).

**Recognition:**
- Listed as a **Core Contributor** in [`README.md`](../README.md) with a brief
  bio/link
- Invited to the private `#cookbook-core` channel (if a community chat exists)
- May be nominated for maintainer rights

---

### Tier 4 — Maintainer

**Threshold:** Nominated by an existing maintainer and approved by consensus.
Typically reached after Tier 3 with demonstrated commitment to project
direction and code quality.

**Recognition:**
- Listed as **Maintainer** in [`README.md`](../README.md)
- Write access to the repository
- Participates in roadmap decisions

---

## Rewards

> Soroban Cookbook is an open-source community project.  Rewards are
> non-monetary and community-driven unless explicitly noted otherwise.

| Tier | Rewards |
|------|---------|
| 1 — Contributor | Credit in README; `contributor` GitHub label |
| 2 — Cookbook Author | Credit in README (Authors section); quarterly shout-out |
| 3 — Core Contributor | Prominent README credit with bio; invited to core channel; roadmap input |
| 4 — Maintainer | Repo write access; release authority; featured on project homepage if applicable |

### Special recognitions

- **First-time contributor badge**: Automatically added (via bot comment) on a
  contributor's first merged PR.
- **Milestone badge**: Awarded for contributions that close a tracked phase
  milestone (e.g., completing all Phase 5 examples).
- **Bug hunter**: Awarded for finding and fixing a security or correctness bug.

---

## Automation Plan

The following automations are planned to reduce manual overhead:

### Phase 1 — Immediate (manual process)

- Maintainers manually update the `Contributors` / `Authors` / `Core
  Contributors` tables in `README.md` at the end of each month.
- First-merged-PR welcome comment posted manually or via a GitHub Action
  trigger on `pull_request` events with `merged: true`.

### Phase 2 — GitHub Actions

A lightweight GitHub Action (`recognition-bot`) will:

1. **Trigger:** `pull_request` event with `action: closed` and `merged: true`.
2. **Count** the contributor's total merged PRs by querying the GitHub API.
3. **Determine** the appropriate tier using the thresholds above.
4. **Post** a comment on the PR congratulating the contributor and stating
   their current tier.
5. **Open** a follow-up issue or PR to update `README.md` if a tier threshold
   was crossed.

> Implementation note: the action should use `GITHUB_TOKEN` and avoid
> third-party secrets.  Tier state can be stored in a `docs/contributors.json`
> file committed to the repo.

### Phase 3 — Enhanced tracking

- Leaderboard page generated automatically from `docs/contributors.json` and
  published to the mdBook site.
- Monthly digest GitHub Discussion created automatically listing all new
  contributors and tier promotions.

---

## Nomination and Review Process

1. **Self-nomination**: A contributor may open a GitHub Discussion titled
   "Recognition: [GitHub username]" listing their contributions and the tier
   they believe they qualify for.
2. **Maintainer nomination**: Any maintainer may open the same type of
   Discussion on behalf of a contributor.
3. **Review window**: 7 days for the community to comment.
4. **Decision**: A maintainer merges a PR updating `README.md` and
   `docs/contributors.json` to record the promotion.

---

## Current Contributors

> This section is updated manually by maintainers.  Automated updates are
> planned for Phase 2.

See the [Contributors section of README.md](../README.md#contributors) for the
current list.
