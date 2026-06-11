export interface Question {
  id: number;
  deck_id: number;
  answer_type_id: number;
  answer_id: number;
  reviewed: number;
  raw: string;
  created: string; 
  updated: string;
}

export class Question {
    
  id: number;
  deck_id: number;
  answer_type_id: number;
  answer_id: number;
  reviewed: number;
  raw: string;
  created: string; 
  updated: string;

  constructor({ 
    id,
    deck_id,
    answer_type_id,
    answer_id,
    reviewed,
    raw,
    created,
    updated
  }: {       
    id: number;
    deck_id: number;
    answer_type_id: number;
    answer_id: number;
    reviewed: number;
    raw: string;
    created: string; 
    updated: string;
  }) {
    this.id = id;
    this.deck_id = deck_id;
    this.answer_type_id = answer_type_id;
    this.answer_id = answer_id;
    this.reviewed = reviewed;
    this.raw = raw;
    this.created = created;
    this.updated = updated;
  }
}
