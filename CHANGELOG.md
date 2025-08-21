# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0 (2025-08-21)

<csr-id-d9b7025863f60cf413c41af177beaf1f4f612110/>
<csr-id-0695e605df391001a0bf1999270f782105890fa1/>
<csr-id-172da047bab18da8da09cbc170482728ac005105/>
<csr-id-d0f1e5f3965af83d467000dc5e4fd11f7637dbf0/>
<csr-id-0564449579ec6aaefffe113562bf54e125fd1a06/>
<csr-id-ac21bee3afbfd9c875c647afe694061e17659550/>
<csr-id-234f5a245421ad9d4a3408393b8db56a2739d7e5/>
<csr-id-f85ef0231d8c7aba037dcd3e50734729a238b3e3/>
<csr-id-910e5e53f2aff1d3c0b34eebe897d0c59bd19260/>
<csr-id-9c7962959aa0f8465e11f9d074be2382b6394d95/>
<csr-id-329496a4abf03cca30b4d70bba94463ed06cbe71/>
<csr-id-a12d1076bb0e2902c403d4d21c587d2fb66f60f3/>

### Chore

 - <csr-id-d9b7025863f60cf413c41af177beaf1f4f612110/> Update incompatible dependencies
 - <csr-id-0695e605df391001a0bf1999270f782105890fa1/> Update compatible dependencies
 - <csr-id-172da047bab18da8da09cbc170482728ac005105/> Cargo fmt
 - <csr-id-d0f1e5f3965af83d467000dc5e4fd11f7637dbf0/> Add todo and commit Cargo.lock

### New Features

 - <csr-id-9ba54453ec9f236401d7eb5b0cece73134fcc136/> Add color to repo sizes
 - <csr-id-4c36b195673cc9366ce607d71ca8f4d4cade2e95/> Display repo and total sizes
 - <csr-id-dd231e48cef9415b77a05de0298af153d48c5520/> implement repo size calculation with directory traversal and human-readable formatting
 - <csr-id-a12fbe36fd0266d06138873eb7cff6d9a11ce2a5/> add placeholder for repo size calculation method
 - <csr-id-afecfe6a96b3490dc56142436756e9ee6a5cfdd6/> Use format_display_time for all dates
 - <csr-id-1c2111c53b5ff3252ac4a89eeaacf458e44dcd77/> implement last_fetch method to retrieve repository fetch/clone date
 - <csr-id-eb86d256ccdd7ccf6df93c77c77515c90e3fb629/> add color-coded duration output for commit timestamps
 - <csr-id-96ae736442ff249d723659c67b47f56035a4a4f5/> add human-readable time duration for commit timestamps
 - <csr-id-4b4eda6f014acb24c433624c3eb0f47d9b7e8a4f/> add human-readable time duration to commit timestamp
 - <csr-id-91f6d2667b9f1fe431476ece06c0712a7fa4289d/> display commit timestamp in local timezone
 - <csr-id-b2be3c55abc4461d612da7f1cf9a5525c3ebd526/> implement last_update method to retrieve most recent commit timestamp
 - <csr-id-95c8f0e384164f8d5fe2b15b2af2092852ab8b2e/> add placeholder comment for last_update method implementation
 - <csr-id-3c9703d4a62c8bc1d9d561a0641ea4adeeeff39d/> Canonicalize the work directory path
 - <csr-id-8e4770674f11903226cecaf4f706db144ed32edb/> Allow printing of urls for remotes
 - <csr-id-73cdcbfc9561104eaa3b38a5b3d2e843bcb93ca0/> implement repo_urls method to retrieve fetch URLs from remotes
 - <csr-id-ed7197b75f6025724b588c4aeddb5db45ce2513e/> add placeholder for repo URLs retrieval method
 - <csr-id-e29be4ff4e61c09316a47faaa0a6b83efcf5513e/> create Rust CLI program with stubbed git repository info methods

### Bug Fixes

 - <csr-id-2794159cbd44cf6b60da11d62ee725c997ee6b09/> Minor cleanup
 - <csr-id-2b6b8a4110337b795d7ac9749a326d6fba5f97bb/> resolve build errors in get_most_recent_commit_time method
 - <csr-id-dd6ae3368a40f59142402feb197930c222681ceb/> resolve iterator and time handling in get_most_recent_commit_time method
 - <csr-id-a36d779e613922225a085fc9f2398b12c88982c6/> resolve ownership issue by cloning paths vector
 - <csr-id-ee18c14473903cf66b9c327ec9d26ac4531c9160/> upgrade gix dependency to resolve build errors

