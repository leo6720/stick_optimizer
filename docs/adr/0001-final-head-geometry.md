# 0001 — Coordinate finali deterministiche di una testa

Stato: decisione tecnica implementata in LEO-17, preparatoria a LEO-18.
Fonti funzionali: [LEO-16](https://linear.app/leochist/issue/LEO-16),
definizione accettata il 20 ottobre 2026, e
[LEO-17](https://linear.app/leochist/issue/LEO-17).

## Contesto

Stesso raggruppamento e passo non bastano a certificare una testa condivisa.
La definizione richiede coordinate finali fisse compatibili con l'intero
insieme assegnato. Gruppo 4 mantiene i due centrali al passo di prelievo e
forma due coppie: non può essere trattato come quattro stick compatti.

## Decisione

`HeadFormat` identifica un formato tramite ID e specifica larghezza, passo di
prelievo, raggruppamento e divisori. `final_head_geometry` riceve tutte le
assegnazioni a una testa e i parametri globali. Non deriva il passo dalla
larghezza nominale: lo shift MT permette larghezze diverse allo stesso passo.
Richiede stessa chiave `Pitch`, stesso gruppo 2/3/4 e canali divisibili per il
gruppo. Gli ID sono univoci e vengono restituiti ordinati. I canali e gruppi
mantengono indici zero-based nell'ordine da sinistra a destra.

Posti P=passo di prelievo e d=interasse comune di avvicinamento:

| Gruppo | Coordinate locali finali dei centri | Divisori ammessi e posizione |
| --- | --- | --- |
| 2 | 0, d | 0 oppure 1 fra i due stick |
| 3 | P−d, P, P+d | 0 oppure 2, uno in ogni intervallo |
| 4 | P−d, P, 2P, 2P+d | 0 oppure 1 fra i due centrali, separando le coppie |

Questa posizione dei divisori coincide con i compartimenti esistenti
`[1,1]`, `[1,1,1]`, `[2,2]` e con il requisito parent delle due coppie.
Non introduce un diverso layout funzionale. Il gruppo k aggiunge k×g×P a
ogni coordinata locale; al prelievo il canale i ha coordinata i×P.

Per ogni intervallo di avvicinamento, il minimo del formato è larghezza più
`adjacent_clearance`, oppure larghezza più `divider_width` più due volte
`wall_clearance` se quell'intervallo contiene un divisore. Scegliere d come
massimo dei minimi dell'intero insieme. Richiedere d−minimo ≤
`max_extra_spacing` per **ogni** formato e d≤P, per movimento verso l'interno.
Con gruppi 2/3 divisi valgono quindi gli esempi 25/26→37,5 mm; senza divisori
25/26→27 mm. Nel gruppo 4 i due intervalli mobili sono interni alle coppie:
25/26→27 mm anche con il divisore centrale. Fra i centrali richiedere
P≥minimo pertinente, senza applicare il limite aggiuntivo a P−minimo.
Gli spazi fra gruppi restano almeno P e rispettano così i minimi già verificati.
Cassetti, pareti esterne e assegnazioni separate/condivise saranno dimensionati
e verificati da LEO-18 usando le coordinate effettive.

P usa i tick del core (milionesimi di mm), positivi e strettamente inferiori
a 2^53. Le altre lunghezze conservano f64 finiti e non vengono arrotondate:
non arrotondare d al più vicino tick, perché potrebbe scendere sotto un minimo.
I confronti geometrici usano 1e-9 mm come il core esistente. Impostazioni non
finite/negative, larghezze non positive e configurazioni invalide sono errori;
passi/gruppi diversi, limite superato, spazio fisso insufficiente o movimento
verso l'esterno producono incompatibilità. Non si presume transitività a coppie.

## Alternative e conseguenze

Confronti a coppie concatenati perderebbero il vincolo del formato più stretto.
Coordinate compatte per gruppo 4 sposterebbero i centrali fissi. Arrotondare
ogni lunghezza alla griglia dei passi cambierebbe minimi geometrici esistenti.
Per questo l'API calcola una sola configurazione sull'insieme completo.

Il core resta indipendente dalla persistenza e dalla GUI. Il nuovo parametro
ha default 5 mm ed è validato anche nello snapshot generale; il divisore
incorporato diventa 7,5 mm. Schema studi/preferenze 2 e migrazione in memoria
senza scritture dello schema 1 sono documentati in [rust-files](../rust-files.md).
Nessuna sostituzione parziale della ricerca, nuova penalità, cinematica,
controllo GUI inerte o popup testa fa parte di questa decisione.
