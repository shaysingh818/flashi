import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';
import { Settings } from './types';


export const settingsStore = reactive({

  settings: { export_path: '' } as Settings,

  async getSettingsPath(): Promise<string> {
    const dataDirPath = await dataDir();
    const settingsPath = `${dataDirPath}/flashi/metadata/settings.json`;
    return settingsPath;
  },

  async loadSettings() {
    try {
      const settingsPath = await this.getSettingsPath();
      const settings = await invoke<Settings>('read_settings_file', { settingsPath });
      this.settings = settings;
    } catch (error) {
      console.error("Error loading settings: ", error);
    }
  },

  async saveSettings(settings: Settings) {
    try {
      const settingsPath = await this.getSettingsPath();
      const saved = await invoke<Settings>('write_settings_file', { settings, settingsPath });
      this.settings = saved;
    } catch (error) {
      console.error("Error saving settings: ", error);
    }
  },

})
