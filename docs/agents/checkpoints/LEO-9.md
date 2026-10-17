# Esecuzione asincrona Stick Optimizer — LEO-9

Definizione accettata con l’utente il 16 ottobre 2026:
https://linear.app/leochist/issue/LEO-9/mostrare-una-barra-di-avanzamento-durante-il-calcolo-delle-soluzioni

- Runner: Codex cloud, 17 ottobre 2026.
- Baseline main: `05f6f1e58d8d21a9de49dbb2b95f7774a6584470`.
- Branch: `codex/leo-9-async`.
- PR: https://github.com/leo6720/stick_optimizer/pull/19, verso `main`, draft.
- Head: commit contenente questo checkpoint (`git rev-parse HEAD`); SHA esatto
  e resoconti delle revisioni persistiti anche nel ticket e nella PR.
- Fase: controlli automatici completati; seconda revisione del fix e attesa
  collaudo fisico Windows. Non effettuare merge o deploy.
- Cicli revisione: 1 completato, ciclo 2 sul commit contenente il fix.
- Rilievi del primo ciclo: unico difetto P2 nell’harness del nuovo test di
  chiusura (TexturesDelta non scartato); corretto, test mirato e suite verdi.
  Nessun difetto di prodotto rilevato dai due revisori separati.
- Rilievi del secondo ciclo: consultare resoconti sullo SHA esatto in PR/Linear.

## Verifiche eseguite

Rust 1.99.0, Cargo.lock invariato. Test-first: riprodotti Ctrl+N che apriva
Nuovo studio durante la ricerca e input già a fuoco che riceveva testo sotto
il popup. Correzioni verificate senza impedire la tastiera nei comandi modali.

- `cargo fmt --all -- --check`: superato.
- `cargo check --workspace --all-targets --features eframe/x11,ui-preview --locked`:
  superato su Linux.
- `cargo clippy --workspace --all-targets --features eframe/x11,ui-preview --locked -- -D warnings`:
  superato su Linux, ripetuto dopo il fix del test.
- `cargo test --workspace --features eframe/x11,ui-preview --locked`:
  55 test superati (11 core, 38 desktop egui, 6 IO), zero fallimenti.
- `cargo check` e `cargo clippy` workspace con `--all-targets --all-features
  --target x86_64-pc-windows-msvc --locked` (Clippy con `-D warnings`):
  superati come cross-check; nessun test Windows eseguito.
- Build debug con `eframe/x11,ui-preview`, avvio e sei catture Linux/Xvfb:
  ricerca, preparazione, finalizzazione, annullamento, 760×620 e zoom 150%.
  Ispezionate; sono fixture visive congelate, non misure del motore reale.
- `cargo build --workspace --release --features eframe/x11,ui-preview --locked`:
  superato su Linux (3 minuti e 41 secondi; non è un eseguibile Windows).
- `git diff --check`: superato.

I test osservano eventi egui, contatori reali del caso di riferimento (40
combinazioni/16 Pareto), blocco input già a fuoco, menu/scorciatoie, singolo
worker, Annulla/X, tastiera del popup, chiusura nativa sintetica, scarto di
risultati dopo annullamento, successo, errori e limiti. Nessun cambiamento a
formule, ranking, limiti del motore, schemi dei file o automazioni.

## Blocco umano e prossima azione

Questo host Linux non può certificare interazioni e DPI nativi Windows.
Mantenere la PR draft e il ticket in attesa umana con ready-for-human dopo le
revisioni; rimuovere ready-for-agent. Non riprendere automaticamente il ticket.

Sul PC Windows, dal branch della PR:

1. Eseguire i controlli workspace Windows di `docs/development.md`, inclusi
   test nativi e build release.
2. Avviare un calcolo abbastanza lungo: verificare popup reale e repaint,
   input/menu/Ctrl+N/O/S bloccati, Annulla e X, nessuna classifica parziale,
   ripristino dell’interazione dopo il termine effettivo del worker.
3. Ridimensionare l’app, provare zoom e monitor/DPI; testo e pulsanti devono
   restare accessibili. La X principale annulla la ricerca lasciando l’app
   aperta; una successiva chiusura segue le normali conferme.
4. Registrare gli esiti sulla PR e decidere il merge separatamente.

Le anteprime sono ripetibili con le modalità `progress*` documentate in
`docs/development.md`. La documentazione di questo checkpoint resta sul branch;
PR e Linear conservano gli esiti finali dei revisori e l’head esatto senza
dipendere dai file temporanei della macchina cloud.
