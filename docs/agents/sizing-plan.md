# Prima applicazione del dimensionamento — 21 ottobre 2026

Audit: né main 05f6f1e né il precedente head della PR #18 (664be2cf)
contenevano stima dei token o formula /150.000. to-tickets chiedeva solo
ticket piccoli/verificabili; non equivaleva alla regola richiesta dall’utente.
LEO-8 e LEO-9 erano stati implementati senza una stima preventiva esplicita:
non inventare retroattivamente una verifica allora non eseguita e non rifare
le PR in attesa umana.

## LEO-16

Definizione funzionale accettata il 20 ottobre; scomposizione tecnica autorizzata
dalla richiesta del 21 ottobre. T centrale stimato = **390.000 token**;
N = max(1, ceil(390000 / 150000)) = **3**. Stime arrotondate, incertezza media,
non consumi misurati. Riferimenti letti: models/search/geometry nel core,
DTO e CSV IO, draft e consumatori GUI. Il contratto fisico resta quello del parent.

| Ticket | Confine | Stima token | Dipendenza |
| --- | --- | --- | --- |
| [LEO-17](https://linear.app/leochist/issue/LEO-17/leo-16-definire-geometria-finale-delle-teste-e-parametri-persistiti) | Geometria/compatibilità e parametri persistiti | 120.000 | nessuno |
| [LEO-18](https://linear.app/leochist/issue/LEO-18/leo-16-enumerare-teste-separate-e-condivise-con-dimensioni-effettive) | Ricerca esatta separate/condivise | 140.000 | LEO-17 |
| [LEO-19](https://linear.app/leochist/issue/LEO-19/leo-16-completare-editor-dettagli-e-csv-delle-configurazioni-testa) | Editor, dettagli, CSV e flusso completo | 130.000 | LEO-18 |

Somma =390.000. Test, integrazione e revisioni previste sono inclusi in ciascuna
stima; non è un tetto garantito. Una ristima superiore al target richiede nuovo
split del residuo. LEO-17 è preparatorio: API core e round trip parametri,
nessun controllo GUI inerte né promessa di ricerca già corretta. LEO-18 integra
la ricerca e mantiene coerenti i consumatori necessari. LEO-19 completa editor,
presentazione, CSV e accettazione end-to-end.

ParentId e blockedBy sono relazioni native Linear; il parent LEO-16 non ha
ready-for-agent ed è bloccato dai tre figli. LEO-7 continua a dipendere dal
parent completo/integrato. Non basta una PR di un figlio per sbloccarlo.

## LEO-7

T =**120.000 token**, N =max(1, ceil(120000/150000)) =**1**. Stima motivata da
popup unico, coordinate fornite dal core, rendering/quote, identità/invalidazione,
test egui, prove compatta/zoom e revisioni. Non include rifare la ricerca o
fisica di LEO-16. Nessuno split artificiale; dipendenza LEO-16 irrisolta.

## Esecuzione

Ogni ticket riceve un contesto nuovo e branch/PR/checkpoint propri. La presente
attività ha organizzato il lavoro, senza avviare implementazioni di prodotto.
Il prompt eseguibile del runner è versionato in runner-prompt.md; aggiornare
solo quel prompt nell’automazione esistente, preservando cadenza e abilitazione.
Esito di pubblicazione/revisione e head finale sono registrati in LEO-15/PR #18.

## Autorizzazione alla prosecuzione — 21 ottobre 2026

Fonte esplicita dell'utente: «il protocollo così non va bene, se faccio il merge
lui poi deve andare da solo». Per il solo piano accettato LEO-16 sono autorizzati
LEO-17, LEO-18 e LEO-19 nelle definizioni e stime sopra (120.000/140.000/130.000;
T 390.000, N 3). L'autorizzazione consente ready automatico al risveglio esistente
solo dopo merge raggiungibile su main e verifica dell'accettazione di tutte le
dipendenze, con gli altri gate invariati. Non autorizza nuove funzioni o figli.
Snapshot, fonte e limiti: [checkpoint](checkpoints/dependency-continuation.md).
LEO-7 conserva il proprio ready accettato e aspetta l'intero LEO-16 verificato.
