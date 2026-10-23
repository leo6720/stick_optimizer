---
name: to-tickets
description: "Scomporre con il metodo originale in ticket verticali verificabili, concordare il piano e pubblicare sub-issue Linear dimensionate con blocchi nativi."
disable-model-invocation: true
---

# To Tickets

Leggere integralmente la [fonte originale fissata](references/upstream.md) e
questa integrazione. Applicare il metodo originale con le precedenze locali
esplicite sotto; la fonte da sola non autorizza ready o modifiche al parent.
Fonte/licenza/differenze: [UPSTREAM.md](../UPSTREAM.md).

## 1. Raccogliere il contesto

Partire dalla specifica o conversazione già disponibile. Se arriva un riferimento,
leggerne corpo completo, commenti e relazioni; usare glossario e ADR pertinenti.
Leggere docs/agents/issue-tracker.md e docs/agents/token-sizing.md.
Non rifare il grill né modificare una decisione accettata durante lo split.

## 2. Esplorare il codice e preparare i confini

Se non già esplorato, leggere il codice coinvolto e i suoi consumatori/test.
Cercare preparazioni del codice che rendano semplice la modifica e pianificarle
prima dei ticket dipendenti, con un proprio risultato pubblico verificabile.
Pianificare non significa implementare: nessun codice di prodotto in questa fase.

## 3. Proporre tagli verticali e dimensionarli

Ogni ticket attraversa tutte le parti necessarie al suo comportamento
(dominio/schema/interfacce/UI/test, dove pertinenti), resta dimostrabile o
verificabile da solo e si sviluppa in un contesto nuovo. Evitare ticket di un
solo livello tecnico quando non producono un risultato verificabile.
Ogni ticket dichiara solo i blocchi reali. Non creare sequenze artificiali,
cicli, controlli inerti o una base preparatoria spacciata per funzione completa.

Applicare il dimensionamento locale: stima totale T motivata comprensiva di
analisi, sviluppo, test e review/correzioni; N=max(1,ceil(T/150000)). Stimare ogni
ticket indicativamente entro 150.000 token e suddividerlo ancora se necessario.
Registrare motivazione, incertezza, revisione, overhead e differenze tra stime
iniziali e somma finale. N=1 richiede spiegare perché basta l'originale;
N>1 richiede piano parent/figli. Non è un hard cap del consumo o della finestra.

Per refactor meccanici ampi applicare expand–contract: prima introdurre la nuova
forma accanto alla vecchia, poi migrare consumatori in lotti verificabili e
dimensionati, infine rimuovere la vecchia dopo tutti i lotti; build coerenti
a ogni passaggio. La fonte ammette un branch di integrazione quando i lotti
non possono restare verdi da soli. **Qui quella variante non è esecutiva:**
il runner richiede main e PR verificabili per ticket. Registrare il blocco di
piano e richiedere una decisione esplicita sul diverso protocollo, senza
creare dipendenze fittizie o avviare ticket non verificabili.

## 4. Confrontare la suddivisione con l'utente

Presentare un elenco numerato: titolo, risultato dall'inizio alla fine,
bloccato da, criteri/verifiche e stima. Chiedere se granularità e blocchi sono
corretti e se accorpare o dividere ulteriormente; iterare fino all'approvazione.
Se la stessa suddivisione è già approvata, citarne fonte e revisione.
Se l'utente ha esplicitamente delegato la scomposizione tecnica entro un piano
già accettato e autorizzato, persistere tale fonte e motivare la corrispondenza
senza chiedere un consenso ridondante. La sola accettazione funzionale di una
macro non equivale a tale delega né all'autorizzazione allo sviluppo.
Il silenzio non è approvazione; una variazione sostanziale richiede una decisione.

## 5. Pubblicare il piano approvato su Linear

Prima cercare duplicati, verificare workspace e risolvere gli UUID reali.
Usare docs/agents/templates/ticket.md. Creare prima i bloccanti e poi i
dipendenti; una issue per ticket. Se la fonte è una issue, creare i figli con
parentId e relazioni native blockedBy. Non sostituire i blocchi con sola prosa
e non rimuovere dipendenze esistenti per rendere avviabile il lavoro.
Preservare storie/decisioni della specifica: ogni criterio complessivo deve
essere coperto da un ticket e da una verifica, senza perdere vincoli negativi.
Evitare percorsi/codice nel contratto funzionale; mantenere URL/ID/SHA come
metadati e solo frammenti di prototipo necessari a esprimere una decisione.

**Precedenze locali sulla fonte:**

- Tracker esclusivo Linear leochist; se indisponibile usare bozze in
  docs/agents/drafts/ dichiarandole non pubblicate, senza cambiare tracker.
- N>1: aggiornare il parent solo con piano, stime, tracciabilità e autorizzazione
  pertinente, togliendogli ready; resta contenitore, non chiuderlo. N=1 può
  conservare l'issue originale come ticket esecutivo. Non alterare altri contratti.
- Non marcare tutti i figli ready alla pubblicazione: solo ticket esecutivi
  definiti/dimensionati in Todo, senza blocchi o etichette bloccanti e con sviluppo
  autorizzato possono avere ready-for-agent. Una PR aperta/Done non prova main.
- Per i figli di un piano con sviluppo/prosecuzione già esplicitamente autorizzati,
  persistere «Autorizzazione alla prosecuzione» in parent e figli con fonte,
  ID espliciti e revisioni. Il runner al risveglio abilita quei soli figli dopo
  merge e verifica delle dipendenze su main, senza nuova approvazione per figlio.
  Non inferire autorizzazione dalla parentela o estenderla a funzioni nuove.

Seguire docs/agents/async-workflow.md per tutte le condizioni di presa.
Ogni ticket ha branch/PR/checkpoint e implementatore nuovo
`spawn_agent(fork_turns: "none")`; anche i revisori hanno contesti nuovi separati.
Non cambiare automazioni, non eseguire merge/deploy o implementazione di prodotto
come effetto della sola creazione dei ticket.
