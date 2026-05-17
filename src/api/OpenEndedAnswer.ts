import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';


export interface OpenEndedAnswer {
  answer_id: number;
  answer: string;
  explanation: string;
  keywords: string[];
  created: string; 
  updated: string;
}

export class OpenEndedAnswer {
  
  answer_id: number;
  answer: string;  
  explanation: string;
  keywords: string[];
  created: string; 
  updated: string;

  constructor({
    answer_id,
    answer,  
    explanation,
    keywords,
    created, 
    updated
    
  }: {    
    answer_id: number;
    answer: string;  
    explanation: string;
    keywords: string[];
    created: string; 
    updated: string; 
  }) {
    this.answer_id = answer_id;
    this.answer = answer;
    this.explanation = explanation;
    this.keywords = keywords;
    this.created = created;
    this.updated = updated;
  }
}

export const openEndedStore = reactive({

  openEndedAnswers: [] as OpenEndedAnswer[],

  async getDbPath(): Promise<string> {    
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async loadOpenEndedAnswers() {
    try { 
      const dbPath = await this.getDbPath();
      const answers = await invoke<OpenEndedAnswer[]>('view_open_ended_answers', { dbPath });
      this.openEndedAnswers = answers;
    } catch (error) {
      console.error("Error loading ordered sequence answers: ", error);
    }
  },

  async viewOpenEndedAnswer(id: string): Promise<OpenEndedAnswer> {
    const dbPath = await this.getDbPath();
    return await invoke("view_open_ended_answer", { dbPath, id });
  },

  async createOpenEndedAnswer(answer: string, explanation: string, keywords: string[]): Promise<OpenEndedAnswer> {
    const dbPath = await this.getDbPath();
    return await invoke("create_open_ended_answer", { answer, explanation, keywords, dbPath });
  },

  async editOpenEndedAnswer(id: string, answer: string, explanation: string, keywords: string[]): Promise<string> {        
    const dbPath = await this.getDbPath();
    return await invoke("edit_open_ended_answer", { id, answer, explanation, keywords, dbPath });  
  },

  async deleteOpenEndedAnswer(id: string): Promise<string> {    
    const dbPath = await this.getDbPath();    
    return await invoke("delete_open_ended_answer", { dbPath, id });  
  },
 
})