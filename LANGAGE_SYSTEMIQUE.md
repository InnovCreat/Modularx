# Langage Systémique

**Document de référence** · v1.3 · Juillet 2026
**Veritas Hortus · Alliance · La Fleur Harmonique**

Un vocabulaire pour décrire, penser et concevoir des systèmes modulaires.

---

## Ce qui change en v1.3

Les six concepts qui étaient « en attente de validation » en v1.2 sont entrés dans le noyau — non par débat, mais en remplissant le Gabarit de Module sur cinq modules réels (Justice Parser, Banc de Formants, NEXUS, POLLINATOR, LUNA). Chacun a trouvé sa case ou son mur. Un journal d'intégration trace où chacun est allé.

Deux découvertes ont émergé du remplissage : le **centre de gravité** du triangle (§III) et la loi des **trois échelles du pourquoi** (§V, qui résout Q0).

---

## Sommaire

I. Origine et intention
II. Le vocabulaire — 10 catégories
III. Le triangle Forme · Lien · Temps
IV. Le spark et la chaîne d'activation
V. Les trois échelles du pourquoi
VI. Gouvernance des règles
VII. Les tests de validation
VIII. Filiations et précédents
IX. Pistes d'évolution — lemniscate et gyroscope
X. Journal d'intégration v1.3
XI. Questions ouvertes

---

## I. Origine et intention

Ce langage est né d'une liste brute de mots — `type`, `module`, `trigger`, `intent`… — sortis d'une intuition : les mêmes concepts reviennent quand on décrit un Component SketchUp, un My Block Scratch, ou un module Rust de Veritas Hortus. Un vocabulaire commun devait exister sous ces trois mondes.

L'intention n'est pas de faire un glossaire. C'est de construire un *langage de description de systèmes* qui serve à trois choses : mieux définir les problèmes, concevoir des systèmes modulaires, et donner un vocabulaire partagé entre le code, la modélisation 3D et la logique visuelle.

**Observation fondatrice** — SketchUp expose la modularité *géométrique* (Definition/Instance, Tags, attributs dynamiques). Scratch expose la modularité *logique* (My Blocks, inputs, messages diffusés). Rust expose la modularité *structurelle* (modules, traits, ownership). Mais aucun des trois n'expose la couche du *sens* — contexte, intention, but. C'est précisément cette couche manquante que le langage systémique cherche à nommer.

---

## II. Le vocabulaire — 10 catégories

Les termes **en gras** sont entrés au noyau en v1.3 ; voir §X pour leur module-témoin.

**01 · Identification & Description** — *l'étiquette sur n'importe quoi*
name · identify · description · annotation · reference · id · label · namespace · signature

**02 · Structure & Modularité** — *la Forme : ce qu'une chose est*
object · module · bloc · class · type · mode · interface · component · instance · definition · hierarchy · layer · node · container · nesting · **automodularité**

**03 · Propriétés & Attributs** — *les valeurs mesurables sur la Forme*
size · weight · volume · masse · couleur · style · position · location · direction · distance · scale · orientation · texture · material · value · capacity · **propriétés du message**

**04 · Actions & Opérations** — *les verbes qui font bouger le triangle*
edit · modify · control · instruction · input · output · execute · process · validate · transform · generate

**05 · Relations & Interactions** — *le Lien : à quoi une chose est connectée*
link · call · answer · message · dependency · binding · parent · child · inherit · **spark** · **bruit** · **intention**

**06 · Comportement & Dynamique** — *le Temps : comment une chose change*
events · motion · force · vitesse · time / temporal · behavior · cycle · transition · duration · frequency · **trigger** · **élan**

**07 · État** — *catégorie indépendante : tous les types d'états*
state · status · initialization · active/inactive · termination
Classification par type : *physique* (actif, caché, verrouillé) · *computationnel* (en cours, en attente, en erreur) · *relationnel* (connecté, isolé, en synchronisation) · *émotionnel / qualitatif* (pour les entités vivantes ou les IA — pertinent pour LUNA).

**08 · Contexte & Logique** — *la couche au-dessus du triangle : le pourquoi*
context · intent · logic · vision · condition · rule · constraint · goal · priority · **motivation profonde**

**09 · Persistance & Fichiers** — *la couche en dessous : comment ça survit*
file · storage · cache · version · backup · snapshot

**10 · Gouvernance des règles** — *où vivent les règles*
règle locale (par nœud) · règle globale (héritée du système) · contrainte

**Décisions de structure** — `motion`, `force`, `vitesse` vivent uniquement en Comportement : ce ne sont pas des propriétés statiques. `parent` / `child` sont des Relations ; `nesting` / `hierarchy` restent en Structure. `reference` vit en Identification uniquement.

---

## III. Le triangle Forme · Lien · Temps

Trois catégories forment le squelette du langage. Chacune répond à une question fondamentale sur n'importe quel module :

- **Forme** — qu'est-ce ?
- **Lien** — connecté à quoi ?
- **Temps** — comment ça change ?

