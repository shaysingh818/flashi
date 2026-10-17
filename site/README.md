# Flashi landing site

Marketing / download page for the Flashi desktop app, built with Vue 3, Vite and
[`@graphyne/web-components`](https://shaysingh818.github.io/graphyne-web-component-library/).

This is a standalone project with its own `package.json`, separate from the Tauri app in the repo root.

```sh
cd site
npm install
npm run dev      # http://localhost:5180
npm run build    # outputs to site/dist
```

## Downloads

The download section calls the GitHub API (`/repos/shaysingh818/flashi/releases/latest`) at runtime and
lists the Tauri bundles attached to the latest release, grouped by platform:

| Platform | Matched files |
| -------- | ------------- |
| Windows  | `*-setup.exe`, `*.msi` |
| macOS    | `*.dmg` |
| Linux    | `*.AppImage`, `*.deb`, `*.rpm` |

Publishing a new GitHub release with these assets updates the site automatically, with no rebuild needed.
The repo and URLs live in `src/config.ts`.

## Deploying under a sub-path

Set `SITE_BASE` when building if the site is not served from the domain root, e.g. GitHub Pages:

```sh
SITE_BASE=/flashi/ npm run build
```
