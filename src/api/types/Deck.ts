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
