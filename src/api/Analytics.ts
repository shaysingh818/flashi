import { reactive } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';

export const analyticsStore = reactive({

  async getDbPath(): Promise<string> {    
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async reviewTimeMetrics(): Promise<[number, number, number]> {
    const dbPath = await this.getDbPath();
    return await invoke("review_time_metrics", { dbPath });
  },

  async questionReviewTimeMetrics(): Promise<[number, number, number]> {
    const dbPath = await this.getDbPath();
    return await invoke("question_review_time_metrics", { dbPath });
  },
  
})