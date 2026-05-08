import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';


export interface Subject {
  id: number;
  name: string;
  description: string;
  created: string; 
  updated: string;
}

export class Subject {
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

export const subjectStore = reactive({

  subjects: [] as Subject[],

  async getDbPath(): Promise<string> {    
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async loadSubjects() {
    try {
      const subjects = await invoke<Subject[]>('view_subjects', {});
      this.subjects = subjects;
    } catch (error) {
      console.error("Error loading subjects: ", error);
    }
  },

  async viewSubject(id: string): Promise<Subject> {
    const dbPath = await this.getDbPath();
    return await invoke("view_subject", { dbPath, id });
  },

  async createSubject(name: string, description: string): Promise<string> {
    const dbPath = await this.getDbPath();
    return await invoke("create_subject", { name, description, dbPath });
  },

  async editSubject(id: string, name: string, description: string): Promise<string> {        
    const dbPath = await this.getDbPath();
    return await invoke("edit_subject", { id, name, description, dbPath });  
  },

  async deleteSubject(id: string): Promise<string> {    
    const dbPath = await this.getDbPath();    
    return await invoke("delete_subject", { dbPath, id });  
  },

  async searchSubjects(searchTerm: string) {
    try { 
      const dbPath = await this.getDbPath();
      const subjects = await invoke<Subject[]>("search_subjects", { dbPath, searchTerm }); 
      this.subjects = subjects;
    } catch (error) {
      console.error("Error loading subjects from search term: ", error); 
    }
  }
  
})