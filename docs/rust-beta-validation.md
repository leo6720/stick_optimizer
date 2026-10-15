# Collaudo della beta Rust prima del passaggio a main

Attività: LEO-12. Data: 15 ottobre 2026.
Stato: controlli cloud completati; collaudo Windows da eseguire.

## Versione e perimetro

Baseline applicativa e workflow: `1b4848e5191772c3518f42f9b38dc5363aa321c5`.
Include le PR #12–#16 e la UI `a3d62ef`. Il branch di collaudo aggiunge
solo questa checklist e i riferimenti documentali: nessuna modifica all'app.
Annotare anche lo SHA del checkout effettivamente provato.
Il riferimento `main` esaminato è `3214abe` (Python).

La richiesta dell'utente autorizza a preparare il passaggio a main. Le prove
manuali non sono ancora state svolte: non dichiarare chiuso il collaudo in
base alle catture storiche o al solo cargo check.

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

Il merge a main resta sospeso finché manca il collaudo richiesto. Eventuali
prove non disponibili si discutono esplicitamente; non convertirle in PASS.
Le macro task LEO-6–LEO-11 sono lavoro successivo: non sono requisiti nuovi
di questa beta e non vanno implementate per chiudere il collaudo.
Prima del merge verificare che head e main non siano cambiati; aggiornare
istruzioni e README al passaggio effettivo a Rust e indicare l'avvio predefinito.
Il vecchio Python resta preservato nella cronologia e nel tag di riferimento.
Le PR #12–#16 non sono state integrate o chiuse da questa attività; il pacchetto
di collaudo ne contiene i commit e documenta il successivo passaggio cumulativo.
