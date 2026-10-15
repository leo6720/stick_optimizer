# Collaudo della beta Rust prima del passaggio a main

Attività: LEO-12. Data: 15 ottobre 2026.
Stato: controlli Windows completati; beta accettata dall'utente il 15 ottobre
2026 con accettazione esplicita delle prove manuali residue. Merge non eseguito.

## Esiti locali Windows e accettazione

Commit applicativo provato: `f0aff670ae4a0553a58e72d3d87fc1e9e84900d2`.
Ambiente: Windows build 26200.9711 (25H2), target x86_64-pc-windows-msvc,
Rust 1.99.0, Cargo 1.99.0. Runner Python 3.14.5 della `.venv` esistente.
I successivi aggiornamenti documentali non cambiano l'app verificata.

| Controllo locale | Esito |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo check --workspace --all-targets --all-features --locked` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --all-features --locked` | PASS: 47 test (10 core, 31 desktop, 6 IO), zero fallimenti o ignorati |
| `cargo build --workspace --release --locked` | PASS: eseguibile senza ui-preview |
| Fixture Python rigenerate e confrontate con Git | PASS: identiche, 40/2/10/10 combinazioni, 16/2/4/4 soluzioni Pareto |
| `scripts/preview-gui.ps1` | PASS: 27 catture native rigenerate; 26 ispezionate visivamente, vista many non ispezionata |
| Avvio release e Calcola tramite mouse | PASS: 40 combinazioni, 16 soluzioni Pareto |
| Selezione sul testo, doppio clic sul formato, Esc | PASS: soluzione 04, dettagli A—34 con quote e sezione; Esc chiude |
| Salva e sovrascrittura diretta con Ctrl+S | PASS, file QA `.stic` |
| Nuovo studio e riapertura tramite selettore Windows | PASS: ricalcolo, filtro Una sola testa, 12 righe visibili e selezione 04 ripristinati |
| Studio vuoto non salvabile | PASS: errore specifico, nessun file prodotto |
| Esportazione riepilogo e dettaglio tramite selettori Windows | PASS: 12 righe/11 colonne e 3 righe/28 colonne, BOM UTF-8, posizione 4 e passo 180 mm coerenti |
| Popup Numero di risultati | Apertura verificata, valore 20; modifica/Applica non completati |

I test automatici sono stati rieseguiti fuori dalla sandbox: il primo tentativo
falliva in cinque test desktop di salvataggio con PermissionDenied; gli stessi
casi passano senza la restrizione. Cargo segnala problemi di accesso nella
pulizia di vecchie cache incrementali e fallback da hard link a copia, senza
errori di codice o variazioni di toolchain/lockfile. Nessun dato utente modificato.

L'app release è stata avviata con LOCALAPPDATA isolato in
`work/beta-qa-20261015/profile`, inizialmente vuoto. Log e file di prova restano
in `work/beta-qa-20261015` (non versionata); catture in `work/ui-refinements`.
SHA256 dell'eseguibile verificato:
`425494AA2D2B3B2CC00F1B94F99CEE65370F8DF783C120136CF1E53A02F235B3`.

Residui manuali NON ESEGUITI: modifica input e aggiunta/rimozione tramite UI,
cambio formato nel popup e Apri dettaglio, Applica/Annulla/Esc/X dei cinque
editor, Salva con nome e annullamenti dei selettori, apertura file invalido,
ordinamento, tutti i rami di chiusura/cambio studio, preferenze salvate e
riavvio, interazione a dimensioni ridotte/zoom, più monitor/DPI e annullamento
di una ricerca rappresentativa. Anteprime e test sintetici coprono parte di
questi comportamenti, ma non sono registrati come prove manuali.
La finestra iniziale può superare leggermente l'area disponibile sul desktop
provato; massimizzando Calcola diventa interamente visibile.

Le prove interattive sono state interrotte per il cambio Wi-Fi dell'utente.
Alla ripresa lo strumento segnalava ancora l'arresto tramite Esc.
L'utente ha quindi disposto: «diamo per buoni i collaudi e continuiamo.
se dovessero saltare fuori bug ci penseremo».
Questa è accettazione esplicita della beta e dei residui, non esecuzione delle
prove mancanti. Le caselle della checklist sotto restano un promemoria per
eventuali ripetizioni, non un blocco imposto contro la decisione dell'utente.
Gli eventuali bug successivi saranno trattati separatamente.

## Versione e perimetro

Baseline applicativa e workflow: `1b4848e5191772c3518f42f9b38dc5363aa321c5`.
Include le PR #12–#16 e la UI `a3d62ef`. Il branch di collaudo aggiunge
solo questa checklist e i riferimenti documentali: nessuna modifica all'app.
Annotare anche lo SHA del checkout effettivamente provato.
Il riferimento `main` esaminato è `3214abe` (Python).

