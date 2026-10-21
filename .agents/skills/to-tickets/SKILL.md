---
name: to-tickets
description: "Scomporre una specifica in ticket verticali con criteri e dipendenze Linear."
---

# to-tickets

Adattamento locale per Stick Optimizer. Riferimento: docs/agents/workflow.md.

Leggere la specifica e docs/agents/issue-tracker.md.
Proporre modifiche piccole, ciascuna dimostrabile e verificabile end-to-end.
Usare docs/agents/templates/ticket.md. Per modifiche meccaniche ampie
valutare espansione, migrazione e rimozione mantenendo build coerenti.
Esplicitare dipendenze reali, senza imporre sequenze artificiali.
Rispettare il perimetro concordato; chiarire solo scelte che lo cambiano.
Prima della pubblicazione cercare duplicati e risolvere ID/UUID reali.
Creare i blocchi prima dei dipendenti e usare relazioni native.
ready-for-agent richiede accettazione chiara e informazioni sufficienti;
il lavoro parte solo quando i blocchi sono risolti. Non chiudere il parent.

Applicare obbligatoriamente docs/agents/token-sizing.md: stima totale motivata,
N = max(1, ceil(T / 150000)), stima per figlio entro circa 150.000, criteri propri.
Registrare piano parent e dipendenze; con N=1 documentare perché non serve split.
Con più figli togliere ready al parent. Solo la fase di definizione può rendere
pronti figli che ereditano il perimetro già accettato; niente nuove funzioni.
Ogni ticket si esegue in un contesto nuovo con branch/PR/checkpoint propri.
