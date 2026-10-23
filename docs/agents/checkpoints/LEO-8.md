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

## Preparazione merge autorizzata — 23 ottobre 2026

Questa sezione aggiorna fase e prossima azione delle note storiche sopra.
Fonte: richiesta utente «prepariamole per il merge. le verifiche manuali le
farò in seguito vediamo se ci sono conflitti e correggiamoli», registrata
nel ticket. Ripresa tecnica della PR20 esistente; nessun merge su main,
deploy, nuova PR o chiusura Done autorizzati. Le prove manuali native Windows,
input fisici e monitor/DPI sono rinviate dall’utente, non superate.
Nessun dimensionamento retroattivo inventato per l’attività già In Review.

- Implementatore nuovo e isolato: `/root/leo8_merge_update`, creato dal
  coordinatore con `fork_turns: none`; nessuna cronologia completa ereditata.
- Baseline storica: `05f6f1e58d8d21a9de49dbb2b95f7774a6584470`.
- Head precedente e primo ciclo di revisione PASS preservati:
  `53e1f12a564c56e4e7194ae0cdc931bbbcc9e957`.
- Baseline integrazione main: `509003851451f5ccc8280f4bfc4a577611e266e5`.
- Worktree nuovo: `/workspace/stick-leo8-merge-update`; branch locale
  `codex/leo-8-merge-update`, pubblicato normalmente sul branch PR esistente
  `codex/leo-8-async`. Head esatto: commit contenente questa sezione;
  `git rev-parse HEAD`, riportato anche nel successivo handoff.
- Merge di main nel branch, senza riscrittura della storia. Cinque conflitti
  risolti: geometry.rs conserva coordinate effettive e un solo strato;
  app.rs conserva entrambi i test LEO-7/8; development/rust-files/ui-design
  conservano documentazione main e comportamento LEO-8.
- Nuova integrazione: round 1, cumulativo 2 completato con due revisioni
  separate PASS sullo stesso main/head congelato; dettagli sotto. Nessun
  rilievo bloccante o suggerimento residuo. Il ciclo storico PASS resta valido
  come evidenza storica e non viene azzerato.

### Verifiche della ripresa realmente eseguite

Rust 1.99.0; Cargo.lock, toolchain e manifest invariati rispetto a main.

- fmt, diff whitespace, check e Clippy `-D warnings` workspace/all-targets/
  all-features host con backend `eframe/x11`: PASS.
- `cargo test --workspace --all-features --features eframe/x11 --locked`:
  **81 test PASS** (30 core, 40 desktop, 11 IO). Compresi cambio formato
  sintetico, popup teste LEO-7, ricerca esatta e CSVv2 ereditati da main.
- Test pubblico `single_layer_view_preserves_real_geometry_and_solution_data`
  rafforzato per tutte le coordinate effettive stick/divisori, poi rieseguito:
  PASS. Il test group4 ereditato usa coordinate attese indipendenti.
- Build workspace release con `eframe/x11`: PASS.
- Build workspace debug con `eframe/x11,ui-preview`: PASS.
- Check e Clippy MSVC workspace/all-targets/all-features locked: PASS come
  compilazione cross-target, senza esecuzione o build nativa Windows.
- Sette catture Linux/Xvfb reali acquisite e ispezionate:
  `/workspace/scratch/leo8-integration-previews/{detail,detail-carry,detail-compact,detail-scaled,detail-bottom,detail-carry-bottom,head-popup-compact}.png`.
  Normale: due cassetti con due stick ciascuno e vuoto fino a B35;
  riporto: quattro stick, divisore e gap strutturale reali, B59 e dettagli
  9 strati/pila54/ciclo2; compatta760×620 e zoom150% leggibili con Chiudi
  accessibile. Scroll in fondo conserva coordinate/giochi/teste reali.
  Il popup teste LEO-7 resta visibile e scorrevole nella vista compatta.
- Compatibilità con PR19 finale `96f0e72738739c4a5fa727c4872bb76a2493ff9c`:
  merge-tree scratch pulito dopo spostamento delle sole sezioni documentali8
  lontano dai punti di inserimento9. Nessun codice o commit9 incorporato in20.
  Verifica finale bidirezionale sugli head esatti:
  PR20 `a71048264f4e175d3e982e00c392e377a760d50a` e PR19
  `96f0e72738739c4a5fa727c4872bb76a2493ff9c`: entrambi gli ordini di
  `git merge-tree --write-tree` terminano con codice 0 e lo stesso tree
  `8db3a843622cc5b00717068cef17cdaa9544a0e3`.
  Log `/workspace/scratch/leo8-19-final-merge-tree.log`.
- Il tree combinato è stato estratto soltanto nello scratch
  `/workspace/scratch/leo8-19-combined`, senza merge effettivi o nuovi branch.
  `cargo test --workspace --all-features --features eframe/x11 --locked
  --manifest-path /workspace/scratch/leo8-19-combined/Cargo.toml`:
  **89 test PASS** (31 core, 47 desktop, 11 IO), zero fallimenti.
  Log `/workspace/scratch/leo8-19-combined-tests.log`. Questa suite verifica
  insieme modal LEO-9, strato LEO-8 e popup teste LEO-7 sul tree indicato;
  non costituisce esecuzione nativa Windows.

### Esito finale delle revisioni separate

Baseline comune `509003851451f5ccc8280f4bfc4a577611e266e5`, head prodotto
comune `a71048264f4e175d3e982e00c392e377a760d50a`:

- Specifica: `/root/leo8_merge_spec`, contesto separato dall’implementatore,
  **PASS**, nessun finding. Ha rieseguito indipendentemente i due test
  `single_layer` e diff whitespace, e ispezionato tutte le sette PNG.
- Standard/correttezza: `/root/leo8_merge_standards`, contesto separato
  dall’implementatore, **PASS**, nessun finding. Ha rieseguito due test mirati,
  suite workspace completa **81 PASS** (30 core, 40 desktop, 11 IO) e diff
  whitespace, e ispezionato tutte le sette PNG.

Questo aggiornamento finale modifica solo il checkpoint, registra gli esiti
ricevuti dal coordinatore e precisa le feature delle build; nessun cambiamento
prodotto o nuovo round di correzione. Il coordinatore farà verificare il delta
documentale agli stessi revisori nel medesimo round 1/cumulativo 2.

Fase: preparazione tecnica e revisioni prodotto concluse PASS; resta il controllo
del delta documentale finale e la decisione separata dell’utente sul merge.
Il coordinatore mantiene ticket e PR In Review; nessun merge main, deploy o
Done effettuato. Windows nativo, input fisici e monitor/DPI restano rinviati
per decisione esplicita dell’utente e non sono dichiarati superati.
