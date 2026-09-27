import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';
import { Deck } from './types';


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

  async editDeck(deckId: string, name: string, description: string, tags: string[], reviewed: number): Promise<string> {        
    const dbPath = await this.getDbPath();
    return await invoke("edit_deck", { deckId, name, description, tags, reviewed, dbPath });  
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
  }, 

  async exportDeck(deckId: string): Promise<string> {    
    const dbPath = await this.getDbPath();   
    const path = "/home/shay-gaming/Documents/projects/desktop_applications";
    return await invoke("export_deck", { dbPath, deckId, path });  
  },

  async importDeck(subjectId: string, path: string): Promise<string> {    
    const dbPath = await this.getDbPath();    
    return await invoke("import_deck", { dbPath, subjectId, path });  
  },
  
})
