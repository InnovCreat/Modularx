// user — Object Data uniquement. Aucune logique.

export interface User {
  readonly id:       string;
  readonly name:     string;
  readonly email:    string;
  readonly role:     UserRole;
  readonly joinedAt: number;
}

export type UserRole = "admin" | "member" | "guest";

export interface UserPreferences {
  readonly theme:    "light" | "dark";
  readonly language: string;
}
