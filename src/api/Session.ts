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
