export interface AnswerType {
  id: number;
  name: string;
  created: string; 
  updated: string;
}

export class AnswerType {
  id: number;
  name: string;
  created: string; 
  updated: string;

  constructor({
    id,
    name,
    created,
    updated
  }: {
    id: number;
    name: string;
    created: string;
    updated: string;
  }) {
    this.id = id;
    this.name = name;
    this.created = created;
    this.updated = updated;
  }
}


export interface OpenEndedAnswer {
  answer_id: number;
  answer: string;
  explanation: string;
  keywords: string[];
  created: string; 
  updated: string;
}

export class OpenEndedAnswer {
  
  answer_id: number;
  answer: string;  
  explanation: string;
  keywords: string[];
  created: string; 
  updated: string;

  constructor({
    answer_id,
    answer,  
    explanation,
    keywords,
    created, 
    updated
    
  }: {    
    answer_id: number;
    answer: string;  
    explanation: string;
    keywords: string[];
    created: string; 
    updated: string; 
  }) {
    this.answer_id = answer_id;
    this.answer = answer;
    this.explanation = explanation;
    this.keywords = keywords;
    this.created = created;
    this.updated = updated;
  }
}

export interface OrderedSequenceAnswer {
  answer_id: number;
  sequence: string[];
  explanation: string;
  created: string; 
  updated: string;
}

export class OrderedSequenceAnswer {
  
  answer_id: number;
  sequence: string[];
  explanation: string;
  created: string; 
  updated: string;

  constructor({
    answer_id,
    sequence,
    explanation,
    created, 
    updated
  }: {    
    answer_id: number;
    sequence: string[];
    explanation: string;
    created: string; 
    updated: string;
  }) {
    this.answer_id = answer_id;
    this.sequence = sequence;
    this.explanation = explanation;
    this.created = created;
    this.updated = updated;
  }
}

export interface MultipleChoice {
  answer_id: number;
  created: string; 
  updated: string;
}

export class MultipleChoice {
  answer_id: number;
  created: string; 
  updated: string;

  constructor({
    answer_id,
    created,
    updated
  }: {
    answer_id: number;
    created: string; 
    updated: string;
  }) {
    this.answer_id = answer_id;
    this.created = created;
    this.updated = updated;
  }
}


export interface MultipleChoiceAnswer {
  answer_id: number;
  is_correct: boolean;
  answer: string;
  explanation: string;
  created: string; 
  updated: string;
}

export class MultipleChoiceAnswer {
  answer_id: number;
  is_correct: boolean;
  answer: string;
  explanation: string;
  created: string; 
  updated: string;

  constructor({
    answer_id,
    is_correct,
    answer,
    explanation,
    created,
    updated
  }: {
    answer_id: number;
    is_correct: boolean;
    answer: string;
    explanation: string;
    created: string; 
    updated: string;
  }) {
    this.answer_id = answer_id;
    this.is_correct = is_correct;
    this.answer = answer;
    this.explanation = explanation;
    this.created = created;
    this.updated = updated;
  }
}
