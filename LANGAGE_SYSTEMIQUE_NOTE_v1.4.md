# Les deux centres et l'automodularité

**Note de travail** — la chaîne gravité → révolution → axiome → automodularité
**Candidate v1.4** · à verser dans la Référence · Juillet 2026
**Veritas Hortus · Alliance · La Fleur Harmonique**

---

## Statut de la note

Cette note fige un raisonnement construit en direct, dans l'ordre où il a émergé. Elle n'est pas encore intégrée au noyau : c'est une chaîne cohérente qui demande le même passage que les autres concepts — définition, catégorie, test sur les modules réels. Ce qu'elle propose de changer dans la Référence est listé à la fin (§6).

---

## §1 — Deux centres, pas un

Le centre de gravité (§III de la Référence) décrit où un module se *balance* — son centre de masse, le sommet dominant du triangle. Mais un centre de masse et un centre de *révolution* ne coïncident pas toujours.

En physique, ils se confondent dans un seul cas : un corps **libre**, sans attache — une clé lancée dans le vide tourne autour de son centre de masse. Dès que le corps est **contraint**, les deux se séparent : une porte tourne autour de ses gonds, pas de son centre ; une planète révolue autour d'un point externe.

Le gyroscope (§IX, piste d'évolution) a carrément les *deux à la fois* : il tourne sur son axe propre (par son centre de masse) **et** précesse autour de son point d'appui (un pivot externe à sa masse). C'est ce cas à deux centres qui décrit le mieux un module vivant.

| Centre | Ce qu'il capture | Nature |
|---|---|---|
| **gravité** | ce que le module *est*, au repos — son centre de masse, le sommet dominant | interne, souverain |
| **révolution** | ce autour de quoi son *activité* tourne, en marche — son pivot | relationnel, peut être externe |

**Ce que ça résout — Q5**
On disait que le centre d'un module « migre » à l'activation. Imprécis : il n'y a pas un point qui se déplace, il y a **deux points**, et l'activation révèle le second. LUNA : masse en Contexte (sa motivation profonde), mais activité qui révolue autour de *l'utilisateur* — un pivot au pôle Lien, hors d'elle. Elle précesse autour de son interlocuteur pendant que son centre de gravité reste ailleurs. Q5 se referme : le déplacement apparent était la distance entre deux centres distincts.

---

## §2 — La distance comme mesure de souveraineté

Les deux centres **coïncident** pour un module autonome : il tourne autour de son propre sommet. Le Banc de Formants, au repos comme en marche, pivote autour de sa Forme — un corps libre. Ils se **séparent** pour un module couplé à un extérieur : LUNA orbite l'utilisateur, SPARK orbite la relation creator↔supporter.

D'où une grandeur mesurable : la **distance entre le centre de gravité et le centre de révolution** mesure la dépendance du module à son extérieur.

| Distance | Lecture | Fragilité |
|---|---|---|
| **nulle** | le module tourne autour de lui-même — souverain | robuste : rien d'externe ne le fait tourner |
| **grande** | le module est suspendu à un pivot externe — dérivé | fragile : si le pivot disparaît, il précesse dans le vide |

La précession dont parlait le gyroscope, c'est exactement cet écart. La figure n'était pas une métaphore approximative : elle donnait déjà les deux centres.

---

## §3 — L'axiome central = le centre immobile

Un axiome fait ce que fait un centre de gravité : c'est le point autour duquel tout s'organise, ce qu'on ne peut pas retirer sans effondrer la structure. Le test d'ablation du centre de gravité — « gèle ce sommet, est-ce encore lui-même ? » — **est** le test d'un axiome : un axiome est la proposition dont l'ablation effondre le système. Le gabarit était déjà un détecteur d'axiomes sans le nommer ainsi.

Et les deux centres donnent **deux sortes d'axiomes** :

| Axiome | Ce qu'il fixe | Exemple |
|---|---|---|
| **de gravité** | ce que le module *est* — interne, souverain ; l'ablation le tue | Banc : « je synthétise par formants » |
| **de révolution** | ce autour de quoi il *tourne* — le pivot ; l'ablation le désoriente sans le tuer | LUNA : « je tourne autour du consentement de l'utilisateur » |

**Critère de centralité (scellé)**
Un **axiome central** de Veritas Hortus est un module à **distance nulle** : gravité et révolution confondus, centre immobile. Il tourne autour de lui-même, ne s'appuie sur rien — il est *fondateur*. Un module à distance non nulle est *dérivé* : il orbite un autre. On ne décrète pas qu'un module est central — on le **mesure** par l'écart de ses deux centres.

Premières lectures, à vérifier au gabarit : NEXUS — probablement central (il ne révolue autour de rien, il *est* le pivot des autres). POLLINATOR — peut-être (il se propage autour de lui-même, sans centre externe). LUNA — non, elle orbite l'utilisateur. SPARK — non, il orbite la relation.

---

## §4 — L'automodularité redéfinie — le déplacement du centre

Un système automodulaire (v1.3 : « recomposition pilotée par le système à l'exécution ») doit *décider* comment se recomposer : greffer un nœud *où*, router *vers quoi*. Cette décision exige un point de référence — un centre. La question devient : **quel centre gouverne la recomposition ?**

| Recompose autour de… | Mode de croissance | Effet |
|---|---|---|
| son **centre de gravité** (sa masse) | accrétion — il ajoute autour de lui-même | plus grand, pas différent (conservateur) |
| son **centre de révolution** (un pivot externe) | orbite — il se réorganise selon un dehors | différent, pas juste plus grand (adaptatif) |

POLLINATOR utilise déjà les deux sans qu'on l'ait dit : quand un pollen re-spawn en cascade, il se propage autour du nœud d'arrivée (gravité locale, accrétion) ; quand `addNode()` greffe un nœud avec un parent assigné, le nouveau nœud révolue autour d'un pivot qui n'est pas lui (révolution, orbite). Le même système bascule d'un centre à l'autre selon l'opération — et sa souplesse *vient* de là.

**Définition affinée — v1.4 candidate**
- **modularité** — composition autour d'un centre *fixé par l'architecte*.
- **automodularité** — composition autour d'un centre que *le système déplace lui-même* à l'exécution.

La v1.3 disait *que* le système pilote sa recomposition ; cette note dit **ce qui** est piloté : la location du centre. Un système qui ne peut pas bouger son centre est modulaire ; un système qui le bouge est automodulaire. C'est observable dans le code — *qui décide du pivot ?* — et ça sépare proprement NEXUS (centre cloué par l'architecte, pipeline fixe) de POLLINATOR (pivot choisi à chaque greffe).

---

## §5 — L'équilibre — des points immobiles pour que d'autres orbitent

La chaîne boucle sur elle-même. Un axiome central est un module dont le centre ne bouge pas (§3). L'automodularité est le déplacement du centre (§4). Les deux ne s'opposent pas : ils se **tiennent l'un l'autre**.

**Le principe de souveraineté**
Il faut des points immobiles pour que d'autres puissent orbiter. Un système *tout* automodulaire, sans aucun axiome central, n'aurait aucun point de référence — il ne se recomposerait pas, il se **dissoudrait**. La souveraineté de Veritas Hortus tient à cet équilibre : quelques centres immobiles (les axiomes fondateurs), et autour, la recomposition libre des modules qui déplacent leur centre par rapport à eux.

C'est ce qui donne enfin un socle au programme des axiomes communs : non pas une liste décrétée, mais l'ensemble des modules souverains — distance nulle — autour desquels tous les autres tournent.

---

## §6 — À verser dans la Référence (v1.4)

| Où | Changement proposé |
|---|---|
| §III — centre de gravité | Dédoubler en *deux centres* (gravité / révolution) ; la migration de Q5 devient la distance entre les deux. |
| Nouvelle sous-section | La distance comme mesure de souveraineté (§2 de cette note) — grandeur mesurable, robuste vs fragile. |
| Programme des axiomes communs | Ajouter le critère de centralité : axiome central = module à distance nulle. Donne le socle du programme. |
| §X — définition modularité/automodularité | Affiner : automodularité = déplacement du centre de recomposition à l'exécution. |
| §XI — questions ouvertes | Q5 marquée résolue (deux centres). Nouvelle question : la distance des deux centres est-elle stable, ou varie-t-elle selon la charge du module ? |
| Test à mener | Établir gravité + révolution pour chaque module ; lire la distance ; dresser la liste des axiomes centraux (distance nulle). |

---

*Note de travail · candidate v1.4 · document autonome, aucune dépendance externe*
*Des centres immobiles, et autour, la recomposition libre*
