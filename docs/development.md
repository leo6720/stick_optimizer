# Sviluppo e verifica

## Workspace Rust sul branch di migrazione

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

Nella fase 4 il core contiene il motore esatto; IO resta predisposto. I test
core confrontano campi e classifiche con fixture Python su casi piccoli,
verificano vincoli, arrotondamenti e Pareto contro un prodotto cartesiano
materializzato indipendente. I test desktop verificano snapshot, worker,
annullamento, risultati obsoleti, click su Calcola tramite eventi egui,
filtri, Ctrl+A/Tab, dimensioni ridotte e virtualizzazione.

Per rigenerare le fixture del riferimento: `python scripts/python-reference.py`.
Lo script stima prima le combinazioni (massimo 40 nei quattro casi), richiede
risultati e non legge né scrive user_defaults.json. La suite Rust usa i file
incorporati e non richiede Python durante i test o nell'eseguibile.

Per verificare anche la funzione di anteprima e acquisire dieci schermate native:

```powershell
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
.\scripts\preview-gui.ps1
```

Lo script usa Cargo in `$env:USERPROFILE\.cargo\bin`, compila la funzione
`ui-preview`, avvia e chiude il prototipo per ciascuna vista e salva PNG in
`work/phase4` (non versionata). Controllare visivamente input e
risultati reali nella stessa area di lavoro, parametri, tipi cassetto, tipi testa,
diagnostica senza candidati/senza passo comune, finestra compatta, zoom UI 150%
e molte righe. Per ripetere soltanto una vista:
`.\scripts\preview-gui.ps1 -SkipBuild -Modes results`. Controllare che la sidebar resti visibile accanto
all'output anche dopo aver mostrato i risultati. Lo zoom UI è una prova
di layout, non una verifica di ogni configurazione DPI Windows. La build
release normale non abilita questa funzione. Completare le prove con editing,
selezione, scorrimento e conferme manuali; gli eventi sintetici non sostituiscono
la verifica fisica di tastiera e monitor.

Le nuove istruzioni Rust prevalgono sui percorsi Python soltanto per i componenti Rust. Nessuna
compatibilità con vecchi `.sop` è richiesta. Per preservare il comportamento
tecnico leggere anche `docs/architecture.md` e `docs/rust-migration.md`.

Le PR della migrazione devono avere base `rust-rewrite`, non `main`. Le attività
successive devono rispettare dipendenze e traguardi del piano; il merge resta
una decisione umana. Non eseguire aggiornamenti dal vecchio launcher.

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
