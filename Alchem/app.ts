// Alchem — Entry point
// Triggers only. No logic here.

import "./core/rules/alchem";
import "./core/ethics/covenant";
import "./core/messages";

// System bootstrap trigger
export { ALCHEM_PRINCIPLES } from "./core/rules/alchem";
export { COVENANT }          from "./core/ethics/covenant";
export type { Message }      from "./core/messages";
