import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';
import { openEndedStore } from './OpenEndedAnswer';
import { questionStore } from "./Question";
import { multipleChoiceStore } from "./MutlipleChoiceAnswer";
import { orderedSequenceStore } from "./OrderedSequenceAnswer";
import { Deck, Session, SessionReview, SessionReviewQuestion, SessionReviewMetrics } from './types';


export function getCurrentTimestamp(): string {
  const now = new Date();

  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  const hours = String(now.getHours()).padStart(2, '0');
  const minutes = String(now.getMinutes()).padStart(2, '0');
  const seconds = String(now.getSeconds()).padStart(2, '0');

  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}

export const sessionStore = reactive({

  sessions: [] as Session[],

  async getDbPath(): Promise<string> {
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async loadSessions() {
    try {
      const dbPath = await this.getDbPath();
      const sessions = await invoke<Session[]>('view_sessions', { dbPath });
      this.sessions = sessions;
    } catch (error) {
      console.error("Error loading sessions: ", error);
    }
  },

  async viewSession(sessionId: string): Promise<Session> {
    const dbPath = await this.getDbPath();
    return await invoke("view_session", { dbPath, sessionId });
  },

  async createSession(name: string, description: string): Promise<string> {
    const dbPath = await this.getDbPath();
    return await invoke("create_session", { name, description, dbPath });
  },

  async editSession(sessionId: string, name: string, description: string): Promise<string> {
    const dbPath = await this.getDbPath();
    return await invoke("edit_session", { sessionId, name, description, dbPath });
  },

  async deleteSession(sessionId: string): Promise<string> {
    const dbPath = await this.getDbPath();
    return await invoke("delete_session", { dbPath, sessionId });
  },

  async addDeckToSession(sessionId: string, deckId: string): Promise<string> {
    const dbPath = await this.getDbPath();
    return await invoke("add_deck_to_session", { dbPath, sessionId, deckId });
  },

  async removeDeckFromSession(sessionId: string, deckId: string): Promise<string> {
    const dbPath = await this.getDbPath();
    return await invoke("remove_deck_from_session", { dbPath, sessionId, deckId });
  },

  async viewSessionDecks(sessionId: string): Promise<Deck[]> {
    const dbPath = await this.getDbPath();
    return await invoke("view_session_decks", { dbPath, sessionId });
  },

  async viewSessionProgressStatus(sessionId: string): Promise<[number, number]> {    
    const dbPath = await this.getDbPath();
    return await invoke("session_progress_status", { dbPath, sessionId });
  },

  async searchSessions(searchTerm: string) {
    try {
      const dbPath = await this.getDbPath();
      const sessions = await invoke<Session[]>("search_sessions", { dbPath, searchTerm });
      this.sessions = sessions;
    } catch (error) {
      console.error("Error loading sessions from search term: ", error);
    }
  }

})


export const sessionReviewStore = reactive({

  session_reviews: [] as SessionReview[],
  session_review_questions: [] as SessionReviewQuestion[],
  session_review_metrics: [] as SessionReviewMetrics[],
  current_question: 0 as number,

  async getDbPath(): Promise<string> {
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async loadSessionReviews(sessionId: string) {
    try {
      const dbPath = await this.getDbPath();
      const session_reviews = await invoke<SessionReview[]>('view_session_reviews', { dbPath, sessionId });
      this.session_reviews = session_reviews;
    } catch (error) {
      console.error("Error loading session reviews: ", error);
    }
  },

  async viewSessionReview(sessionReviewId: string) {
    const dbPath = await this.getDbPath();
    return await invoke("view_session_review", { dbPath, sessionReviewId });
  },

  async createSessionReview(sessionId: string): Promise<SessionReview> {
    const dbPath = await this.getDbPath();
    return await invoke("create_session_review", { sessionId, dbPath });
  },

  async editSession(
    sessionReviewId: string,
    complete: boolean,
    cycle: number,
    currentQuestion: number,
    startTime: string,
    endTime: string): Promise<string> {

    const dbPath = await this.getDbPath();
    return await invoke("edit_session_review", {
      sessionReviewId,
      complete,
      cycle,
      currentQuestion,
      startTime,
      endTime,
      dbPath
    });
  },

  async checkSessionReviewActive(sessionId: string): Promise<number> {
    const dbPath = await this.getDbPath();
    return await invoke("check_session_review_active", { dbPath, sessionId });
  },

  async checkSessionReviewComplete(sessionReviewId: string): Promise<boolean> {
    const dbPath = await this.getDbPath();
    return await invoke("check_session_review_complete", { dbPath, sessionReviewId });
  },

  async loadSessionReviewQuestions(sessionReviewId: string) {
    try {
      const dbPath = await this.getDbPath();
      const questions = await invoke<SessionReviewQuestion[]>('view_session_review_questions', { dbPath, sessionReviewId });
      this.session_review_questions = questions;
    } catch (error) {
      console.error("Error loading session review questions: ", error);
    }
  },

  async loadActiveSessionReviewQuestions(sessionId: string) {
    try {
      const dbPath = await this.getDbPath();
      const questions = await invoke<SessionReviewQuestion[]>('view_active_session_review_questions', { dbPath, sessionId });
      this.session_review_questions = questions;
    } catch (error) {
      console.error("Error loading session review questions: ", error);
    }
  },

  async loadSessionReviewMetricsHistory(sessionId: string) {   
    try { 
      const dbPath = await this.getDbPath();
      const sessionMetrics = await invoke<SessionReviewMetrics[]>('retrieve_session_metrics_history', { dbPath, sessionId });
      this.session_review_metrics = sessionMetrics;
      console.log(this.session_review_metrics)
    } catch (error) {      
      console.error("Error loading session review metrics: ", error);
    }
  },
  
  async viewSessionReviewQuestion(id: string) {
    const dbPath = await this.getDbPath();
    return await invoke("view_session_review_question", { dbPath, id });
  },

  async editSessionReviewQuestion(
    id: string,
    priorityIndex: number,
    reviewCount: number,
    incorrectCount: number,
    active: boolean): Promise<string> {

    const dbPath = await this.getDbPath();
    return await invoke("edit_session_review_question", {
      dbPath,
      id,
      priorityIndex,
      reviewCount,
      incorrectCount,
      active,
    });
  },

  async completeAnswerMetadata(
    answerId: string,
    sessionQuestion: SessionReviewQuestion,
    openEndedAnswerValue?: string,
    submittedMultipleChoiceAnswersValue?: string[],
    submittedOrderedSequenceValue?: string[]): Promise<[boolean, string]> {

    let result: boolean = false;
    let msg: string = '';

    if (openEndedAnswerValue) {

      [result, msg] = await  openEndedStore.validateOpenEndedAnswer(
        answerId,
        openEndedAnswerValue
      );

    } else if (submittedMultipleChoiceAnswersValue) {

      [result, msg] = await  multipleChoiceStore.validateMultipleChoice(
        answerId,
        submittedMultipleChoiceAnswersValue.map(item => item.id.toString())
      );

    } else if (submittedOrderedSequenceValue) {
      [result, msg] = await orderedSequenceStore.validateOrderedSequenceAnswer(
        answerId,
        submittedOrderedSequenceValue.map(item => item.item.toString())
      );

    }

    if (result === true) {
      
      const editResult = await sessionReviewStore.editSessionReviewQuestion(
        `${sessionQuestion.id}`,
        sessionQuestion.priority_index,
        parseInt(sessionQuestion.review_count) + 1,
        sessionQuestion.incorrect_count,
        false
      );

      if (editResult === "Success") {
        console.log("Updated session review question with metadata for correct answer...")
      } else {
        console.error("Could not update session review question metadata for correct answer: ", editResult); 
      }

      const currQuestion = await questionStore.viewQuestion(`${sessionQuestion.question_id}`);
      const questionResult = await questionStore.editQuestion(
        `${sessionQuestion.question_id}`,
        currQuestion.raw,
        currQuestion.reviewed + 1,
        currQuestion.correct + 1,
        currQuestion.incorrect
      ); 
      
      if (questionResult === "Success") {
        console.log("Updated question entity with metadata for correct answer...")
      } else {        
        console.error("Could not update question entity metadata for correct answer: ", questionResult); 
      }
      
    } else {
      const editResult = await sessionReviewStore.editSessionReviewQuestion(
        `${sessionQuestion.id}`,
        sessionQuestion.priority_index,
        parseInt(sessionQuestion.review_count) + 1,
        sessionQuestion.incorrect_count + 1,
        true
      );

      if (editResult === "Success") {
        console.log("Updated question with metadata for incorrect answer...")
      }
    }

    return [result, msg];
  },

  async nextQuestion(sessionId: string, sessionReview: SessionReview): Promise<number> {

    if (this.current_question === this.session_review_questions.length - 1) {

      const cycleResult = await this.editSession(
        `${sessionReview.id}`,
        sessionReview.complete,
        sessionReview.cycle + 1,
        0,
        sessionReview.start_time,
        sessionReview.end_time
      );

      if (cycleResult === "Success") {
        console.log("Updated session cycle count: ", sessionReview.cycle + 1);
      }

      const lastQuestion = this.session_review_questions[this.current_question];
      this.current_question = 0;
      await this.loadActiveSessionReviewQuestions(sessionId);
      if (this.session_review_questions.length === 0) {
        return lastQuestion.id;
      }
      return this.session_review_questions[this.current_question].id;
    } else {
      this.current_question += 1;
      return this.session_review_questions[this.current_question].id;
    }
  },

  async generateSessionReviewMetrics(sessionReviewId: string) {
    const dbPath = await this.getDbPath();
    return await invoke("generate_session_review_metrics", { sessionReviewId, dbPath });
  },

})
