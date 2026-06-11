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
