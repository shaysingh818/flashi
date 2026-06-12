import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';
import { Deck, Session, SessionReview, SessionReviewQuestion } from './types'; 


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

  async loadSessionReviewQuestions(sessionReviewId: string) {
    try { 
      const dbPath = await this.getDbPath();
      const questions = await invoke<Session[]>('view_session_review_questions', { dbPath, sessionReviewId });
      this.session_review_questions = questions;
    } catch (error) {
      console.error("Error loading session review questions: ", error);
    }
  },

  async viewSessionReviewQuestion(id: string) {
    const dbPath = await this.getDbPath();
    return await invoke("view_session_review_question", { dbPath, id });
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
      EndTime,
      dbPath 
    });  
  },

  async nextQuestion(): Promise<SessionReviewQuestion> {
    console.log(this.current_question);
    console.log(this.session_review_questions.length); 
    if(this.current_question === this.session_review_questions.length-1) {
      this.current_question = 0;
      return this.session_review_questions[this.current_question]; 
    } else {
      this.current_question += 1;
      return this.session_review_questions[this.current_question];
    }
  },
  
})
