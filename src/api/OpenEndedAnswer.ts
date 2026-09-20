import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';
import { OpenEndedAnswer, SessionReviewQuestion } from './types';
import { sessionReviewStore } from './Session';

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

  async validateOpenEndedAnswer(id: string, answer: string): Promise<[boolean, string]> {    
    const dbPath = await this.getDbPath();    
    return await invoke("validate_open_ended_answer", { dbPath, id, answer });  
  },

  async completeOpenEndedAnswerMetadata(    
    answerId: string,
    sessionQuestion: SessionReviewQuestion,
    openEndedAnswerValue: string) {
 
    const [result, msg] = await  this.validateOpenEndedAnswer(
      answerId,
      openEndedAnswerValue
    );

    if (result === true) {
      
      await sessionReviewStore.editSessionReviewQuestion(
        `${sessionQuestion.id}`,
        sessionQuestion.priority_index,
        parseInt(sessionQuestion.review_count) + 1, 
        sessionQuestion.incorrect_count,
        false
      );
        
    } else {
      
      await sessionReviewStore.editSessionReviewQuestion(
        `${sessionQuestion.id}`,
        sessionQuestion.priority_index,
        parseInt(sessionQuestion.review_count) + 1,
        sessionQuestion.incorrect_count + 1,
        true
      );
      
    }

    return [result, msg];
  },
 
})
