# LEO-17 — checkpoint di implementazione

- Data: 21 ottobre 2026.
- Ticket: https://linear.app/leochist/issue/LEO-17.
- Parent accettato: https://linear.app/leochist/issue/LEO-16.
- Fase: implementazione e verifiche completate; attesa di due revisioni indipendenti.
- Proprietario implementazione: `/root/leo17_implementer`.
- Isolamento effettivo: `spawn_agent(fork_turns: "none")`, contesto nuovo per il solo LEO-17.
- Claim Linear eseguito dal coordinatore; In Progress, nessuna chiusura da questo agente.
- Baseline main verificata live e aggiornata: `05f6f1e58d8d21a9de49dbb2b95f7774a6584470`.
- Branch: `codex/leo-17-async`; worktree: `/workspace/stick-leo-17`.
- Head implementazione verificata: `692ca9a4542840778eb1e5074970365946e5a93a`.
- Questo checkpoint è il commit successivo, solo documentale; risolvere la head
  del branch per la revisione completa, senza limitarsi al commit di implementazione.
- Cicli di revisione indipendente consumati: **0**. Nessuna certificazione indipendente dell'implementatore.
- Rilievi aperti di implementazione: nessuno noto; revisori indipendenti ancora da eseguire.

## Contratto e perimetro

API pubblica `final_head_geometry` sull'intero insieme di formati assegnati,
coordinate e ID deterministici, gruppi 2/3/4 e divisori secondo ADR 0001.
Minimo comune e limite per ogni formato, senza transitività a coppie; centrali
g4 fissi a passo di prelievo, divisore centrale e avvicinamenti entro le coppie.
Parametro `max_extra_spacing` finito/non negativo, default 5 mm; divisore
incorporato 7,5 mm. DTO e draft conservano il parametro senza nuovo controllo GUI.

Schema studi/preferenze 2; migrazione Rust 1 solo in memoria, aggiungendo il
parametro mancante e conservando tutti gli altri valori (divisore incluso).
Nessuna riscrittura in lettura; salvataggio sempre esplicito. Versioni/campi
non supportati producono errore. Il percorso preferenze storico resta valido.

LEO-18 (ricerca/assegnazioni/cassetti/ranking), LEO-19 (editor/dettagli/CSV),
LEO-7 (popup testa) restano fuori perimetro. La ricerca attuale non usa questa
API e non è dichiarata corretta per la condivisione delle posizioni finali.
Le sole modifiche in `search.rs` e nei test motore fissano il divisore storico
2 mm degli input fixture Python, lasciando formule e attesi invariati.

## Test-first e verifiche

1. Test pubblico iniziale: RED con `HeadFormat` e `final_head_geometry` assenti;
   poi GREEN su esempi approvati, canali fissi e ripetizioni di tutti i gruppi.
2. Test migrazione: RED effettivo a runtime, `missing field max_extra_spacing`
   aprendo lo schema 1; GREEN dopo migrazione. Confronto dell'intero JSON
   preservato (solo versione e nuovo parametro aggiunti), file sorgenti invariati.
3. Test pubblici: uguaglianza/vicinanza/incompatibilità, limite esatto/oltre,
   tre formati congiuntamente e catena non transitiva, divisori misti,
   permutazione ID, precisione f64, input invalidi, g4 centrale fisso.
4. Round trip core/DTO/draft/studio/preferenze con valori personalizzati e
   controlli di vecchio schema, campi mancanti/inattesi e versioni sconosciute.

Toolchain **Rust 1.99.0**, Cargo.lock invariato. Cache riusata senza alterare
le altre worktree; nessun dato utente, launcher, workflow GitHub Actions,
merge, deploy o nuova automazione.

Comandi passati dalla worktree, con Cargo/Rustup nelle directory cache della
sessione e `CARGO_TARGET_DIR=/workspace/stick-rust-validation/target`:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features --features eframe/x11 --locked --offline
cargo clippy --workspace --all-targets --all-features --features eframe/x11 --locked --offline -- -D warnings
cargo test --workspace --all-features --features eframe/x11 --locked --offline
cargo build --workspace --release --features eframe/x11 --locked --offline
cargo check --workspace --all-targets --all-features --target x86_64-pc-windows-msvc --locked --offline
```

Suite completa: **57 test passati** (core 4 unit + 6 engine + 7 API testa;
desktop 32; IO 8). Dopo le ultime asserzioni su limite numerico/migrazione
e la correzione meccanica Clippy dei test, ripetuti core+IO: **25 passati**.
Nessuna modifica desktop dopo il passaggio della suite completa.
`git diff --check` passato. La prima esecuzione Clippy segnalava due clone
superflui nei test, corretti senza soppressioni.

Build di anteprima Linux con `ui-preview,eframe/x11` passata. Avvio e cattura
in Xvfb delle viste `parameters` e `results`, controllate visivamente:
divisore 7,5 visibile, nessun controllo inerte nuovo, 16 risultati reali,
sidebar e dettagli conservati. Artefatti locali non versionati:
`/workspace/scratch/leo17-qa/parameters.png` e `results.png`.
Il primo avvio Xvfb nel sandbox senza socket falliva; ripetuto con accesso
socket autorizzato, listener TCP disabilitato, catture riuscite.

## Limiti e prossimo passo

Il controllo MSVC è compilazione cross-target, **non** build/esecuzione Windows.
Nessuna nuova prova fisica Windows di selettori, mouse/tastiera o monitor/DPI.
La GUI Linux non sostituisce quelle prove. Nessun benchmark o simulazione.

Il coordinatore deve avviare due nuovi revisori su baseline/head fisse,
registrare gli esiti e l'eventuale contatore cicli (massimo 3 irrisolti), quindi
gestire PR e stato Linear. Nessun merge/deploy autorizzato dal checkpoint.
