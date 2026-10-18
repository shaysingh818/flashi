import { computed, onMounted, ref } from "vue";
import { LATEST_RELEASE_API, RELEASES_URL } from "../config";

export type Platform = "windows" | "macos" | "linux";

export interface ReleaseAsset {
  name: string;
  browser_download_url: string;
  size: number;
}

export interface DownloadOption {
  label: string;
  url: string;
  size: number;
}

interface Release {
  tag_name: string;
  html_url: string;
  published_at: string;
  assets: ReleaseAsset[];
}

// Tauri bundle file extensions, in the order we prefer to offer them per platform.
const PLATFORM_FORMATS: Record<Platform, { ext: string; label: string }[]> = {
  windows: [
    { ext: "-setup.exe", label: "Installer (.exe)" },
    { ext: ".msi", label: "MSI package (.msi)" },
  ],
  macos: [{ ext: ".dmg", label: "Disk image (.dmg)" }],
  linux: [
    { ext: ".AppImage", label: "AppImage" },
    { ext: ".deb", label: "Debian / Ubuntu (.deb)" },
    { ext: ".rpm", label: "Fedora / RHEL (.rpm)" },
  ],
};

function architectureLabel(name: string): string {
  if (/aarch64|arm64/i.test(name)) return "Apple Silicon / ARM";
  if (/x64|x86_64|amd64/i.test(name)) return "x64";
  return "";
}

export function detectPlatform(): Platform | null {
  const ua = navigator.userAgent.toLowerCase();
  if (ua.includes("win")) return "windows";
  if (ua.includes("mac")) return "macos";
  if (ua.includes("linux") && !ua.includes("android")) return "linux";
  return null;
}

export function formatSize(bytes: number): string {
  if (!bytes) return "";
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

export function useLatestRelease() {
  const release = ref<Release | null>(null);
  const loading = ref(true);
  const error = ref("");

  async function load() {
    loading.value = true;
    error.value = "";
    try {
      const res = await fetch(LATEST_RELEASE_API, {
        headers: { Accept: "application/vnd.github+json" },
      });
      if (!res.ok) throw new Error(`GitHub responded with ${res.status}`);
      release.value = await res.json();
    } catch (e) {
      error.value = String(e);
      release.value = null;
    } finally {
      loading.value = false;
    }
  }

  function optionsFor(platform: Platform): DownloadOption[] {
    const assets = release.value?.assets ?? [];
    const options: DownloadOption[] = [];
    for (const format of PLATFORM_FORMATS[platform]) {
      for (const asset of assets) {
        if (!asset.name.endsWith(format.ext)) continue;
        const arch = architectureLabel(asset.name);
        options.push({
          label: arch ? `${format.label} · ${arch}` : format.label,
          url: asset.browser_download_url,
          size: asset.size,
        });
      }
    }
    return options;
  }

  const version = computed(() => release.value?.tag_name ?? "");
  const releaseUrl = computed(() => release.value?.html_url ?? RELEASES_URL);

  onMounted(load);

  return { release, loading, error, version, releaseUrl, optionsFor, reload: load };
}
