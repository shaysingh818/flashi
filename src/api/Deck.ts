import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';


export interface Deck {
  id: number;
  subject_id: number;
  name: string;
  tags: string[];
  description: string;
  created: string; 
  updated: string;
}

export class Deck {
  id: number;
  subject_id: number;
  name: string;
  tags: string[];
  description: string;
  created: string; 
  updated: string;

  constructor({
    id,
    subject_id,
    name,
    tags,
    description,
    created,
    updated
  }: {
    id: number;
    subject_id: number;
    name: string;
    tags: string[];
    description: string;
    created: string;
    updated: string;
  }) {
    this.id = id;
    this.subject_id = subject_id;
    this.name = name;
    this.tags = tags;
    this.description = description;
    this.created = created;
    this.updated = updated;
  }
}

export const deckStore = reactive({

  decks: [] as Deck[],

  async getDbPath(): Promise<string> {    
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async loadDecks() {
    try { 
      const dbPath = await this.getDbPath();
      const decks = await invoke<Deck[]>('view_decks', { dbPath });
      this.decks = decks;
    } catch (error) {
      console.error("Error loading decks: ", error);
    }
  },

  async loadDecksBySubject(subjectId: string) {
    try { 
      const dbPath = await this.getDbPath();
      const decks = await invoke<Deck[]>('view_decks_by_subject', { dbPath, subjectId });
      this.decks = decks;
    } catch (error) {
      console.error("Error loading decks: ", error);
    }
  },

  async viewDeck(deckId: string): Promise<Deck> {
    const dbPath = await this.getDbPath();
    return await invoke("view_deck", { dbPath, deckId });
  },

  async createDeck(subjectId: string, name: string, tags: string[], description: string): Promise<string> {
    const dbPath = await this.getDbPath();
    return await invoke("create_deck", { subjectId , name, tags, description, dbPath });
  },

  async editDeck(deckId: string, name: string, description: string, tags: string[]): Promise<string> {        
    const dbPath = await this.getDbPath();
    return await invoke("edit_deck", { deckId, name, description, tags, dbPath });  
  },

  async deleteDeck(deckId: string): Promise<string> {    
    const dbPath = await this.getDbPath();    
    return await invoke("delete_deck", { dbPath, deckId });  
  },

  async searchDecks(searchTerm: string) {
    try { 
      const dbPath = await this.getDbPath();
      const decks = await invoke<Deck[]>("search_decks", { dbPath, searchTerm }); 
      this.decks = decks;
    } catch (error) {
      console.error("Error loading decks from search term: ", error); 
    }
  }
  
})
