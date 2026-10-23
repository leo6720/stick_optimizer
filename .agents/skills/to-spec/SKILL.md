---
name: to-spec
description: "Sintetizzare la conversazione in una specifica completa con il metodo originale, confini pubblici di test e gate Linear di Stick Optimizer."
disable-model-invocation: true
---

# To Spec

Leggere integralmente la [fonte originale fissata](references/upstream.md) e
questa integrazione. Applicare il metodo originale con le precedenze locali
esplicite sotto; non usare la fonte da sola come autorizzazione a pubblicare o dare ready.
Provenienza, licenza e differenze: [UPSTREAM.md](../UPSTREAM.md).

## 1. Sintetizzare il contesto già chiarito

Usare la conversazione del grill nella stessa sessione, il ticket completo con
commenti/relazioni e il comportamento corrente del repo. Non ricominciare
l'intervista e non perdere decisioni numeriche, esclusioni o casi limite.
Leggere AGENTS.md, glossario, ADR pertinenti e docs/agents/issue-tracker.md.
Se manca una decisione funzionale bloccante, registrarla in una bozza con
needs-info; non colmarla con una scelta inventata o dichiarare la definizione accettata.

## 2. Definire e concordare i confini dei test

Individuare il punto pubblico più alto da cui osservare il comportamento.
Preferire confini esistenti a nuove interfacce e pochi confini ben scelti a
test distribuiti sui dettagli interni. Documentare moduli coinvolti ed esempi
di test analoghi nel repo. Proporre un nuovo confine solo quando necessario.
Confrontare i confini con le aspettative dell'utente: se già concordati nel
grill citarne la decisione senza chiedere di nuovo; altrimenti presentarli e
attendere la risposta prima di considerarli concordati. Non è un nuovo grill.

## 3. Scrivere la specifica completa

Usare docs/agents/templates/spec.md includendo tutte le parti del metodo originale:

- Problema dal punto di vista dell'utente e soluzione attesa.
- Elenco numerato completo di storie utente («Come ..., voglio ..., per ...»)
  che copra tutti i comportamenti nel perimetro, con criteri osservabili.
  Completezza non significa aggiungere funzioni o riempire una quota di storie.
- Decisioni di implementazione già prese: moduli/interfacce, architettura,
  schema, contratti, interazioni, unità e precisione; separare dubbi e assunzioni.
- Decisioni di verifica: comportamento esterno, confini concordati, moduli
  e precedenti di test; distinguere prove native umane da controlli cloud.
- Fuori perimetro e note pertinenti, comprese dipendenze e rischi.

Nel contratto funzionale evitare percorsi specifici e frammenti di codice
che invecchiano rapidamente. URL/ID, baseline SHA e fonti di verifica restano
metadati di tracciabilità. Eccezione: un frammento nato da un prototipo può
esprimere una decisione (schema, stati, tipi) più precisamente della prosa;
includere solo la parte necessaria e indicarne la provenienza.
Collegare ogni decisione del grill alla storia/criterio e alla verifica
corrispondenti, conservando vincoli negativi e valori esatti.

## 4. Pubblicare e passare a to-tickets

Pubblicare sul Linear configurato quando richiesto o già autorizzato,
verificando workspace, duplicati e issue esistente; altrimenti lasciare
una bozza esplicitamente non pubblicata. Nessuna duplicazione in GitHub Issues.
Conservare nel ticket «Definizione accettata» con fonte e revisione reali.
Includere Dimensionamento secondo docs/agents/token-sizing.md:
T motivato, incertezza e N=max(1,ceil(T/150000)), poi passare a to-tickets.

**Precedenza locale sulla fonte:** la sola specifica non applica automaticamente
ready-for-agent. Servono ticket esecutivo accettato/dimensionato, dipendenze
risolte e autorizzazione allo sviluppo secondo docs/agents/async-workflow.md.
Nessuna implementazione di prodotto durante sintesi, stima o scomposizione.
