import { invoke } from "@tauri-apps/api/core";
import { dataDir} from '@tauri-apps/api/path';
import { reactive } from 'vue';
import { Subject } from './types'; 


export const subjectStore = reactive({

  subjects: [] as Subject[],

  async getDbPath(): Promise<string> {    
    const dataDirPath = await dataDir();
    const dbPath = `${dataDirPath}/flashi/metadata/flashi.db`;
    return dbPath;
  },

  async loadSubjects() {
    try { 
      const dbPath = await this.getDbPath();
      const subjects = await invoke<Subject[]>('view_subjects', { dbPath });
      this.subjects = subjects;
    } catch (error) {
      console.error("Error loading subjects: ", error);
    }
  },

  async viewSubject(subjectId: string): Promise<Subject> {
    const dbPath = await this.getDbPath();
    return await invoke("view_subject", { dbPath, subjectId });
  },

  async createSubject(name: string, description: string): Promise<string> {
    const dbPath = await this.getDbPath();
    return await invoke("create_subject", { name, description, dbPath });
  },

  async editSubject(subjectId: string, name: string, description: string): Promise<string> {        
    const dbPath = await this.getDbPath();
    return await invoke("edit_subject", { subjectId, name, description, dbPath });  
  },

  async deleteSubject(subjectId: string): Promise<string> {    
    const dbPath = await this.getDbPath();    
    return await invoke("delete_subject", { dbPath, subjectId });  
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
