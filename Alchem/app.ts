// Alchem — Entry point
// Triggers only. No logic here.

import "./core/environment";
import "./core/rules/alchem";
import "./core/ethics/covenant";
import "./core/messages";
import "./core/message-bus";
import "./core/classification/dimensions";
import "./core/classification/categories";
import "./core/classification/conventions";
import "./core/classification/examples";

// Framework exports
export { ENV }                  from "./core/environment";
export { ALCHEM_PRINCIPLES }    from "./core/rules/alchem";
export type { AlchemPrinciple } from "./core/rules/alchem";
export { COVENANT }             from "./core/ethics/covenant";
export type { CovenantKey }     from "./core/ethics/covenant";
export type { Message }         from "./core/messages";
export { MessageBus }           from "./core/message-bus";
export { SETTINGS }             from "./config/settings";

// Classification exports
export { DIMENSIONS }           from "./core/classification/dimensions";
export type { DimensionKey, AlchemType } from "./core/classification/dimensions";
export { CATEGORIES }           from "./core/classification/categories";
export type { CategoryKey, CategoryType } from "./core/classification/categories";
export { FILE_CONVENTIONS, MODULE_FILES } from "./core/classification/conventions";
export type { ConventionKey, ConventionSuffix } from "./core/classification/conventions";
export {
  IDENTITY_TERMS,
  ACTION_TERMS,
  CAPACITY_TERMS,
  SPACE_TERMS,
  CONTROL_TERMS,
} from "./core/classification/examples";