Au centre, le **spark** : le point où Lien et Temps se croisent.

### Comment les autres catégories habillent le triangle

| Catégorie | Position | Rôle |
|---|---|---|
| Identification | Sur n'importe quel sommet | L'étiquette d'un objet, d'un lien ou d'un événement |
| Propriétés | Sur la Forme | Les valeurs mesurables à un instant donné |
| Actions | Traversent le triangle | edit modifie la Forme, trigger active le Temps, link crée un Lien |
| État | Badge porté par la Forme | Une photo classifiable de la situation du nœud — pas un nœud séparé |
| Contexte & Logique | Au-dessus du triangle | Le pourquoi : intent, goal, rule donnent une raison d'être au tout |
| Persistance | En dessous | Comment le triangle survit quand le système s'éteint |

### Le centre de gravité *(nouveau v1.3)*

Le triangle a l'air symétrique, mais en pratique aucun module ne pèse également sur ses trois sommets. Chaque module a un **centre de gravité** — le sommet qui domine sa nature :

| Penchant | Le module est surtout… | Exemples |
|---|---|---|
| **Forme-lourd** | défini par ce qu'il est ; le Lien/spark y est mince | Banc de Formants (paramètres → onde) |
| **Lien-lourd** | défini par ce qu'il connecte ; né autour du spark | SPARK, LUNA |
| **Temps-lourd** | défini par son rythme, sa propagation | POLLINATOR (auto-tick, cascade), un scheduler |

Conséquence pratique : pour décrire un module, commence par son centre de gravité — c'est le sommet qui portera le plus d'information.

---

## IV. Le spark et la chaîne d'activation

**Définition — spark** : Le spark est le moment où *une idée se transforme en message*. Une fois le spark devenu message, ce n'est plus l'idée qui est liée à ce message — le message se détache et vit sa propre existence dans le système.

Avant le spark : une idée — privée, malléable, sans forme fixe. Après : un message — formulé, détaché, avec sa propre existence. Le spark n'est donc pas une arête entre deux nœuds existants : c'est le point de naissance d'un *nouvel objet* qui n'existait pas avant.

Le `trigger` et le `spark` ne sont pas synonymes : le trigger est ce qui *cause* le spark (le clic, la condition remplie), le spark est la transformation elle-même.

**Intention** *(intégré v1.3)* — n'est pas une étape de la chaîne : c'est la *charge* que porte le message qui naît au spark. Naît au spark, encodée dans le message, meurt avec lui.

### La chaîne d'activation complète

```
trigger  →  spark  →  action  →  action-communication  →  concret
```

| Étape | Catégorie | Ce qui se passe |
|---|---|---|
| trigger | Comportement (Temps) | L'événement déclencheur · son intensité = `élan` |
| spark | Relations (Lien) | L'idée devient message · sa charge = `intention` |
| action | Actions & Opérations | Quelque chose s'exécute |
| action-communication | Relations | Le lien porte un message et ses `propriétés` |
| concret | Forme / Persistance | Le résultat tangible — une Forme changée, stockable |

**Bruit** *(intégré v1.3)* — ce qui intercepte ou altère le message entre émission et décodage. Ce n'est pas un nœud : c'est une *force qui agit sur le lien*. Module-témoin : Justice Parser.

**Propriétés du message** *(intégré v1.3)* — weight, volume, force, direction, frequency, style, texture : mesurées à l'étape *action-communication*. Module-témoin : Banc de Formants.

---

## V. Les trois échelles du pourquoi *(nouveau v1.3)*

La confusion entre `élan`, `trigger` et `spark` venait d'un empilement : on mettait sur une seule ligne trois « forces du pourquoi » qui vivent à trois étages de temps différents.

| Force | Échelle de temps | Ce qu'elle charge | Où elle vit |
|---|---|---|---|
| **élan** | l'événement | l'intensité du trigger | attribut du trigger — Comportement |
| **intention** | le message | la charge d'un spark | attribut du spark — Relations |
| **motivation profonde** | l'entité, toute sa vie | la direction durable | la couche Contexte elle-même |

**Loi des échelles** : Quand deux termes du langage « glissent » l'un dans l'autre, demander d'abord : *vivent-ils à la même échelle de temps ?* S'ils ne s'appliquent pas à la même durée, ce ne sont pas des doublons — ce sont des étages.

---

## VI. Gouvernance des règles

Où vivent les règles d'un nœud ? Modèle hybride, calqué sur la gouvernance existante de Veritas Hortus :

| Niveau | Principe | Équivalent VH existant |
|---|---|---|
| Règles locales | Chaque nœud porte ses propres règles par défaut | Chaque tag/module a ses règles propres |
| Règles globales | Le graphe entier peut imposer des contraintes héritées | Template de tag obligatoire · procédure en 4 phases |

Les règles se logent naturellement au niveau du **spark** — le seul instant où une validation a du sens.

