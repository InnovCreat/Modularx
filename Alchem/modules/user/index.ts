// user — Interface publique. Exports uniquement.

export type { User, UserRole, UserPreferences } from "./types";
export type { UserMessage, UserMessageType }     from "./messages";
export { createUser, canAdminister, canContribute } from "./core";
