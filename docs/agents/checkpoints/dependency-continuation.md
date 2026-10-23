# Prosecuzione autorizzata dopo le dipendenze

- Data: 21 ottobre 2026.
- Fonte: richiesta esplicita dell'utente nella sessione di coordinamento:
  «il protocollo così non va bene, se faccio il merge lui poi deve andare da solo».
- Risultato accettato: i figli dello split già grilled, accettato e dimensionato
  proseguono senza una nuova approvazione ready per ciascun figlio quando tutte
  le dipendenze sono integrate e verificate su main.
- Baseline della modifica documentale:
  `f2956653e5f6bb80d638fdbd28eab8cef50ef087`.
- Branch: `codex/async-dependency-continuation`.
- Perimetro di questa consegna: solo documentazione/protocollo e skill locali;
  nessuna implementazione di prodotto, mutazione Linear, automazione o merge
  eseguita dall'implementatore di questa modifica.
- Dimensionamento preventivo di questa modifica: circa **20.000 token**,
  incertezza media, N = max(1, ceil(20000 / 150000)) = **1**. Revisione accettata:
  richiesta sopra e istruzioni di delega del coordinatore del 21 ottobre.
  Un unico diff documentale è verificabile insieme; nessuno split necessario.

## Autorizzazione alla prosecuzione

Parent: [LEO-16](https://linear.app/leochist/issue/LEO-16).
Elenco completo dei figli autorizzati: **LEO-17, LEO-18, LEO-19**.
La presente richiesta non autorizza alcun altro piano o figlio sconosciuto.
Il coordinatore ha confermato di aver persistito l’autorizzazione in Linear
LEO-16, LEO-18 e LEO-19; l’implementatore di questa modifica non ha eseguito
quelle scritture esterne.

Revisioni/snapshot di riferimento:

- Definizione funzionale LEO-16 accettata il 20 ottobre 2026 e scomposizione
  tecnica del 21 ottobre: [sizing-plan.md](../sizing-plan.md) alla baseline sopra.
- Dimensionamento LEO-16: T 390.000, target 150.000, N 3, somma figli 390.000.
- LEO-17: geometria/compatibilità e parametri persistiti, 120.000, nessun blocco.
- LEO-18: ricerca esatta separate/condivise, 140.000, dipende da LEO-17.
- LEO-19: editor/dettagli/CSV e flusso completo, 130.000, dipende da LEO-18.
- LEO-7: definizione/ready già accettata e stima propria 120.000, N 1; conserva
  la dipendenza nativa dall'intero LEO-16 integrato e verificato.

Questi riferimenti fissano il piano autorizzato, non provano che ogni ticket sia
ora eseguibile. Il runner rilegge sempre le revisioni complete correnti in Linear
e le confronta col registro persistito. Una modifica sostanziale del contratto,
richiede una nuova decisione. Per uno split/ristima nel medesimo perimetro già
autorizzato, la fase di definizione aggiorna esplicitamente ID e revisioni nel
registro senza inventare ready e senza chiedere un consenso ridondante.
Pause, revoche e blocchi umani prevalgono; non vengono cancellati dal merge.

## Evidenze da registrare al risveglio

Per ogni figlio ancora Todo senza ready: fonte dell'autorizzazione e revisioni,
lettura aggiornata di descrizione/commenti/etichette/relazioni/stato, ogni PR
dipendente effettivamente merged verso main nel repository corretto, merge SHA,
main SHA aggiornato e prova di raggiungibilità, riferimenti ed esiti della
verifica dei criteri di accettazione del prerequisito. Non usare da soli push,
PR aperta o Linear Done. Se manca prova non abilitare il figlio.

Se tutti i gate di [async-workflow.md](../async-workflow.md) passano, aggiungere
ready-for-agent e annotare le prove senza eliminare dipendenze o blocchi umani.
Poi applicare la normale selezione e presa di un solo ticket, con controllo dei
claim/branch/PR concorrenti. Non creare nuove pianificazioni o webhook.
LEO-17 merged/verificato abilita LEO-18; LEO-18 merged/verificato abilita LEO-19;
LEO-7 aspetta il parent LEO-16 completo. Sono esempi di regola, non una nuova
attestazione di merge o verifica eseguita in questa attività documentale.

## Verifica e consegna

Controlli superati: `git diff --check`, 21 link Markdown locali nei 12 file
modificati, confronto byte per byte del runner-prompt con il testo operativo
fornito dal coordinatore e ricerca delle regole contraddittorie su ready/isolamento. Nessun test Rust pertinente a un
diff soltanto documentale. La revisione indipendente, la PR e l'allineamento del
prompt live sono responsabilità del coordinatore su baseline/head fisse;
modificare il repository non modifica da solo il runner esistente. Restano
invariati toolchain, verifiche pertinenti, limite di tre cicli e divieto di merge.

## Esito della consegna — ciclo 1

Resoconto ricevuto dal coordinatore il 21 ottobre 2026:

- Head revisionata: `b60ade4b7f165c3cf61fd5a2d04201320fce1122`, baseline
  `f2956653e5f6bb80d638fdbd28eab8cef50ef087`.
- Due revisori indipendenti creati in contesti nuovi `fork_turns: "none"`:
  `/root/continuation_spec_review` **PASS** e
  `/root/continuation_standards_review` **PASS**.
- Cicli conclusi: **1**. Rilievi aperti: **nessuno**. Gli esiti sono quelli
  comunicati dal coordinatore, non un’autorevisione dell’implementatore.
- PR draft: [#22](https://github.com/leo6720/stick_optimizer/pull/22).
- Il coordinatore ha aggiornato con successo il solo prompt dell’automazione
  esistente il **21 ottobre 2026 alle 20:44 UTC**. Testo identico al file operativo
  fornito e al runner-prompt versionato; cadenza, abilitazione e fuso preservati.
- Autorizzazione alla prosecuzione persistita dal coordinatore in Linear
  LEO-16, LEO-18 e LEO-19. LEO-19 resta privo di ready fino a integrazione e
  verifica di LEO-18; nessuna relazione o blocco umano rimosso da questa modifica.

Questo aggiornamento cambia soltanto il checkpoint per registrare gli esiti;
non cambia il prompt né implementa prodotto. Non richiede test Rust. I revisori
confermano separatamente il delta documentale finale su SHA fisso.

Prossimo passo dell’utente: decidere il merge della PR documentale #22. Il runner
segue già il prompt attivo e prosegue autonomamente al risveglio esistente sui
soli figli autorizzati quando tutti i gate sono soddisfatti. Nessun avvio
istantaneo promesso, nessun merge o deploy eseguito da questa consegna.