La richiesta dell'utente autorizza a preparare il passaggio a main. L'esito
aggiornato e l'accettazione sono riportati sopra; conservare la distinzione
fra controlli automatici, prove native e residui accettati.

## Esiti sul cloud

Linux x86_64, Rust `1.99.0 (b940084d7 2026-09-28)`, lockfile invariato.

| Controllo | Esito |
| --- | --- |
| `cargo fmt --all -- --check` | Superato |
| `python scripts/python-reference.py` e confronto fixture con Git | Identico; casi 40/2/10/10 combinazioni e 16/2/4/4 soluzioni |
| `cargo test -p stick-optimizer-core -p stick-optimizer-io --locked` | 16 test superati: 10 core e 6 IO; nessun test ignorato |
| `cargo clippy -p stick-optimizer-core -p stick-optimizer-io --all-targets --all-features --locked --offline -- -D warnings` | Superato |
| `cargo build -p stick-optimizer-core -p stick-optimizer-io --release --locked --offline` | Superato |
| `cargo check --workspace --all-targets --all-features --target x86_64-pc-windows-msvc --locked` | Superato, inclusi i sorgenti dei test desktop |
| `cargo clippy --workspace --all-targets --all-features --target x86_64-pc-windows-msvc --locked --offline -- -D warnings` | Superato |
| `git diff --check` | Superato |

Il cross-check Windows verifica tipi e compilazione, non collega un eseguibile
né esegue i test Windows. I 31 test desktop, la build release desktop e i flussi
nativi vanno rieseguiti su Windows. La suite storica documenta 47 test totali;
questo numero non è un nuovo esito della suite completa.
Autorevisione dei riferimenti e del diff documentale eseguita; non è una
revisione indipendente di tutti i sorgenti della migrazione.

## Preparare una copia di prova su Windows

Da PowerShell nella propria copia del repository, usare una cartella nuova
per evitare di cambiare branch sopra modifiche locali:

```powershell
git fetch origin
git worktree add ..\stick_optimizer-qa origin/codex/leo-12-rust-main-validation
Set-Location ..\stick_optimizer-qa
git rev-parse HEAD
git status --short
rustc --version
python scripts/check.py
```

Richiede Git, Python 3.10+ per il solo runner di sviluppo, Rust della toolchain
versionata e Visual Studio Build Tools C++ con Windows SDK. Non cambiare
toolchain o lockfile se compare un errore: conservare l'output e segnalarlo.
Il runner si ferma al primo controllo fallito. Annotare il riepilogo di tutti
i test, non solo l'ultimo gruppo. Se fallisce non considerare superata la suite.

L'app da collaudare è `target\release\stick-optimizer-desktop.exe`, senza
la feature ui-preview. Non usare launcher.py o la GUI Python.

### Profilo e file isolati

Per evitare di sovrascrivere le proprie preferenze, avviare con LOCALAPPDATA
temporaneo per questi due processi. Il primo avvio serve per la checklist;
il secondo verifica anche il caricamento delle preferenze salvate nel primo.

```powershell
$qaPreviousLocalAppData = $env:LOCALAPPDATA
$qaRoot = Join-Path (Get-Location) 'work\beta-qa'
New-Item -ItemType Directory -Force $qaRoot | Out-Null
try {
    $env:LOCALAPPDATA = Join-Path $qaRoot 'profile'
    New-Item -ItemType Directory -Force $env:LOCALAPPDATA | Out-Null
    Start-Process .\target\release\stick-optimizer-desktop.exe -Wait
    Start-Process .\target\release\stick-optimizer-desktop.exe -Wait
} finally {
    $env:LOCALAPPDATA = $qaPreviousLocalAppData
}
```

Usare file di prova sotto `work\beta-qa`, senza aprire o sovrascrivere studi
personali. Al primo avvio verificare che non esistano già preferenze nel profilo
di prova; se presenti usare un'altra cartella. Il secondo avvio parte dopo
la chiusura del primo: non equivale a una prova eseguita automaticamente.

## Checklist manuale

Per ogni riga registrare PASS, FAIL o NON ESEGUITO con nota. In caso di errore
annotare passi, atteso, osservato e screenshot. Le caselle restano vuote finché
non viene riportato l'esito reale.

- [ ] **Avvio ed esempio.** Aprire la release, File → Carica esempio, Calcola.
  Senza modificare i parametri incorporati: 40 combinazioni e 16 soluzioni
  Pareto; input a sinistra e risultati/dettagli a destra. Finestra responsiva.
- [ ] **Selezione e dettaglio.** Selezionare una soluzione cliccando anche sul
  testo; doppio clic su un formato apre quote e disegno nello stesso popup.
  Cambiare formato e chiudere con Esc; verificare anche Apri dettaglio.
  Tipi cassetto/testa e quantità devono riferirsi alla selezione corrente.
