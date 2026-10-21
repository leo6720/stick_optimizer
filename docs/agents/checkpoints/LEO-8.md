# Esecuzione asincrona Stick Optimizer — LEO-8

Ticket: https://linear.app/leochist/issue/LEO-8/mostrare-un-solo-strato-completo-di-stick-per-cassetto-nel-popup
Specifica accettata: `docs/agents/specs/LEO-8.md`, dalle decisioni del grill già
riportate nel ticket pronto il 20 ottobre 2026. Nessuna estensione funzionale.

- Runner: Codex cloud, 21 ottobre 2026.
- Baseline main: `05f6f1e58d8d21a9de49dbb2b95f7774a6584470`.
- Branch: `codex/leo-8-async`.
- Head: commit contenente questo checkpoint (`git rev-parse HEAD`).
- PR: unica verso main dal branch sopra; numero/link e head esatto sono
  registrati nel ticket e nella descrizione della PR alla pubblicazione.
- Fase: implementazione verificata; revisione separata e collaudo umano Windows.
- Cicli revisione: ciclo 1 sul commit contenente questo checkpoint; esiti dei
  revisori e contatore finale persistiti nel ticket e nella PR sullo stesso SHA.
- Rilievi aperti prima della revisione: nessuno identificato.

## Verifiche realmente eseguite

Rust 1.99.0 e Cargo.lock invariato. Test-first: la nuova aspettativa su un solo
strato fallisce sulla baseline (7 rettangoli invece di 1), poi passa con la
modifica minima alla sola rappresentazione.

- fmt e `git diff --check`: superati.
- Check e Clippy workspace/all-targets Linux con `eframe/x11,ui-preview`, locked
  e Clippy `-D warnings`: superati.
- Test workspace Linux con `eframe/x11,ui-preview`, locked: **49 superati**,
  zero fallimenti (11 core, 32 desktop egui, 6 IO).
- Check e Clippy workspace/all-targets/all-features MSVC Windows con
  `--target x86_64-pc-windows-msvc --locked` e `-D warnings`: superati come
  cross-check, non come esecuzione di test Windows.
- Build debug e release workspace Linux con `eframe/x11,ui-preview`, locked:
  superate. Nessuna build release Windows dichiarata.
- Quattro catture desktop Linux/Xvfb acquisite e ispezionate: `detail`,
  `detail-carry`, `detail-compact` (760×620), `detail-scaled` (zoom 150%).
  Usano risultati veri dell’esempio incorporato; non modificano dati utente.

Il test pubblico schematic usa candidati reali con uno/due cassetti, gruppo 1
e gruppi multipli, con/senza divisori, ultimo strato parziale e più di 32
strati reali. Controlla contorni B, passo, divisori, fila completa a y=0 e
conservazione del candidato. Il test desktop usa gli eventi egui per cambiare
formato nel popup e controlla rettangoli, quote e risultati invariati.
Nessuna modifica a ricerca, ranking, formule, persistenza o CSV.

## Limite e prossima azione

Completare le due revisioni separate su baseline/head fissi e pubblicare la PR.
I risultati della revisione sono nella PR e nel ticket: non dipendono da file
temporanei di questa macchina. Se emergono rilievi, correggere nel perimetro e
incrementare il contatore senza azzerarlo; non superare tre cicli irrisolti.

Il collaudo fisico Windows e monitor/DPI nativi non è eseguibile da questo host
Linux. Dopo la revisione mantenere PR draft, In Review + ready-for-human e
rimuovere ready-for-agent. Non riprendere automaticamente in attesa umana.
Sul PC, eseguire test/build Windows e la checklist LEO-8 di docs/development.md:
aprire formati con uno/due cassetti, divisori e riporto, cambiare formato,
controllare fila completa, vuoto fino a B, quote e dettagli reali, finestra
compatta e zoom/DPI. Registrare esiti e decidere il merge separatamente.
Nessun merge, deploy o nuova automazione autorizzati.
