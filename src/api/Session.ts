import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';
import { Deck } from './Deck'; 


export interface Session {
  id: number;
  name: string;
  description: string;
  created: string; 
  updated: string;
}

export class Session {
  id: number;
  name: string;
  description: string;
  created: string;
  updated: string;

  constructor({
    id,
    name,
    description,
    created,
    updated
  }: {
    id: number;
    name: string;
    description: string;
    created: string;
    updated: string;
  }) {
    this.id = id;
    this.name = name;
    this.description = description;
    this.created = created;
    this.updated = updated;
  }
}


export interface SessionReview {
  id: number;
  session_id: number;
  complete: boolean;
  cycle: number;
  current_question: number; 
  start_time: string;
  end_time: string; 
  created: string;
  updated: string;
}

export class SessionReview {
  id: number;
  session_id: number;
  complete: boolean;
  cycle: number;
  current_question: number; 
  start_time: string;
  end_time: string; 
  created: string;
  updated: string;

  constructor({
    id,
    session_id,
    complete,
    cycle,
    current_question,
    start_time,
    end_time,
    created,
    updated
  }: {
    id: number;
    session_id: number;
    complete: boolean;
    cycle: number;
    current_question: number; 
    start_time: string;
    end_time: string; 
    created: string;
    updated: string;

  }) {
    this.id = id;
    this.session_id = session_id;
    this.complete = complete;
    this.cycle = cycle;
    this.current_question = current_question; 
    this.start_time = start_time;
    this.end_time = end_time;
    this.created = created;
    this.updated = updated;
  }
}


export interface SessionReviewQuestion {
  id: number;
  session_review_id: number;
  question_id: number ;
  priority_index: number;
  incorrect_count: number; 
  review_count: string;
  active: boolean; 
  created: string;
  updated: string;
}

export class SessionReviewQuestion {

  id: number;
  session_review_id: number;
  question_id: number ;
  priority_index: number;
  incorrect_count: number; 
  review_count: string;
  active: boolean; 
  created: string;
  updated: string;

  constructor({
    id,
    session_review_id,
    question_id,
    priority_index,
    incorrect_count,
    review_count,
    active,
    created,
    updated
  }: {
    id: number;
    session_review_id: number;
    question_id: number ;
    priority_index: number;
    incorrect_count: number; 
    review_count: string;
    active: boolean; 
    created: string;
    updated: string;


  }) {
    this.id = id;
    this.session_review_id = session_review_id;
    this.question_id = question_id;
    this.priority_index = priority_index;
    this.incorrect_count = incorrect_count; 
    this.review_count = review_count;
    this.active = active;
    this.created = created;
    this.updated = updated;
  }
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
  
})
