---
name: tdd
description: "Verificare comportamento e bug con cicli test fallito, modifica minima e verifica."
---

# tdd

Adattamento locale per Stick Optimizer. Riferimento: docs/agents/workflow.md.

Leggere glossario, specifica e test esistenti. Definire il confine pubblico
da osservare e riusare quelli già concordati; chiarire nuovi contratti incerti.
Scegliere un esempio con risultato atteso indipendente dall'implementazione.
Scrivere un test, eseguirlo e verificare che fallisca per il motivo previsto.
Implementare il minimo comportamento e rieseguire; ripetere per un caso alla volta.
Ristrutturare solo con test verdi, mantenendo invariati i contratti.
Non copiare le formule sotto test per costruire gli attesi.
Nel core verificare vincoli e Pareto su casi piccoli, raggruppamento 1 e
precisione pertinenti. Per IO verificare round trip e conservazione dei dati.
Gli eventi GUI sintetici non certificano selettori Windows e monitor/DPI.
Non aggiungere test che rispecchiano l'implementazione o per soli ritocchi testuali.
