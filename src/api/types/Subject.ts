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
