// Bus de messages central d'Alchem
// Toute communication inter-modules passe ici. Jamais d'appels directs.

export type MessageType =
  | "MODULE_READY"
  | "MODULE_ERROR"
  | "CONSCIENCE_QUERY"
  | "CONSCIENCE_RESPONSE"
  | "SYSTEM_EVENT";

export interface Message {
  id:        string;
  type:      MessageType;
  source:    string;
  payload:   unknown;
  timestamp: number;
}

export interface ConscienceQuery extends Message {
  type:    "CONSCIENCE_QUERY";
  payload: { question: string };
}

export interface ConscienceResponse extends Message {
  type:    "CONSCIENCE_RESPONSE";
  payload: { guidance: string; covenantKey: string };
}