### Refactor

 - <csr-id-0564449579ec6aaefffe113562bf54e125fd1a06/> Move `calculate_directory_size` to util
 - <csr-id-ac21bee3afbfd9c875c647afe694061e17659550/> Pass times directly and use `max`
 - <csr-id-234f5a245421ad9d4a3408393b8db56a2739d7e5/> Add util function format_display_time
 - <csr-id-f85ef0231d8c7aba037dcd3e50734729a238b3e3/> Replace custom code with yansi
 - <csr-id-910e5e53f2aff1d3c0b34eebe897d0c59bd19260/> Replace custom code with chrono-humanize
 - <csr-id-9c7962959aa0f8465e11f9d074be2382b6394d95/> Improve timestamp formatting with color-based duration display
 - <csr-id-329496a4abf03cca30b4d70bba94463ed06cbe71/> improve code formatting and add comment for local timezone display
 - <csr-id-a12d1076bb0e2902c403d4d21c587d2fb66f60f3/> Restructure program

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 42 commits contributed to the release.
 - 34 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add auto-generated changelog ([`5c5f7fd`](https://github.com/egrieco/giti/commit/5c5f7fd7eb9d8288f5cbc02ffa07e0a13efc0f1f))
    - Add license and update metadata ([`4d70161`](https://github.com/egrieco/giti/commit/4d70161fd1ddeb0d017f5752e49a53af5932cece))
    - Move `calculate_directory_size` to util ([`0564449`](https://github.com/egrieco/giti/commit/0564449579ec6aaefffe113562bf54e125fd1a06))
    - Update incompatible dependencies ([`d9b7025`](https://github.com/egrieco/giti/commit/d9b7025863f60cf413c41af177beaf1f4f612110))
    - Update compatible dependencies ([`0695e60`](https://github.com/egrieco/giti/commit/0695e605df391001a0bf1999270f782105890fa1))
    - Add color to repo sizes ([`9ba5445`](https://github.com/egrieco/giti/commit/9ba54453ec9f236401d7eb5b0cece73134fcc136))
    - Display repo and total sizes ([`4c36b19`](https://github.com/egrieco/giti/commit/4c36b195673cc9366ce607d71ca8f4d4cade2e95))
    - Minor cleanup ([`2794159`](https://github.com/egrieco/giti/commit/2794159cbd44cf6b60da11d62ee725c997ee6b09))
    - Implement repo size calculation with directory traversal and human-readable formatting ([`dd231e4`](https://github.com/egrieco/giti/commit/dd231e48cef9415b77a05de0298af153d48c5520))
    - Add placeholder for repo size calculation method ([`a12fbe3`](https://github.com/egrieco/giti/commit/a12fbe36fd0266d06138873eb7cff6d9a11ce2a5))
    - Use format_display_time for all dates ([`afecfe6`](https://github.com/egrieco/giti/commit/afecfe6a96b3490dc56142436756e9ee6a5cfdd6))
    - Pass times directly and use `max` ([`ac21bee`](https://github.com/egrieco/giti/commit/ac21bee3afbfd9c875c647afe694061e17659550))
    - Add util function format_display_time ([`234f5a2`](https://github.com/egrieco/giti/commit/234f5a245421ad9d4a3408393b8db56a2739d7e5))
    - Replace custom code with yansi ([`f85ef02`](https://github.com/egrieco/giti/commit/f85ef0231d8c7aba037dcd3e50734729a238b3e3))
    - Replace custom code with chrono-humanize ([`910e5e5`](https://github.com/egrieco/giti/commit/910e5e53f2aff1d3c0b34eebe897d0c59bd19260))
    - Cargo fmt ([`172da04`](https://github.com/egrieco/giti/commit/172da047bab18da8da09cbc170482728ac005105))
    - Implement last_fetch method to retrieve repository fetch/clone date ([`1c2111c`](https://github.com/egrieco/giti/commit/1c2111c53b5ff3252ac4a89eeaacf458e44dcd77))
    - Add color-coded duration output for commit timestamps ([`eb86d25`](https://github.com/egrieco/giti/commit/eb86d256ccdd7ccf6df93c77c77515c90e3fb629))
    - Improve timestamp formatting with color-based duration display ([`9c79629`](https://github.com/egrieco/giti/commit/9c7962959aa0f8465e11f9d074be2382b6394d95))
    - Add human-readable time duration for commit timestamps ([`96ae736`](https://github.com/egrieco/giti/commit/96ae736442ff249d723659c67b47f56035a4a4f5))
    - Add human-readable time duration to commit timestamp ([`4b4eda6`](https://github.com/egrieco/giti/commit/4b4eda6f014acb24c433624c3eb0f47d9b7e8a4f))
    - Add todo and commit Cargo.lock ([`d0f1e5f`](https://github.com/egrieco/giti/commit/d0f1e5f3965af83d467000dc5e4fd11f7637dbf0))
    - Display commit timestamp in local timezone ([`91f6d26`](https://github.com/egrieco/giti/commit/91f6d2667b9f1fe431476ece06c0712a7fa4289d))
    - Improve code formatting and add comment for local timezone display ([`329496a`](https://github.com/egrieco/giti/commit/329496a4abf03cca30b4d70bba94463ed06cbe71))
    - Resolve build errors in get_most_recent_commit_time method ([`2b6b8a4`](https://github.com/egrieco/giti/commit/2b6b8a4110337b795d7ac9749a326d6fba5f97bb))
    - Resolve iterator and time handling in get_most_recent_commit_time method ([`dd6ae33`](https://github.com/egrieco/giti/commit/dd6ae3368a40f59142402feb197930c222681ceb))
    - Implement last_update method to retrieve most recent commit timestamp ([`b2be3c5`](https://github.com/egrieco/giti/commit/b2be3c55abc4461d612da7f1cf9a5525c3ebd526))
    - Add placeholder comment for last_update method implementation ([`95c8f0e`](https://github.com/egrieco/giti/commit/95c8f0e384164f8d5fe2b15b2af2092852ab8b2e))
    - Canonicalize the work directory path ([`3c9703d`](https://github.com/egrieco/giti/commit/3c9703d4a62c8bc1d9d561a0641ea4adeeeff39d))
    - Allow printing of urls for remotes ([`8e47706`](https://github.com/egrieco/giti/commit/8e4770674f11903226cecaf4f706db144ed32edb))
    - Implement repo_urls method to retrieve fetch URLs from remotes ([`73cdcbf`](https://github.com/egrieco/giti/commit/73cdcbfc9561104eaa3b38a5b3d2e843bcb93ca0))
    - Add placeholder for repo URLs retrieval method ([`ed7197b`](https://github.com/egrieco/giti/commit/ed7197b75f6025724b588c4aeddb5db45ce2513e))
    - Restructure program ([`a12d107`](https://github.com/egrieco/giti/commit/a12d1076bb0e2902c403d4d21c587d2fb66f60f3))
    - Reorganize code ([`bbae4e2`](https://github.com/egrieco/giti/commit/bbae4e2efb154072edb58f2a3af1dc179a121fa6))
    - Update Cargo.toml metadata ([`8f03879`](https://github.com/egrieco/giti/commit/8f03879b183236cf89a64d64c1159839eeff4c03))
    - Update ignore and add Cargo.lock ([`d88419a`](https://github.com/egrieco/giti/commit/d88419a5f972c740909dae52a0e20f3dbad29850))
    - Resolve ownership issue by cloning paths vector ([`a36d779`](https://github.com/egrieco/giti/commit/a36d779e613922225a085fc9f2398b12c88982c6))
    - Upgrade gix dependency to resolve build errors ([`ee18c14`](https://github.com/egrieco/giti/commit/ee18c14473903cf66b9c327ec9d26ac4531c9160))
    - Create Rust CLI program with stubbed git repository info methods ([`e29be4f`](https://github.com/egrieco/giti/commit/e29be4ff4e61c09316a47faaa0a6b83efcf5513e))
    - Add ignore file ([`66633cb`](https://github.com/egrieco/giti/commit/66633cb5a3a507f90c690f32733be4781a5d9b23))
    - Add design overview ([`eb6f7f9`](https://github.com/egrieco/giti/commit/eb6f7f939668444f71d9cbd6eae49b9be5bb1a9c))
    - Initial Commit ([`b6e0f41`](https://github.com/egrieco/giti/commit/b6e0f419d6cf31d4569e06241b12efd9eca6e668))
</details>

