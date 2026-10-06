import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';
import { MultipleChoice, MultipleChoiceAnswer } from './types';

export const multipleChoiceAnswerStore = reactive({

  multipleChoiceAnswers: [] as MultipleChoiceAnswer[],

  async getDbPath(): Promise<string> {    
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async loadMultipleChoiceAnswers(answerId: string) {
    try { 
      const dbPath = await this.getDbPath();
      const answers = await invoke<MultipleChoiceAnswer[]>('view_multiple_choice_answers', { answerId, dbPath });
      this.multipleChoiceAnswers = answers;
    } catch (error) {
      console.error("Error loading multiple choice answers: ", error);
    }
  },

  async viewMultipleChoiceAnswer(id: string): Promise<MultipleChoiceAnswer> {
    const dbPath = await this.getDbPath();
    return await invoke("view_multiple_choice_answer", { dbPath, id });
  },

  async createMultipleChoiceAnswer(answerId: string, answer: string, isCorrect: boolean): Promise<MultipleChoiceAnswer> {
    const dbPath = await this.getDbPath();
    return await invoke("create_multiple_choice_answer", { answerId, answer, isCorrect, dbPath });
  },

  async editMultipleChoiceAnswer(id: string, answerId: string, answer: string, isCorrect: boolean): Promise<string> {        
    const dbPath = await this.getDbPath();
    return await invoke("edit_multiple_choice_answer", { id, answerId, answer, isCorrect, dbPath });  
  },

  async deleteMultipleChoiceAnswer(id: string): Promise<string> {    
    const dbPath = await this.getDbPath();    
    return await invoke("delete_multiple_choice_answer", { dbPath, id });  
  },
 
})

export const multipleChoiceStore = reactive({

  multipleChoices: [] as MultipleChoice[],

  async getDbPath(): Promise<string> {    
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async viewMultipleChoice(id: string): Promise<MultipleChoice> {
    const dbPath = await this.getDbPath();
    return await invoke("view_multiple_choice", { dbPath, id });
  },

  async createMultipleChoice(explanation: string): Promise<MultipleChoiceAnswer> {
    const dbPath = await this.getDbPath();
    return await invoke("create_multiple_choice", { dbPath, explanation });
  },

  async deleteMultipleChoice(id: string): Promise<string> {    
    const dbPath = await this.getDbPath();    
    return await invoke("delete_multiple_choice", { dbPath, id });  
  },

  async validateMultipleChoice(id: string, answer: string[]): Promise<[boolean, string]> {    
    const dbPath = await this.getDbPath();    
    return await invoke("validate_multiple_choice_answer", { dbPath, id, answer });  
  },
  
})
