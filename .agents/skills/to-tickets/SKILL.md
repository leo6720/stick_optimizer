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
Con più figli togliere ready al parent. La fase di definizione rende pronti i
figli nel perimetro accettato e registra, se autorizzata dall'utente,
**Autorizzazione alla prosecuzione** nel parent/figli: fonte, ID figli espliciti,
revisioni accettate di definizione e sizing. Se l'esecuzione del piano è già
autorizzata, persisterla e identificare i figli dello split nello stesso perimetro
senza chiedere un nuovo consenso ridondante; la sola accettazione funzionale di
una macro non basta. Il runner può poi aggiungere ready
a quei soli figli dopo merge e verifica delle dipendenze su main, senza nuova
approvazione per ogni figlio, secondo async-workflow.md. Non estendere ad altri
ticket o funzioni non coperti dal piano autorizzato; conservare relazioni e blocchi umani.
Ogni ticket si esegue con `spawn_agent(fork_turns: "none")` in un contesto nuovo con branch/PR/checkpoint propri.
