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

## Deploying to GitHub Pages

`.github/workflows/deploy-site.yml` builds and deploys the site whenever changes under `site/` are pushed to
`main`. You can also run it by hand from the Actions tab. To turn it on, go to **Settings → Pages** in the repo
and set **Source** to **GitHub Actions**. The site is then served at `https://shaysingh818.github.io/flashi/`.

The workflow sets the base path for you. To build for a sub-path locally, set `SITE_BASE`:

```sh
SITE_BASE=/flashi/ npm run build
```
