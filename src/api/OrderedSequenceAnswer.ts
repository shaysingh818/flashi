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

  async completeOrderSequenceAnswer(
    answerId: string,
    sessionQuestion: SessionReviewQuestion, 
    sessionReview: SessionReview, 
    submittedOrderedSequenceValue: string[],
    sessionId: string): Promise<[boolean, number | null]> {
    
    let nextQuestion = true;
    let nextId: number | null = null;
    const sequenceAnswers = submittedOrderedSequenceValue.map(item => item.item.toString());

    const [result, msg] = await  this.validateOrderedSequenceAnswer(
      answerId,
      sequenceAnswers
    );

    if (result === true) {
      
      await sessionReviewStore.editSessionReviewQuestion(
        `${sessionQuestion.id}`,
        sessionQuestion.priority_index,
        parseInt(sessionQuestion.review_count) + 1,
        sessionQuestion.incorrect_count,
        false
      );

      const next = await sessionReviewStore.nextQuestion(`${sessionId}`);
      console.log("Next ordered sequence beginning: ", next.id); 
      try {         
        const sessionComplete = await sessionReviewStore.checkSessionReviewComplete(`${sessionReview.id}`);
        if (sessionComplete === true) {
          console.log(`${sessionReview.id}`); 
          await sessionReviewStore.editSession(
            `${sessionReview.id}`,
            true,
            0,
            sessionReview.current_question,
            getCurrentTimestamp(),
            getCurrentTimestamp()
          );        
          console.log("Session complete result: ", result); 
          nextQuestion = false;
        } else {           
          nextId = next.id;
        }
      } catch (err) {
        console.error('Navigation failed:', err);
      }
      
    } else {

      console.log("Incorrect order sequence answer")
      await sessionReviewStore.editSessionReviewQuestion(
        `${sessionQuestion.id}`,
        sessionQuestion.priority_index,
        parseInt(sessionQuestion.review_count) + 1,
        sessionQuestion.incorrect_count + 1,
        true
      );
      
      const next = await sessionReviewStore.nextQuestion(`${sessionId}`);      
      console.log("Next ordered sequence end: ", next.id); 
      nextId = next.id;
    }

    console.log(nextId);
    return [nextQuestion, nextId]
  }
 
})
