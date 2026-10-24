# Sviluppo e verifica

Il 15 ottobre 2026 sono stati superati i controlli completi Windows e l'utente
ha accettato la beta, incluse le prove manuali residue. Il
[verbale di collaudo](rust-beta-validation.md) distingue test eseguiti,
prove non eseguite e accettazione. Le istruzioni per un profilo isolato e la
checklist restano disponibili per ripetere le prove e verificare futuri bug.

## Workspace e avvio Rust

Il workspace contiene `stick-optimizer-core`, `stick-optimizer-io` e
`stick-optimizer-desktop`. Il motore non dipende da IO o GUI; IO dipende dal
motore; desktop dipende da entrambi. La GUI usa egui/eframe e egui_extras
0.36.2, con backend Glow e font incorporati. `Cargo.lock` viene versionato.

Usare la toolchain indicata in `rust-toolchain.toml`. Su Windows serve anche
il workload C++ di Visual Studio Build Tools con Windows SDK. Rust 1.95 è il
minimo dichiarato dai manifest; la verifica del minimo è distinta dalla verifica
con la toolchain fissata. L'installazione standard mette Cargo in
`$env:USERPROFILE\.cargo\bin`; dopo l'installazione può servire una nuova shell.

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --release --locked
cargo run -p stick-optimizer-desktop --release --locked
```

Nella fase 4 il core contiene il motore esatto; la fase 5 implementa IO. I test
core confrontano campi e classifiche con fixture Python su casi piccoli,
verificano vincoli, arrotondamenti e Pareto contro un prodotto cartesiano
materializzato indipendente. I test desktop verificano snapshot, worker,
annullamento, risultati obsoleti, click su Calcola tramite eventi egui,
filtri, Ctrl+A/Tab, dimensioni ridotte e virtualizzazione. I test delle rifiniture
verificano clic sul testo di una soluzione, doppio clic sul formato corrispondente,
apertura esplicita ed Esc del popup, invalidazione dei dettagli obsoleti,
Applica/Annulla con risultati esistenti, validazione prima di applicare,
rimozione distinta di stick/formato, scorciatoie e rinomina senza ricalcolo.
I cinque editor da Modifica sono testati tramite apertura dal menu, rifiuto di
valori invalidi, Applica e Annulla; ogni applicazione conserva i campi estranei
al popup. Il ripristino usa le preferenze caricate, oppure i valori incorporati,
e modifica solo la bozza del gruppo. Il salvataggio dei predefiniti resta
esplicito nel menu Modifica.
La fase 5 verifica anche la riapertura con risultati, filtri e selezione,
ID ricaricati, errori senza perdita dello studio corrente, modifiche durante
la riapertura e richiesta di chiusura nativa. I test IO usano solo cartelle
temporanee isolate per studi, preferenze e CSV; non usano dati utente.
Schema e limiti di persistenza in [rust-files.md](rust-files.md).

LEO-24 aggiunge regressioni a eventi/output egui per i footer dei cinque editor
con validazione lunga a720×560/150%, editing/Tab/Esc e conservazione dello
studio; per filtri, formato, testa e conferme con contenuti lunghi verifica
uscita cliccabile e risultati invariati. I selettori formato indicano anche
con una spunta la scelta corrente nei due temi. Le prove esistenti di quote,
strato completo, tutti i prelevatori, parametri/preferenze e worker reale
rimangono distinte dalle fixture visive. Le modalità preview `parameters-error`
e `filters-error` mostrano soltanto errori congelati per verificare l'aspetto.

LEO-19 aggiunge prove pubbliche di quote effettive e CSV per gruppi2/3/4,
con25/26mm, divisori e centrale strutturale; sei scelte separate/condivise
attraversano editor→Applica→Calcola→dettagli→CSV→salva/riapri tramite eventi
egui e IO pubblico. Si confrontano configurazione scelta completa e byte CSV,
non soltanto rango o numero risultati. Le preferenze sono verificate in cartelle
temporanee: lettura e Applica non scrivono; solo salvataggio esplicito modifica
il file; errori di lettura conservano studio e sorgente.

Per rigenerare le fixture del riferimento: `python scripts/python-reference.py`.
Lo script stima prima le combinazioni (massimo 40 nei quattro casi), richiede
risultati e non legge né scrive user_defaults.json. La suite Rust usa i file
incorporati e non richiede Python durante i test o nell'eseguibile.

Per verificare anche la funzione di anteprima e acquisire ventisette schermate native:

```powershell
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
.\scripts\preview-gui.ps1
```

Lo script usa Cargo in `$env:USERPROFILE\.cargo\bin`, compila la funzione
`ui-preview`, avvia e chiude il prototipo per ciascuna vista e salva PNG in
`work/ui-refinements` (non versionata). Controllare visivamente input e
risultati reali nella stessa area di lavoro, parametri, tipi cassetto, tipi testa,
diagnostica senza candidati/senza passo comune, finestra compatta, zoom UI 150%
e molte righe. Le nuove viste coprono filtri, menu File/Modifica, popup del formato
normale/compatto/zoomato, quote dopo lo scorrimento e formato con riporto (`detail-carry-bottom` include
le nuove quote del gruppo4 dopo scorrimento),
studio salvato e riaperto, conferma modifiche non salvate e
messaggio di errore file. L'anteprima `reopened` scrive solo
`work/phase5/qa-roundtrip.stic`; le altre non salvano preferenze.
Per ripetere soltanto una vista:
`.\scripts\preview-gui.ps1 -SkipBuild -Modes results`. Controllare che la sidebar resti visibile accanto
all'output anche dopo aver mostrato i risultati. Lo zoom UI è una prova
di layout, non una verifica di ogni configurazione DPI Windows. La build
release normale non abilita questa funzione. Completare le prove con editing,
selezione, scorrimento e conferme manuali; gli eventi sintetici non sostituiscono
la verifica fisica di tastiera e monitor.
Le catture non interagiscono con i selettori file Windows. Completare a mano
apertura, annullamento, sovrascrittura, preferenze e chiusura con modifiche.

Le viste `parameters`, `parameters-weights`, `parameters-results`,
`parameters-ab` e `parameters-mt` mostrano i cinque popup separati;
`parameters-compact` e `parameters-scaled` verificano quello astucciatrice
in finestra compatta e con zoom al 150%. Le immagini incorporate vengono
adattate al limite delle texture del dispositivo prima del caricamento.

Le nuove istruzioni Rust prevalgono sui percorsi Python soltanto per i componenti Rust. Nessuna
compatibilità con vecchi `.sop` è richiesta. Per preservare il comportamento
tecnico leggere anche `docs/architecture.md` e `docs/rust-migration.md`.

La PR cumulativa #17 porta la beta Rust su `main`, includendo le PR #12–#16.
Le nuove attività partono da `main` aggiornato, con branch dedicati e PR verso
`main`; `rust-rewrite` resta storico. Le attività successive rispettano
criteri e dipendenze dei ticket Linear. Il merge richiede una richiesta
esplicita dell'utente; non usare il vecchio launcher.

## Sviluppo della versione Python di riferimento

## Preparazione

Da una shell nella radice del repository:

```powershell
python --version
python -m venv .venv
.\.venv\Scripts\python -m pip install -r requirements.txt
.\.venv\Scripts\python main.py
```

Usare Python 3.10 o superiore. La GUI usa Tkinter, che deve essere disponibile nella distribuzione Python, e Pillow per caricare le immagini. Il core dell'ottimizzatore usa solo la libreria standard. Su ambienti senza display, eseguire soltanto le verifiche del core.

`launcher.py` è un percorso di distribuzione separato: controlla il commit remoto di `main`, scarica uno ZIP e copia i file nel repository locale prima di avviare `main.py`. Non usarlo per prove su una working tree con modifiche, perché può sovrascriverle.

## Uso dell'app

1. Aprire `main.py` e scegliere **Nuovo Progetto** per uno stato nuovo oppure **Apri Progetto** per un file `.sop` esistente.
2. Inserire i dati MT e i tipi di stick/formati nella tabella gerarchica. Il menu **Modifica** contiene pesi, numero di risultati, obiettivo A/B, dati astucciatrice e offset MT.
3. Premere **Calcola**. La tabella mostra soluzioni ordinate; selezionare una riga per le metriche per formato. Le intestazioni dei risultati aprono filtri.
4. Usare **File → Salva progetto** per conservare input, risultati e filtri in `.sop`. Usare **File → Esporta** per CSV riepilogativo o dettaglio della soluzione selezionata.

Il termine “progetto” nella GUI indica un file di studio `.sop`; è distinto dal progetto Codex che associa questa cartella al repository.

`user_defaults.json` si trova nella radice e viene caricato all'avvio. Le azioni **Sovrascrivi predefiniti** della GUI possono modificarlo. È tracciato in Git: controllare il diff prima di includerlo in un commit.

## Verifica rapida del core

Lo script storico `tests.py` stampa i candidati per tutti i formati di esempio e prova a eseguire l'ottimizzazione. **Non è un gate affidabile**: con i dati incorporati correnti `B_50` non ha candidati per il limite di 12 strati; lo script ritorna senza fallire e quindi può terminare con codice 0 senza verificare le asserzioni. Un semplice `python tests.py` non dimostra che sia stata trovata una soluzione.

Per una prova deterministica che richiede soluzioni, usare un sottoinsieme piccolo dei dati incorporati:

```powershell
@'
from defaults import DEFAULT_FORMATS, DEFAULT_GLOBAL_SETTINGS, DEFAULT_STICK_TYPES, DEFAULT_WEIGHTS
from optimizer import optimize

