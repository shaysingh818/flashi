import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';
import { sessionReviewStore, getCurrentTimestamp } from './Session'; 
import { OrderedSequenceAnswer, SessionReview, SessionReviewQuestion } from './types';

export const orderedSequenceStore = reactive({

  orderedSequenceAnswers: [] as OrderedSequenceAnswer[],

  async getDbPath(): Promise<string> {    
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async loadOrderedSequenceAnswers() {
    try { 
      const dbPath = await this.getDbPath();
      const answers = await invoke<OrderedSequenceAnswer[]>('view_ordered_sequence_answers', { dbPath });
      this.orderedSequenceAnswers = answers;
    } catch (error) {
      console.error("Error loading ordered sequence answers: ", error);
    }
  },

  async viewOrderedSequenceAnswer(id: string): Promise<OrderedSequenceAnswer> {
    const dbPath = await this.getDbPath();
    return await invoke("view_ordered_sequence_answer", { dbPath, id });
  },

  async createOrderedSequenceAnswer(sequence: string[], explanation: string): Promise<OrderedSequenceAnswer> {
    const dbPath = await this.getDbPath();
    return await invoke("create_ordered_sequence_answer", { sequence, explanation, dbPath });
  },

  async editOrderedSequenceAnswer(id: string, sequence: string[], explanation: string): Promise<string> {        
    const dbPath = await this.getDbPath();
    return await invoke("edit_ordered_sequence_answer", { id, sequence, explanation, dbPath });  
  },

  async deleteOrderedSequenceAnswer(id: string): Promise<string> {    
    const dbPath = await this.getDbPath();    
    return await invoke("delete_ordered_sequence_answer", { dbPath, id });  
  },

  async validateOrderedSequenceAnswer(id: string, answer: string[]): Promise<[boolean, string]> {    
    const dbPath = await this.getDbPath();    
    return await invoke("validate_ordered_sequence_answer", { dbPath, id, answer });  
  },
 
})
