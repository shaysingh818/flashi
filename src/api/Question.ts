import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';


export interface Question {
  id: number;
  deck_id: number;
  answer_type_id: number;
  answer_id: number;
  reviewed: number;
  raw: string;
  created: string; 
  updated: string;
}

export class Question {
    
  id: number;
  deck_id: number;
  answer_type_id: number;
  answer_id: number;
  reviewed: number;
  raw: string;
  created: string; 
  updated: string;

  constructor({ 
    id,
    deck_id,
    answer_type_id,
    answer_id,
    reviewed,
    raw,
    created,
    updated
  }: {       
    id: number;
    deck_id: number;
    answer_type_id: number;
    answer_id: number;
    reviewed: number;
    raw: string;
    created: string; 
    updated: string;
  }) {
    this.id = id;
    this.deck_id = deck_id;
    this.answer_type_id = answer_type_id;
    this.answer_id = answer_id;
    this.reviewed = reviewed;
    this.raw = raw;
    this.created = created;
    this.updated = updated;
  }
}

export const questionStore = reactive({

  questions: [] as Question[],
  currQuestion: 0 as number,

  getCurrQuestion(): number {
    return this.currQuestion;
  },

  cyclePreviousQuestion() {

    if(this.currQuestion == 0) {
      this.currQuestion = this.questions.length - 1;
    } else {
      this.currQuestion -= 1;
    }

  },

  cycleNextQuestion() {

    if(this.currQuestion == this.questions.length - 1) {
      this.currQuestion = 0;
    } else {
      this.currQuestion += 1;
    }

  },

  async getDbPath(): Promise<string> {    
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async loadQuestions(deckId: string) {
    try { 
      const dbPath = await this.getDbPath();
      const questions = await invoke<Question[]>('view_questions', { dbPath, deckId});
      this.questions = questions;
    } catch (error) {
      console.error("Error loading questions: ", error);
    }
  },

  async viewQuestion(questionId: string): Promise<Question> {
    const dbPath = await this.getDbPath();
    return await invoke("view_question", { dbPath, questionId });
  },

  async createQuestion(deckId: string, answerTypeId: string, answerId: string, raw: string): Promise<string> {
    const dbPath = await this.getDbPath();
    return await invoke("create_question", { deckId, answerTypeId, answerId, raw, dbPath });
  },

  async editQuestion(id: string, raw: string): Promise<string> {        
    const dbPath = await this.getDbPath();
    return await invoke("edit_question", { id, raw, dbPath });  
  },

  async deleteQuestion(questionId: string): Promise<string> {    
    const dbPath = await this.getDbPath();    
    return await invoke("delete_question", { dbPath, questionId });  
  },
 
})
