# Esecuzione asincrona Stick Optimizer — LEO-9

Ticket e definizione accettata il 16 ottobre 2026:
https://linear.app/leochist/issue/LEO-9/mostrare-una-barra-di-avanzamento-durante-il-calcolo-delle-soluzioni

- Runner: Codex cloud, 17 ottobre 2026.
- Baseline main: `05f6f1e58d8d21a9de49dbb2b95f7774a6584470`.
- Branch: `codex/leo-9-async`; PR verso `main`, da pubblicare.
- Fase: implementazione e regressioni completate; controlli finali e revisione.
- Head: commit contenente questo checkpoint, da risolvere con `git rev-parse HEAD`.
- Cicli di revisione: 0 completati; prima coppia di revisioni da eseguire.
- Rilievi aperti: nessuno già identificato; non dichiarata revisione superata.

## Verifiche

Test-first: riprodotto il bypass di Ctrl+N (test rosso); corretto e verificato
verde. Un secondo test ha riprodotto la scrittura nel campo già a fuoco sotto
il popup: corretto preservando il focus dei pulsanti modali da tastiera.

Eseguiti con Rust 1.99.0 e Cargo.lock invariato:

- Suite workspace Linux con `--features eframe/x11,ui-preview --locked`:
  passaggio precedente verde; esecuzione finale con test aggiuntivo di chiusura
  nativa in corso al primo checkpoint.
- `cargo fmt --all -- --check`, Clippy host con tutte le feature workspace e
  `eframe/x11`, `-D warnings`: superati.
- Check e Clippy Windows MSVC con `--all-targets --all-features --locked`:
  superati come cross-check; nessun test Windows eseguito.
- Build debug e avvio desktop Linux/Xvfb: superati. Sei catture locali del
  popup (ricerca, preparazione, finalizzazione, annullamento, compatta, zoom
  150%) da fixture visive esplicitamente separate dal motore di produzione.
- Build release host: in corso al primo checkpoint.

La suite osserva eventi egui, contatori reali del caso di riferimento (40
combinazioni/16 Pareto), blocco input già a fuoco, menu/scorciatoie, singolo
worker, Annulla/X, tastiera del popup, chiusura nativa sintetica, scarto di
risultati dopo annullamento, errori e limiti. Le fixture di avanzamento congelate
non sono una misura di velocità né una prova dei tempi del motore.

## Prossima azione e limiti

Completare controlli e due revisioni separate su baseline/head fissi; registrare
esiti e relativo head nel ticket e nella PR. Non effettuare merge o deploy.
Il collaudo fisico Windows (interazioni, più monitor/DPI e calcolo abbastanza
lungo da osservare il popup reale) non è disponibile su questo host Linux.
Per riprenderlo usare `docs/development.md`, senza riprendere automaticamente
un ticket marcato ready-for-human. Nessuna modifica a formule, ranking,
limiti, schemi dei file o automazioni.