---

## VII. Les tests de validation

Le patron *Forme → trigger → spark → action → action-communication → concret → Forme* a été testé sur trois modules réels (SPARK, LUNA, Planner 3 Cercles) puis, en v1.3, sur cinq modules supplémentaires pour valider les concepts (Justice Parser, Banc de Formants, NEXUS, POLLINATOR, LUNA relu côté Contexte).

Le spark peut *avorter* (refus utilisateur — LUNA), ou l'objet peut *mourir* aussitôt né (POLLINATOR). Le patron doit prévoir ces chemins.

---

## VIII. Filiations et précédents

Trois registres distincts :

**Registre 1 — Filiations primaires** (formation universitaire de communication) :
Stuart Hall (Encoding/Decoding) · Marx via Hall · Saussure · Barthes · Wiener · Luhmann · Chomsky · Richard Hoggart · Gregory Bateson · Henri Bergson.

**Registre 2 — Racines pratiques** :
SketchUp (modularité géométrique) · Scratch My Blocks (modularité logique) · Rust / Veritas Hortus (modularité structurelle + gouvernance des règles).

**Registre 3 — Précédents convergents** (découverts après coup) :
Shannon & Weaver · Schramm / Barnlund · David Harel (statecharts) · Hewitt / modèle d'acteurs · Christopher Alexander · Event sourcing · Prigogine · Théorie du chaos · Cybernétique de second ordre.

**Note de méthode** : La frontière entre registres est *vivante* : dès qu'un précédent convergent est étudié en profondeur, il devient une influence réelle des versions suivantes.

---

## IX. Pistes d'évolution — lemniscate et gyroscope

Deux figures géométriques posées comme prochains concepts à développer. État actuel : hypothèses de travail.

**La lemniscate (∞)** — hypothèse : la chaîne d'activation bouclée. Boucle gauche = monde intérieur ; boucle droite = monde extérieur ; l'unique point de croisement = le **spark**.

**Le gyroscope** — hypothèse : la stabilité par le mouvement. L'identité d'un module n'est pas figée mais *maintenue par son activité*. Les états de Luna (Solid / Liquid / Gas) seraient des vitesses de rotation différentes. Filiation : autopoïèse de Luhmann.

**Combinaison** — La lemniscate est la *trajectoire* du flux ; le gyroscope est ce qui garde le système *orienté* pendant qu'il la parcourt. Le centre de gravité (§III) est peut-être la version statique de ce que le gyroscope décrit en mouvement.

---

## X. Journal d'intégration v1.3

| Concept | Case dans le noyau | Module-témoin | Ce que le remplissage a tranché |
|---|---|---|---|
| **élan** | Attribut d'intensité du trigger (Comportement) | POLLINATOR | Pas une étape — une intensité. Échelle : l'événement. |
| **bruit** | Force agissant sur le lien (Relations) | Justice Parser | Bruit = l'objet, parser = l'acteur qui agit dessus. |
| **propriétés du message** | À l'étape action-communication (Propriétés) | Banc de Formants | weight/volume/force/frequency/texture appliqués à un son réel. |
| **automodularité** | Structure & Modularité | NEXUS (négatif) + POLLINATOR (positif) | Recomposition pilotée par le système à l'exécution. |
| **intention** | Attribut du spark (Relations) | LUNA | Charge locale d'un message ; naît et meurt avec lui. |
| **motivation profonde** | La couche Contexte elle-même | LUNA | Direction durable qui traverse tous les sparks. |

**Définition scellée — modularité vs automodularité** :
- **modularité** = composition conçue par l'architecte, fixe à l'exécution (NEXUS : le pipeline ne se réordonne jamais).
- **automodularité** = recomposition pilotée par le système lui-même à l'exécution (POLLINATOR : greffe de nœuds, propagation en cascade).

**Règle de passage** : Un concept entre dans le noyau quand il a une définition en une phrase, une catégorie d'appartenance, et au moins deux tests réussis sur les modules réels.

---

## XI. Questions ouvertes

- **Q0 · Élan / trigger / spark** — RÉSOLUE en v1.3 par la loi des trois échelles.
- **Q1 · Le chemin du refus** — Comment représenter le spark qui avorte, ou l'objet qui naît puis se termine aussitôt ?
- **Q2 · L'état comme historique** — Le nœud devrait-il porter un historique d'états horodatés ? Résonance directe avec l'event sourcing.
- **Q3 · Le graphe des chaînes entrecroisées** — Comment un Creator redevient-il Supporter d'un autre ? La lemniscate est l'hypothèse en cours.
- **Q4 · Le spark interne** — Une idée qui devient message *pour soi-même*, sans destinataire externe — est-ce encore un spark ?
- **Q5 · Le centre de gravité** — Est-il fixe, ou se déplace-t-il au cours de la vie d'un module ? Si dynamique → rejoint le gyroscope.

---

*Forme · Lien · Temps — et le spark où tout naît*
