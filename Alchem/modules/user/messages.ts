// user — Command Data & Messages. Jamais de logique ici.

import type { Message } from "../../core/messages";

export type UserMessageType =
  | "USER_LOGIN"
  | "USER_LOGOUT"
  | "USER_UPDATE_PREFS";

export interface UserLogin extends Message {
  type:    "USER_LOGIN";
  payload: { email: string; password: string };
}

export interface UserLogout extends Message {
  type:    "USER_LOGOUT";
  payload: { userId: string };
}

export interface UserUpdatePrefs extends Message {
  type:    "USER_UPDATE_PREFS";
  payload: { userId: string; prefs: Record<string, unknown> };
}

export type UserMessage = UserLogin | UserLogout | UserUpdatePrefs;
