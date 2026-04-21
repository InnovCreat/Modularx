// Alchem — Bus de messages central
// Communication unidirectionnelle. Jamais d'appels directs entre modules.

import type { Message, MessageType } from "./messages";

type Handler = (message: Message) => void;

const handlers = new Map<MessageType, Handler[]>();

export const MessageBus = Object.freeze({

  on(type: MessageType, handler: Handler): void {
    const existing = handlers.get(type) ?? [];
    handlers.set(type, [...existing, handler]);
  },

  emit(message: Message): void {
    const targets = handlers.get(message.type) ?? [];
    targets.forEach(h => h(message));
  },

  off(type: MessageType, handler: Handler): void {
    const existing = handlers.get(type) ?? [];
    handlers.set(type, existing.filter(h => h !== handler));
  },

});
