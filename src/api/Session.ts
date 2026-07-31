import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';
import { Deck, Session, SessionReview, SessionReviewQuestion } from './types';


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
  current_question: 0 as number,

  async getDbPath(): Promise<string> {
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async loadSessionReviews(sessionId: string) {
    try {
      const dbPath = await this.getDbPath();
      const session_reviews = await invoke<Session[]>('view_session_reviews', { dbPath, sessionId });
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
      const questions = await invoke<Session[]>('view_session_review_questions', { dbPath, sessionReviewId });
      this.session_review_questions = questions;
    } catch (error) {
      console.error("Error loading session review questions: ", error);
    }
  },

  async loadActiveSessionReviewQuestions(sessionId: string) {
    try {
      const dbPath = await this.getDbPath();
      const questions = await invoke<Session[]>('view_active_session_review_questions', { dbPath, sessionId });
      this.session_review_questions = questions;
    } catch (error) {
      console.error("Error loading session review questions: ", error);
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

  async nextQuestion(sessionId: string): Promise<SessionReviewQuestion> {
    if (this.current_question === this.session_review_questions.length - 1) {
      console.log("End of question bank, going back to start point.."); 
      const lastQuestion = this.session_review_questions[this.current_question];
      this.current_question = 0;
      await this.loadActiveSessionReviewQuestions(sessionId);
      if (this.session_review_questions.length === 0) {
        return lastQuestion;
      }
      return this.session_review_questions[this.current_question];
    } else {
      console.log("Still more questions to go through.."); 
      console.log(this.session_review_questions); 
      this.current_question += 1;
      console.log(this.session_review_questions[this.current_question]); 
      return this.session_review_questions[this.current_question];
    }
  },

})
