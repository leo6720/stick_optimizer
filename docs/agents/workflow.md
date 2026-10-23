# Workflow di sviluppo con agenti

Configurazione iniziale del 15 ottobre 2026 per LEO-5, aggiornata per
l'intervista originale il 23 ottobre su richiesta dell'utente.
Grilling e domain-modeling sono copie integrali della fonte; i due ingressi
grill-me/grill-with-docs adattano il caricamento ai client cloud e PC.
Le altre skill restano adattamenti locali, non un'installazione integrale
dell'intero pacchetto né una trascrizione del video. Fonte fissata:
https://github.com/mattpocock/skills/tree/49dd158d1076134a641b33efb035946536778336
File importati, licenza e differenze: [.agents/skills/UPSTREAM.md](../../.agents/skills/UPSTREAM.md).

## Avvio asincrono

Dopo grill, accettazione e dimensionamento in ticket, Todo + ready-for-agent
avviano sviluppo e revisione autonoma tramite il runner configurato.
Protocollo, ruoli, checkpoint e limiti in [async-workflow.md](async-workflow.md).
Il merge resta manuale. I figli esplicitamente autorizzati di uno split accettato
diventano pronti al risveglio esistente dopo integrazione e verifica delle
dipendenze su main, secondo il registro e i gate di async-workflow.md.
Senza tale autorizzazione persistita il runner non inventa ready.

## Uso quotidiano

Aggiungi in Linear un'idea, un bug o un esempio nel progetto Stick Optimizer.
Poi chiedi, per esempio:
- «Chiarisci LEO-N con grill-with-docs».
- «Metti alla prova questa idea con grill-me» (sola intervista, senza scrivere file o Linear).
- «Trasforma questa richiesta in una specifica con to-spec».
- «Dividi la specifica in ticket con to-tickets».
- «Implementa LEO-N».
- «Rivedi la PR rispetto alla specifica e agli standard».
- «Fai la retrospettiva della sessione».

Le dieci skill locali sono in .agents/skills/: grill-me, grill-with-docs,
grilling, domain-modeling, to-spec, to-tickets, implement, tdd, code-review, retro.
Aprire il relativo SKILL.md e tutte le dipendenze che indica.
Su client che le rilevano automaticamente si possono usare i comandi del client.
Se il cloud non le mostra nel catalogo, chiedere esplicitamente di leggere il
file: la presenza nel repo non garantisce registrazione o comandi slash.

## Flusso

1. Su richiesta dell'utente, applicare grilling integralmente: albero delle decisioni,
   round con l'intera frontiera di domande indipendenti e una raccomandazione per
   ciascuna; attendere le risposte prima delle domande dipendenti. Prima leggere
   codice, glossario e documentazione e delegare la ricerca dei fatti reperibili.
   Decisioni già accettate restano risolte; approfondire le assunzioni mancanti
   fino alla frontiera vuota e alla conferma dell'intesa, senza allargare il perimetro.
   Per il lavoro sul repo usare grill-with-docs: carica anche domain-modeling,
   aggiorna termini durante il chiarimento e ADR soltanto quando qualificati,
   conserva tutte le decisioni nel ticket e richiede conferma prima di dichiararle accettate.
2. Scrivere la specifica con criteri osservabili e confini di test.
3. Stimare i token della specifica e calcolare N = ceil(totale / 150.000),
   minimo uno. Scomporre e stimare i ticket secondo [token-sizing.md](token-sizing.md),
   con criteri e dipendenze native Linear. Pronti solo i ticket esecutivi dimensionati.
4. Ogni ticket si sviluppa in un contesto nuovo, con branch e PR propri.
   Lavorare una modifica alla volta; test-first per comportamento e bug,
   controlli proporzionati per documentazione e cambi cosmetici.
5. Rivedere il diff rispetto a uno SHA fisso su due assi: specifica e standard.
   Un'autorevisione va dichiarata e non sostituisce un revisore indipendente.
6. Aprire una PR draft con ID Linear, test e limiti; In Review fino
   all'integrazione e alla verifica dell'accettazione. Non eseguire merge.
7. Su richiesta fare retrospettiva: proporre miglioramenti a strumenti,
   accesso, documentazione e controlli basati su difficoltà realmente osservate.

## Cloud e PC

La configurazione è versionata nel repo. Ogni ambiente deve usare il branch
che la contiene e avere connessioni proprie a GitHub e Linear.
Non installare una seconda copia delle stesse skill globalmente.
Per il PC basta un checkout aggiornato e un client compatibile con skill locali;
l'accesso ai servizi e la rilevazione delle skill vanno verificati sul PC.

Per le verifiche: python scripts/check.py --plan mostra i comandi senza eseguirli.
python scripts/check.py esegue fmt, check, Clippy, test e build release
con il workspace e tutte le feature; richiede Python 3.10+ solo nello sviluppo,
Rust della toolchain versionata e gli strumenti di compilazione del sistema.
python scripts/check.py --scope core verifica soltanto core e io quando il
desktop non è compilabile nell'ambiente. Non equivale a verifica workspace.
Le prove native Windows restano quelle di docs/development.md; non dichiararle
superate da un cloud privo di desktop. Non cambiare toolchain per aggirare
problemi di rete o dipendenze: registrarli come blocchi.

Dopo il passaggio della beta con PR #17, le nuove attività e PR usano main
come base. rust-rewrite resta storico. LEO-5 parte dalla baseline
codex/ui-refinements a3d62ef: il diff della sola configurazione va revisionato
contro quello SHA, distinto dal diff complessivo della migrazione.
L'integrazione GitHub–Linear e gli eventuali job ogni tre ore già documentati
sono configurazioni separate: questa attività non li abilita né li modifica.
