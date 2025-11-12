# Repository Guidelines

## Project Structure & Module Organization
- SvelteKit UI lives in `src`; `src/routes/+page.svelte` drives the main view and shares layout logic via `+layout.ts`.
- Static files (icons, fonts, mock JSON) go in `static` so Vite serves them verbatim.
- Native shell code sits in `src-tauri` (`src/main.rs`, `Cargo.toml`, `tauri.conf.json`), and build artifacts stay inside `src-tauri/target`.
- Build configs such as `package.json`, `svelte.config.js`, `vite.config.js`, `tsconfig.json`, and `bun.lock` stay at the root.

## Build, Test, and Development Commands
- `bun install` (or `npm install`) — sync JavaScript dependencies; Bun is preferred because the repo tracks `bun.lock`.
- `npm run dev` — hot-reload the SvelteKit app in the browser.
- `npm run tauri dev` — wrap the dev server in the native window.
- `npm run build && npm run preview` — create and spot-check the production bundle before packaging.
- `npm run check` / `npm run check:watch` — run `svelte-kit sync` plus `svelte-check`.
- `cargo test` (from `src-tauri`) — validate Rust units before packaging.

## Coding Style & Naming Conventions
Use two-space indentation, single quotes in scripts, and double quotes in markup. Components and layouts should be `PascalCase.svelte`; colocated helpers or stores use `camelCase.ts`. Keep shared utilities inside `src/lib`, prefer Svelte stores over global singletons, and keep Rust modules snake_case with `cargo fmt`.

## Testing Guidelines
UI tests should live in `src/routes/__tests__` (create as needed) using Vitest + Svelte Testing Library with filenames such as `ListView.spec.ts`; cover props, emitted events, and accessibility states. The native layer should place module tests in `src-tauri/src/lib.rs` and integration tests under `src-tauri/tests`, executed via `cargo test`. Aim for roughly 80% coverage on new logic and capture any manual QA steps (OS, test command, result) in the PR description.

## Commit & Pull Request Guidelines
History favors short imperative subjects (e.g., `Primer commit`), so keep summaries ≤50 characters and describe intent. Reference issues with `Closes #123`, list the commands you ran, and include screenshots or recordings for UI-visible work. Pull requests must explain any updates to `src-tauri/capabilities` or environment expectations so reviewers can assess security.

## Security & Configuration Tips
Secrets belong in `.env` / `.env.local`, never in Git; mirror required keys in `tauri.conf.json`. Grant the smallest capability set when editing `src-tauri/capabilities`, and justify new permissions in the PR. Update dependencies via `bun update` or `npm update`, and run `cargo audit` before tagging a release build.
