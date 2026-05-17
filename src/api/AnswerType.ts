import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';


export interface AnswerType {
  id: number;
  name: string;
  created: string; 
  updated: string;
}

export class AnswerType {
  id: number;
  name: string;
  created: string; 
  updated: string;

  constructor({
    id,
    name,
    created,
    updated
  }: {
    id: number;
    name: string;
    created: string;
    updated: string;
  }) {
    this.id = id;
    this.name = name;
    this.created = created;
    this.updated = updated;
  }
}

export const answerTypeStore = reactive({

  answerTypes: [] as AnswerType[],

  async getDbPath(): Promise<string> {    
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async loadAnswerTypes() {
    try { 
      const dbPath = await this.getDbPath();
      const answerTypes = await invoke<AnswerType[]>('view_answer_types', { dbPath });
      this.answerTypes = answerTypes;
      console.log(answerTypes); 
    } catch (error) {
      console.error("Error loading answer types: ", error);
    }
  },

  async viewAnswerType(answerTypeId: string): Promise<AnswerType> {
    const dbPath = await this.getDbPath();
    return await invoke("view_answer_type", { dbPath, answerTypeId });
  },

})