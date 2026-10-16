# Istruzioni per Codex

## Migrazione Rust: istruzioni prevalenti per il nuovo codice

La versione di sviluppo è la beta Rust su `main`, integrata tramite PR #17.
`rust-rewrite` conserva il percorso storico della migrazione;
`python-beta-reference` conserva lo stato Python iniziale. Leggere anche `docs/rust-migration.md` prima di
lavorare. Il workspace Rust ha tre componenti: `crates/core` (dominio e ricerca),
`crates/io` (file e preferenze) e `crates/desktop` (GUI). Il passo 4 implementa
dominio, validazione e ricerca esatta in `core`, collegati alla GUI egui/eframe.
Le bozze in `draft.rs` si convertono in uno snapshot validato dal core. I
risultati sono reali. La fase 5 aggiunge studi `.stic` versionati, preferenze,
CSV, filtri e sezioni geometriche. Leggere anche `docs/rust-engine.md` e
`docs/rust-files.md` per precisione, limiti, formati e controlli di correttezza.

- Conservare la disposizione Python: sidebar sinistra con Dati MT e tabella
  gerarchica stick/formati; area principale destra con risultati e dettagli
  sotto. I parametri si aprono da Modifica, senza sostituire l'area di lavoro.
  Non introdurre una navigazione per pagine tra input e output senza richiesta.
- Per lo stile seguire `docs/ui-design.md`: riferimento Marchesini Group,
  rosso/bianco/grigi, mantenendo la gerarchia funzionale Python e i suoi schemi.

- Le nuove attività partono da `main` aggiornato e aprono PR verso `main`.
  Usare branch dedicati con ID Linear; i vecchi branch della migrazione sono storici.
- Il core non dipende dalla GUI o dai formati di file; la GUI non ricalcola
  formule, metriche o vincoli fisici.
- Nessuna compatibilità richiesta con vecchi `.sop`, CSV o preferenze. Per
  nuovi formati definire uno schema versionato e verificarne il round trip.
- Conservare significato fisico, millimetri ed esclusione delle teste con
  raggruppamento 1. Documentare ogni cambiamento intenzionale. Definire
  arrotondamenti e confronto dei passi; Python e Rust possono differire.
- Usare la toolchain versionata; eseguire `cargo fmt --all -- --check`,
  `cargo check --workspace --all-targets --locked`,
  `cargo clippy --workspace --all-targets --locked -- -D warnings`,
  `cargo test --workspace --locked` e
  `cargo build --workspace --release --locked` quando pertinenti.
- Zero test nella struttura iniziale non certificano il motore. Quando il
  motore esiste, verificare casi deterministici, vincoli, Pareto e risultati
  contro un riferimento esatto piccolo prima di ottimizzare.
- Per la GUI eseguire prove desktop; riportare ciò che non è stato verificato.
- Non modificare dati utente per prove, non usare `launcher.py`, non introdurre
  workflow GitHub Actions, non eseguire merge o deploy autonomamente.

Le istruzioni Python seguenti continuano a valere per il codice Python di
riferimento. Il requisito legacy di apertura dei `.sop` non si applica alla
nuova app Rust, secondo l'indicazione esplicita dell'utente.

## Contesto

Questo repository contiene un'app desktop Rust per confrontare configurazioni geometriche di trasferimento degli stick dalla macchina stickpack all'astucciatrice. Il codice Python/Tkinter resta come riferimento storico. Il codice produce candidati per ciascun formato, combina quelli con lo stesso passo astucciatrice, applica un filtro Pareto e ordina le soluzioni con un punteggio pesato. È uno strumento di supporto alla progettazione: una soluzione calcolata richiede comunque verifica meccanica e di processo.

Prima di modificare il codice, leggere `README.md`, `docs/architecture.md` e la sezione pertinente di `docs/development.md`. Le istruzioni valgono per l'intero repository. Le istruzioni esplicite dell'utente prevalgono.

## Mappa del codice Python di riferimento