formats = DEFAULT_FORMATS[:2]
solutions, candidates = optimize(DEFAULT_GLOBAL_SETTINGS, DEFAULT_STICK_TYPES, formats, DEFAULT_WEIGHTS)
assert solutions, {name: len(items) for name, items in candidates.items()}
assert all(len(s.candidates) == len(formats) for s in solutions)
assert all(len({c.cartoner_pitch for c in s.candidates}) == 1 for s in solutions)
assert all(c.occupied_width <= c.cartoner_pitch + 1e-9 for s in solutions for c in s.candidates)
print(f"OK: {len(solutions)} soluzioni")
'@ | python -
```

Questo controllo è uno smoke test del percorso principale, non una copertura completa delle formule. Per ogni modifica aggiungere o eseguire verifiche mirate su casi di confine, incluse le condizioni di scarto e i campi consumati da GUI, `.sop` e CSV. Evitare la ricerca su tutti i formati se il prodotto delle liste dei candidati per passo comune è grande.

## Contratti dei file

- `.sop`: JSON di `project_io.serialize_project`, con `settings`, `weights`, `stick_types`, `formats`, `results`, `active_filters`, `selected_index`. Contiene risultati calcolati, non soltanto gli input. Prima di cambiare schema o nomi dei campi, prevedere una migrazione o segnalare la perdita di compatibilità.
- `user_defaults.json`: preferenze caricate all'avvio dalla GUI, separate dai singoli studi `.sop`.
- CSV riepilogo: campi in `export.SUMMARY_FIELDS`; CSV dettagli: campi in `export.DETAIL_FIELDS`. Controllare questi elenchi quando si aggiungono metriche.
- Immagini `img/` e icona nella radice: risorse della GUI, caricate da `gui/app.py`.

## Checklist per modifiche tecniche

1. Identificare il vincolo o la metrica in `optimizer.py` o `scoring.py` e chiarire se è filtro rigido o penalità.
2. Aggiornare dataclass e validazione se cambiano input o output.
3. Cercare tutti gli usi del campo in `gui/`, `project_io.py`, `export.py` e nei dati di esempio.
4. Eseguire uno smoke test con soluzione presente e casi mirati per la modifica; per la GUI, provare manualmente il flusso interessato.
5. Aggiornare README e documentazione tecnica con comportamento e limiti reali. Riportare verifiche non eseguite.

## Verifica LEO-8: singolo strato nel dettaglio formato

Il test pubblico `core/tests/schematic.rs` usa soluzioni reali e verifica uno/due
cassetti, gruppo 1 e gruppi multipli, con/senza divisori, riporto e più di 32
strati: `schematic` mantiene i dati del candidato e produce un solo strato completo.
Il test desktop cambia formato nel popup tramite eventi egui e controlla quote,
numero di stick disegnati e conservazione dei risultati.

In cloud Linux si eseguono test e anteprime con backend host abilitato soltanto
sulla riga di comando, senza modificare manifest o lockfile:
`cargo test --workspace --features eframe/x11,ui-preview --locked`.
Per le catture servono un display X11 (anche Xvfb) e OpenGL; build con
`cargo build -p stick-optimizer-desktop --features eframe/x11,ui-preview --locked`.
Le modalità di anteprima esistenti `detail`, `detail-carry`, `detail-compact` e
`detail-scaled` coprono la sezione normale, con riporto, compatta e zoomata.
Queste prove Linux non eseguono Windows né certificano monitor/DPI fisici.

Sul PC Windows ripetere i controlli workspace e aprire il dettaglio di formati
con due cassetti/divisori e riporto: verificare una sola fila completa, spazio
vuoto fino a B, quote e valori reali invariati. Cambiare formato, ridimensionare
e provare zoom/DPI. Registrare separatamente ciò che non viene eseguito.

## Workflow con Linear

Il flusso condiviso cloud/PC è in [agents/workflow.md](agents/workflow.md).
Il tracker è configurato in [agents/issue-tracker.md](agents/issue-tracker.md).
Per vedere i controlli Rust: `python scripts/check.py --plan`.
Per eseguirli: `python scripts/check.py`. Le prove GUI native restano necessarie.

## Verifica del popup di calcolo (LEO-9)

Le anteprime `progress`, `progress-preparing`, `progress-finalizing`,
`progress-cancelling`, `progress-compact` e `progress-scaled` mostrano fixture
congelate del popup: servono al controllo visivo, non misurano un calcolo reale.
Su Windows: `.\scripts\preview-gui.ps1 -Modes progress,progress-preparing,progress-finalizing,progress-cancelling,progress-compact,progress-scaled`.

In cloud Linux si possono eseguire test egui e anteprime abilitando il backend
host solo da riga di comando, senza cambiare manifest o lockfile:
`cargo test --workspace --features eframe/x11,ui-preview --locked`.
Per le catture serve un display X11 (anche Xvfb) e OpenGL. Questi controlli non
sono esecuzioni Windows. Il cross-check usa
`cargo check --workspace --all-targets --all-features --target x86_64-pc-windows-msvc --locked`.

Prova nativa residua: avviare un calcolo più lungo dell’esempio; verificare
aggiornamento e repaint, input/menu/Ctrl+N/O/S bloccati, Annulla e X,
“Annullamento…” fino all’arresto, nessun risultato parziale e riutilizzo dopo
successo/errore. Ridimensionare l’app e provare zoom/DPI diversi: testo e pulsanti
devono restare accessibili. La X principale durante la ricerca annulla il
lavoro e lascia l’app aperta; la chiusura successiva segue le normali conferme.

## Verifiche LEO-17

`crates/core/tests/head_geometry.rs` osserva l'API pubblica delle coordinate:
esempi approvati, raggruppamenti/divisori, canali fissi e ripetizioni, insiemi
congiunti e non transitività, limiti esatti/oltre e input invalidi.
I test IO verificano schema 2, migrazione Rust 1 senza scritture o perdita di
campi, parametri personalizzati e versioni invalide; il desktop verifica
anche il round trip delle bozze. Le fixture Python conservano esplicitamente
il proprio divisore storico di 2 mm, indipendentemente dal nuovo predefinito.
Su Linux il manifest Windows non abilita un backend finestra: per verificare
il workspace senza cambiarlo, aggiungere `--features eframe/x11` ai comandi
Cargo pertinenti. Un `cargo check --target x86_64-pc-windows-msvc` resta
un controllo di compilazione, distinto da build/esecuzione native Windows.

## Verifiche LEO-18

`crates/core/tests/head_search.rs` confronta la ricerca pubblica con un
riferimento indipendente che enumera tutte le mappe di etichette e calcola
Pareto materializzato. Attesi fisici espliciti per gruppo4/limiti/frazioni;
non rigenerare le fixture storiche Python dalla nuova implementazione.
I campi invariati restano confrontati con la fixture originale. Test IO/desktop
verificano ricostruzione di teste separate/condivise, cambio contratto motore,
conservazione file e righe testa distinte pur con uguale chiave di prelievo.


LEO-7 verifica quote e coordinate tramite API core pubblica e confronta le primitive
disegnate per g2/3/4, tutti i canali e larghezze reali25/26mm. Eventi egui esercitano
doppio clic su teste diverse, selezione del formato condiviso, Chiudi/Esc e invalidazione.
Le anteprime `head-popup`, `head-popup-g4`, `head-popup-alternate`,
`head-popup-compact`, `head-popup-scaled` e `head-popup-zoom` usano risultati reali;
`head-popup-compact-bottom` e `head-popup-scaled-bottom` mostrano lo scorrimento
in fondo. Sono prove Linux separate dall'esecuzione nativa Windows.

## Verifica LEO-21: fondazione e tema globale

Test IO pubblici con percorsi temporanei: tema separato dai parametri,
round trip, schema/versione/campi invalidi, file assente e sostituzione fallita.
Eventi egui del menu verificano spunta, cambio immediato, riavvio, Nuovo/Apri/Esempio,
file UI corrotto, errore scrittura e sessione ancora utilizzabile. Il confronto
include byte documento/CSV, risultati completi, filtri, selezione, dirty e
revisione; i test esistenti continuano a coprire worker e interazioni.
Le asserzioni geometriche della testa sono conservate nei due temi; i colori
non identificano più arbitrariamente i prelevatori. Contrasto dei token
verificato rispetto a4.5:1 per testi e3:1 per contorni/disegni essenziali.

Solo nella feature QA `ui-preview`, le modalità già esistenti accettano
`STICK_OPTIMIZER_PREVIEW_THEME=dark` (default chiaro),
`STICK_OPTIMIZER_PREVIEW_SIZE=720x560` e `STICK_OPTIMIZER_PREVIEW_SCALE=1.5`.
La modalità `components` mostra gli stessi componenti egui e i loro stati.
Queste opzioni non leggono né scrivono preferenze personali. Le immagini
tecniche JPG originali mantengono il proprio sfondo incorporato nei due temi.
Le fixture `progress*` restano congelate per l'aspetto: il worker è verificato
separatamente dagli eventi e dalle regressioni esistenti. Le prove Linux/Xvfb
non certificano input fisico, file picker o più monitor/DPI Windows.
