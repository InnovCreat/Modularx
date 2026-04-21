// user — Logique principale. Reçoit des messages, retourne des états.

import type { User, UserRole } from "./types";

export function createUser(id: string, name: string, email: string, role: UserRole): User {
  return Object.freeze({ id, name, email, role, joinedAt: Date.now() });
}

export function canAdminister(user: User): boolean {
  return user.role === "admin";
}

export function canContribute(user: User): boolean {
  return user.role === "admin" || user.role === "member";
}
