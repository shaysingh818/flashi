import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';


export interface MultipleChoiceAnswer {
  answer_id: number;
  is_correct: boolean;
  answer: string;
  explanation: string;
  created: string; 
  updated: string;
}

export class MultipleChoiceAnswer {
  answer_id: number;
  is_correct: boolean;
  answer: string;
  explanation: string;
  created: string; 
  updated: string;

  constructor({
    answer_id,
    is_correct,
    answer,
    explanation,
    created,
    updated
  }: {
    answer_id: number;
    is_correct: boolean;
    answer: string;
    explanation: string;
    created: string; 
    updated: string;
  }) {
    this.answer_id = answer_id;
    this.is_correct = is_correct;
    this.answer = answer;
    this.explanation = explanation;
    this.created = created;
    this.updated = updated;
  }
}

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

  async createMultipleChoiceAnswer(answerId: string, answer: string, explanation: string, isCorrect: boolean): Promise<MultipleChoiceAnswer> {
    const dbPath = await this.getDbPath();
    return await invoke("create_multiple_choice_answer", { answerId, answer, explanation, isCorrect, dbPath });
  },

  async editMultipleChoiceAnswer(id: string, answerId: string, answer: string, explanation: string, isCorrect: boolean): Promise<string> {        
    const dbPath = await this.getDbPath();
    return await invoke("edit_multiple_choice_answer", { id, answerId, answer, explanation, isCorrect, dbPath });  
  },

  async deleteMultipleChoiceAnswer(id: string): Promise<string> {    
    const dbPath = await this.getDbPath();    
    return await invoke("delete_multiple_choice_answer", { dbPath, id });  
  },
 
})


export interface MultipleChoice {
  answer_id: number;
  created: string; 
  updated: string;
}

export class MultipleChoice {
  answer_id: number;
  created: string; 
  updated: string;

  constructor({
    answer_id,
    created,
    updated
  }: {
    answer_id: number;
    created: string; 
    updated: string;
  }) {
    this.answer_id = answer_id;
    this.created = created;
    this.updated = updated;
  }
}


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

  async createMultipleChoice(): Promise<MultipleChoiceAnswer> {
    const dbPath = await this.getDbPath();
    return await invoke("create_multiple_choice", { dbPath });
  },

  async deleteMultipleChoice(id: string): Promise<string> {    
    const dbPath = await this.getDbPath();    
    return await invoke("delete_multiple_choice", { dbPath, id });  
  },
 
})
