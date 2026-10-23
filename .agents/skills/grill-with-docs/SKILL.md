---
name: grill-with-docs
description: "Intervistare a fondo su una richiesta con il metodo grilling originale, documentando termini e decisioni di Stick Optimizer."
disable-model-invocation: true
---

# grill-with-docs

Usare lo stesso metodo di intervista di grill-me. Prima di iniziare leggere
integralmente entrambe le dipendenze locali, senza affidarsi al solo nome:

- [grilling](../grilling/SKILL.md): albero delle decisioni, round sull'intera
  frontiera delle domande indipendenti, raccomandazione per ciascuna domanda,
  attesa delle risposte e conferma finale dell'intesa.
- [domain-modeling](../domain-modeling/SKILL.md): glossario aggiornato durante
  il chiarimento dei termini e ADR solo quando tutti e tre i criteri si applicano.

Sono copie delle dipendenze originali alla revisione indicata in
[UPSTREAM.md](../UPSTREAM.md). Se il client offre un meccanismo Skill, usarlo;
altrimenti aprire i file indicati e applicarli. Se manca una dipendenza,
segnalarla prima dell'intervista, senza improvvisarne una versione ridotta.

## Integrazione con Stick Optimizer

Leggere AGENTS.md, GLOSSARY.md, docs/agents/workflow.md e il ticket completo.
Prima delle domande esplorare il comportamento attuale; delegare la ricerca
dei fatti secondo grilling. Non chiedere all'utente ciò che è reperibile nel repo.
Non ripetere decisioni già accettate: inserirle come nodi risolti dell'albero.
Le risposte possono riaprire un nodo e cambiare le domande successive.

L'intervista approfondisce assunzioni, alternative, casi limite, errori ed
esclusioni pertinenti alla richiesta. Non basta rendere descrivibile l'idea.
Non anticipare nello stesso round domande che dipendono da risposte ancora aperte.
Inviare i round attraverso l'interfaccia di domande disponibile, con numerazione
e raccomandazioni; una raccomandazione non selezionata o il silenzio non sono consenso.
Non allargare il perimetro per prolungare il grill: se serve una prova concreta
invece di un'altra domanda, registrare il punto aperto e concordare come chiarirlo.
Non avviare implementazione di prodotto durante il grill.

Oltre ai soli termini e ADR di domain-modeling, conservare nel ticket le decisioni
con riferimento alle risposte dell'utente: risultato desiderato, criteri
osservabili, esclusioni, valori numerici e unità, errori/casi limite e verifiche.
Le risposte mancanti restano dubbi aperti, non scelte inventate.
Alla frontiera vuota presentare la sintesi e attendere la conferma dell'intesa.
Solo dopo l'accettazione registrare «Definizione accettata» con la fonte utente.

Passare a to-spec nella stessa conversazione, conservando le decisioni, poi
a to-tickets per stima e divisione a circa 150.000 token per ticket.
Il grill da solo non rende pronta la macro. Todo + ready-for-agent richiedono
definizione, dimensionamento e autorizzazione secondo docs/agents/async-workflow.md;
resta valida la prosecuzione dei figli esplicitamente già autorizzata.