- [ ] **Input e tastiera.** Modificare un numero con virgola e uno con punto;
  usare Ctrl+A e Tab. Quantità non intere e numeri negativi non validi sono
  segnalati. Cambiare un input invalida i risultati; rinominare lo studio li
  conserva. Provare aggiunta/rimozione formato e conferma di rimozione stick.
- [ ] **Cinque popup parametri.** Aprire pesi, numero risultati, A/B,
  astucciatrice e dati MT. Annulla/Esc/X conservano i valori e i risultati;
  Applica con un valore invalido segnala l'errore senza applicare la bozza.
  Applica con valore valido diverso invalida i risultati. Gli schemi sono
  visibili nei popup astucciatrice e MT. Ricaricare l'esempio per il resto.
- [ ] **Salvataggio e riapertura.** Calcolare, impostare un filtro e selezionare
  una soluzione visibile. Salvare come `work\beta-qa\studio.stic`.
  Cambiare studio e riaprire: input, filtro, ordine e selezione coerenti dopo
  il ricalcolo. Annullare il selettore Apri conserva lo studio corrente.
- [ ] **Sovrascrittura e Salva con nome.** Modificare un dato valido e salvare
  sullo stesso file; riaprire e verificare la modifica. Salva con nome crea
  una seconda copia; annullare il selettore non cambia dati o percorso.
  Verificare la conferma di sovrascrittura del selettore nativo, se presente.
- [ ] **Errori e dati incompleti.** Tentare di aprire un file `.stic` di prova
  contenente `{}`: messaggio leggibile, studio corrente conservato. Uno studio
  vuoto/incompleto non è salvabile: è il comportamento documentato della beta.
- [ ] **Filtri e CSV.** Filtrare e ordinare le soluzioni; esportare sia le
  soluzioni visibili sia il dettaglio selezionato. Annullare e ripetere ciascun
  selettore. Verificare righe, selezione, caratteri italiani, numeri e unità;
  il riepilogo contiene 11 colonne, il dettaglio 28. Scegliere esplicitamente
  il delimitatore virgola nel programma che apre il CSV.
- [ ] **Chiusura e cambio studio.** Dopo una modifica non salvata, provare
  Annulla, Abbandona e Salva nei flussi di nuovo/apertura/chiusura. Annullare
  il successivo selettore Salva non deve causare perdita dello studio.
- [ ] **Preferenze isolate.** A fine primo avvio impostare numero risultati 7
  e salvare esplicitamente i parametri come predefiniti. Nel secondo avvio e
  in un nuovo studio il valore è 7; aprire uno studio salvato usa invece i
  parametri di quel file. La scrittura riguarda solo il profilo QA isolato.
- [ ] **Ridimensionamento e zoom.** Provare finestra normale e compatta,
  scorrimenti, popup e zoom al 150%; campi, azioni e quote devono restare
  raggiungibili. Annotare risoluzione e scala Windows effettive.
- [ ] **Monitor/DPI.** Se disponibili, spostare la finestra tra monitor con
  scale diverse e riaprire i popup. Se non disponibili scrivere NON ESEGUITO;
  non considerare lo zoom equivalente a questa prova.

Il caso incorporato può terminare troppo rapidamente per premere Annulla calcolo.
Verificarlo solo con uno studio rappresentativo concordato e di dimensioni
stimate: non aumentare arbitrariamente formati o combinazioni. Questo caso va
registrato come non eseguito se non riproducibile; il test automatico del worker
non sostituisce un esito manuale inventato.

Per catture aggiuntive: `.\scripts\preview-gui.ps1` genera le viste native
documentate, ma non interagisce con i selettori file e non chiude la checklist.

## Restituzione dell'esito e integrazione

Riportare su LEO-12 o nella PR:

```text
Commit provato:
Windows / Rust:
Suite completa e release: PASS / FAIL, numero test:
Avvio/input/popup:
File/sovrascrittura/annullamenti:
CSV:
Chiusura e preferenze:
Finestra/zoom:
Monitor-DPI:
Annullamento calcolo:
Problemi e passi per riprodurli:
```

Il requisito di accettazione della beta è stato accettato dall'utente con i
residui indicati sopra. Non convertirli in PASS e non ripetere le prove contro
la sua decisione. Il merge resta un'azione distinta da eseguire su richiesta.
Le macro task LEO-6–LEO-11 sono lavoro successivo: non sono requisiti nuovi
di questa beta e non vanno implementate per chiudere il collaudo.
Prima del merge verificare che head e main non siano cambiati; aggiornare
istruzioni e README al passaggio effettivo a Rust e indicare l'avvio predefinito.
Il vecchio Python resta preservato nella cronologia e nel tag di riferimento.
Le PR #12–#16 non sono state integrate o chiuse da questa attività; il pacchetto
di collaudo ne contiene i commit e documenta il successivo passaggio cumulativo.