- `models.py`: dataclass di input, candidati e soluzioni.
- `defaults.py`: costanti, configurazioni ammesse e dati dimostrativi incorporati.
- `validation.py`: validazione degli input prima della ricerca.
- `optimizer.py`: generazione dei candidati e combinazioni tra formati.
- `scoring.py`: penalità, aggregazione e punteggio.
- `pareto.py`: confronto di dominanza.
- `project_io.py`: serializzazione dei progetti applicativi `.sop`.
- `export.py`: esportazione CSV.
- `gui/`: interfaccia, parsing, tabelle e presentazione dei risultati.
- `main.py`: ingresso dell'app; `launcher.py`: aggiornamento autonomo dal branch remoto e avvio.

## Regole di modifica

1. Tenere formule e vincoli tecnici nel core (`optimizer.py`, `scoring.py`, `validation.py`); la GUI deve leggere i risultati, non ricalcolarli.
2. Quando cambia un campo di `Candidate`, `Solution`, `GlobalSettings` o `Weights`, controllare tutti i consumatori: GUI, `.sop`, CSV, dati predefiniti e documentazione. Verificare l'apertura dei progetti salvati se si altera il formato `.sop`; non presumere compatibilità automatica.
3. Quando cambia una definizione di formato cassetto o testa robot, controllare insieme `build_solution()`, filtro Pareto, tabelle GUI, popup e CSV. Le teste con `grouping == 1` sono escluse dal conteggio dei tipi di testa.
4. Conservare unità in millimetri e significato dei campi. Non cambiare pesi, soglie o vincoli fisici senza spiegare l'effetto sui risultati.
5. `user_defaults.json` è tracciato e viene letto o sovrascritto dall'app. Considerarlo dato utente di esempio: non modificarlo per esperimenti o test, salvo richiesta esplicita.
6. Non eseguire `launcher.py` durante lo sviluppo: può scaricare `main` e sostituire file e directory locali. Usare `python main.py` per la GUI.
7. Non confondere i progetti `.sop` creati dall'app con il progetto/repository Codex.

## Verifica

- Per modifiche al core, usare piccoli casi deterministici con asserzioni che richiedano almeno una soluzione e verifichino il passo comune, i vincoli fisici e i valori coinvolti nella modifica.
- `python tests.py` è un report dimostrativo, non una suite affidabile: i dati predefiniti attuali contengono `B_50` con zero candidati e lo script può uscire con codice 0 senza eseguire le asserzioni finali. Non riportarlo da solo come test superato.
- Per modifiche a `project_io.py`, verificare round trip di `.sop` con risultati presenti; per `export.py`, verificare le colonne CSV coinvolte.
- Per modifiche alla GUI, eseguire una prova manuale di avvio e del flusso interessato su un desktop con Tkinter e Pillow. Se non è disponibile un display, dichiarare esplicitamente che la GUI non è stata verificata.
- Riportare i comandi eseguiti, l'esito, e le limitazioni residue. Non lanciare la ricerca completa con molti formati senza stimarne prima il numero di combinazioni: `itertools.product` le materializza in memoria prima del filtro Pareto.

## Documentazione

Aggiornare `README.md` per installazione e uso; `docs/architecture.md` per flusso, formule e limiti; `docs/development.md` per workflow, verifica e formati dei file. Documentare il comportamento implementato, segnalando separatamente idee o correzioni future.

## Agent skills

Workflow cloud e PC: [docs/agents/workflow.md](docs/agents/workflow.md).
Le skill locali sono in `.agents/skills/<nome>/SKILL.md`; se il client non le
rileva, leggere il file su richiesta. Non presumere disponibilità di slash command.
Tracker: Linear, workspace leochist, team Leochist (LEO), progetto Stick Optimizer.
Leggere [issue-tracker.md](docs/agents/issue-tracker.md) prima di usare Linear e
[triage-labels.md](docs/agents/triage-labels.md) per le etichette.
Dominio: [GLOSSARY.md](GLOSSARY.md) e [domain.md](docs/agents/domain.md).
In revisione leggere anche [CODING_STANDARDS.md](CODING_STANDARDS.md).
Verifiche: `python scripts/check.py --plan`; istruzioni e limiti nel workflow.
