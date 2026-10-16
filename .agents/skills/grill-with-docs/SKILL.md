---
name: grill-with-docs
description: "Chiarire una richiesta di Stick Optimizer e documentare termini e decisioni prima di implementarla."
---

# grill-with-docs

Adattamento locale per Stick Optimizer. Riferimento: docs/agents/workflow.md.

Leggere AGENTS.md, GLOSSARY.md, docs/agents/workflow.md e il ticket.
Esplorare il comportamento attuale prima delle domande. Usare esempi concreti.
Chiedere una decisione necessaria alla volta; raggruppare soltanto domande
strettamente collegate. Non ripetere decisioni già prese.
Registrare risultato desiderato, criteri, esclusioni, dubbi e confini di test.
Aggiornare glossario e ADR solo per termini o decisioni effettivamente emersi.
Non inventare approvazioni e non iniziare codice che dipenda da risposte mancanti.
Consegnare una sintesi utilizzabile da to-spec. Dopo l'accettazione dell'utente
registrare nel ticket Definizione accettata con decisioni e criteri; su sua
indicazione impostare Todo + ready-for-agent secondo docs/agents/async-workflow.md.
Non iniziare l'implementazione durante il grill o marcare pronto senza consenso.
