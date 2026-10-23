# LEO-21 — rapporto QA persistito

Data23ottobre2026. Baseline `44bb771d7e87a89b08cba07103316c3b333e302c`.
Prodotto verificato e congelato: `b6c78c80b08dee35088cc93ba09aee8c9cd7aa92`.
Head sorgente successivo: `1c4615fbfb74e0ee4fc8e0a3970f53cf1de3aef0`:
solo6righe aggiunte/2rimosse nella gallery `cfg(ui-preview)`, per confinare le
celle dimostrative a24punti. Nessun helper o callsite prodotto cambiato.
Le44catture dell'app provengono da b6c78c8; le2gallery sono rigenerate su1c4615.
Il [manifest](manifest.json) conserva head per immagine, percorso relativo,
dimensioni, modalità, tema, zoom, exit code e hash dei pixel/PNG.

Tutte le46catture sono viewport **reali egui/eframe Glow** su Linux/Xvfb,
acquisite con la funzione esistente `ui-preview`, lanciate e ispezionate
singolarmente dall'implementatore. Nessun concept generato o HTML sostituisce
l'app. PNG ricompressi senza perdite: RGBA originale confrontato byte per byte
con il file persistito prima di calcolare gli hash. Le immagini tecniche JPG
originali restano incorporate con il proprio sfondo nei due temi.

## Controlli automatici

**96 test superati**:31core (2unitari+29integrazione),52desktop e13IO.
Doctest0. Baseline:89 (31core/47desktop/11IO). I primi riepiloghi chat87/94
omettevano per errore aritmetico i2unitari core; i log non sono cambiati.
[Log completo](logs/test.log), [comandi/esiti](validation.json).

Su b6c78c8 passano, con Rust1.99.0 e Cargo.lock invariati:

- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets --features eframe/x11,ui-preview --locked`
- `cargo clippy --workspace --all-targets --features eframe/x11,ui-preview --locked -- -D warnings`
- `cargo test --workspace --features eframe/x11,ui-preview --locked`
- `cargo build --workspace --release --features eframe/x11 --locked`
- `cargo check --workspace --all-targets --all-features --target x86_64-pc-windows-msvc --locked`

Dopo la sola correzione gallery su1c4615: fmt, check, Clippy, i2test
`ui::tests`, buildpreview e checkMSVC nuovamente PASS. Log `gallery-*` e
`leo21-gallery-*` nella cartella [logs](logs). La suite completa96 non è
riportata come rieseguita dopo questa modifica della sola fixture QA.

Test-first IO: API pubbliche e percorsi temporanei, schema/versione/tema/campi
invalidi, file assente senza scrittura, roundtrip, errore sostituzione e dati
precedenti conservati. Test-first GUI: menu e spunta renderizzata, cambio
immediato, riavvio, Nuovo/Apri/Esempio, documenti/parametri/risultati/view/CSV
conservati, corruzione e scrittura fallita con messaggio e sessione utilizzabile.
Worker e conferma rispettano i blocchi; regressioni worker/cancel esistenti verdi.
Test oggettivi del contrasto e del testo strong renderizzato nei due temi.
Geometria della testa verificata con tutti i12stick/prelevatori, dimensioni e
coordinate originarie per g2/g3/g4 e25/26mm, ripetuta nei due temi.
Nessuna modifica a core, schema studio, CSV, toolchain o Cargo.lock.

## Matrice visiva

In entrambe le palette: testi, intestazioni, selezione, campi, menu, quote,
strutture testa e contorni sezione risultano leggibili. Le finestre compatte e
il150% conservano le aree scorrevoli; i pulsanti dei popup restano fuori dal
contenuto scorrevole. Non si tratta del ridisegno A completo: input, risultati
e popup saranno ridisegnati nei soli ticket LEO-22/23/24.

Le modalità `progress*` sono **fixture congelate** dei quattro stati: verificano
l'aspetto, non un avanzamento reale. La correttezza del worker è verificata
separatamente dalla suite. `components` usa la medesima libreria dell'app ed è
solo una tavola QA; non introduce una nuova pagina o comandi nell'app normale.

| Modalità | Dimensioni / zoom | Chiaro | Scuro |
| --- | --- | --- | --- |
| results | 1450x900 / 1 | [PNG](screenshots/results-1450x900-1-light.png) | [PNG](screenshots/results-1450x900-1-dark.png) |
| edit-menu | 1450x900 / 1 | [PNG](screenshots/edit-menu-1450x900-1-light.png) | [PNG](screenshots/edit-menu-1450x900-1-dark.png) |
| parameters | 1450x900 / 1 | [PNG](screenshots/parameters-1450x900-1-light.png) | [PNG](screenshots/parameters-1450x900-1-dark.png) |
| filters | 1450x900 / 1 | [PNG](screenshots/filters-1450x900-1-light.png) | [PNG](screenshots/filters-1450x900-1-dark.png) |
| progress | 1450x900 / 1 | [PNG](screenshots/progress-1450x900-1-light.png) | [PNG](screenshots/progress-1450x900-1-dark.png) |
| progress-preparing | 1450x900 / 1 | [PNG](screenshots/progress-preparing-1450x900-1-light.png) | [PNG](screenshots/progress-preparing-1450x900-1-dark.png) |
| progress-finalizing | 1450x900 / 1 | [PNG](screenshots/progress-finalizing-1450x900-1-light.png) | [PNG](screenshots/progress-finalizing-1450x900-1-dark.png) |
| progress-cancelling | 1450x900 / 1 | [PNG](screenshots/progress-cancelling-1450x900-1-light.png) | [PNG](screenshots/progress-cancelling-1450x900-1-dark.png) |
| detail | 1450x900 / 1 | [PNG](screenshots/detail-1450x900-1-light.png) | [PNG](screenshots/detail-1450x900-1-dark.png) |
| detail-bottom | 1450x900 / 1 | [PNG](screenshots/detail-bottom-1450x900-1-light.png) | [PNG](screenshots/detail-bottom-1450x900-1-dark.png) |
| head-popup | 1450x900 / 1 | [PNG](screenshots/head-popup-1450x900-1-light.png) | [PNG](screenshots/head-popup-1450x900-1-dark.png) |
| components | 1450x900 / 1 | [PNG](screenshots/components-1450x900-1-light.png) | [PNG](screenshots/components-1450x900-1-dark.png) |
| compact | 760x620 / 1 | [PNG](screenshots/compact-760x620-1-light.png) | [PNG](screenshots/compact-760x620-1-dark.png) |
| scaled | 1450x900 / 1.5 | [PNG](screenshots/scaled-1450x900-1.5-light.png) | [PNG](screenshots/scaled-1450x900-1.5-dark.png) |
| results | 720x560 / 1 | [PNG](screenshots/results-720x560-1-light.png) | [PNG](screenshots/results-720x560-1-dark.png) |
| detail-compact | 760x620 / 1 | [PNG](screenshots/detail-compact-760x620-1-light.png) | [PNG](screenshots/detail-compact-760x620-1-dark.png) |
| parameters-compact | 760x620 / 1 | [PNG](screenshots/parameters-compact-760x620-1-light.png) | [PNG](screenshots/parameters-compact-760x620-1-dark.png) |
| progress-compact | 760x620 / 1 | [PNG](screenshots/progress-compact-760x620-1-light.png) | [PNG](screenshots/progress-compact-760x620-1-dark.png) |
| head-popup-compact-bottom | 760x620 / 1 | [PNG](screenshots/head-popup-compact-bottom-760x620-1-light.png) | [PNG](screenshots/head-popup-compact-bottom-760x620-1-dark.png) |
| detail-scaled | 1450x900 / 1.5 | [PNG](screenshots/detail-scaled-1450x900-1.5-light.png) | [PNG](screenshots/detail-scaled-1450x900-1.5-dark.png) |
| parameters-scaled | 1450x900 / 1.5 | [PNG](screenshots/parameters-scaled-1450x900-1.5-light.png) | [PNG](screenshots/parameters-scaled-1450x900-1.5-dark.png) |
| progress-scaled | 1450x900 / 1.5 | [PNG](screenshots/progress-scaled-1450x900-1.5-light.png) | [PNG](screenshots/progress-scaled-1450x900-1.5-dark.png) |
| head-popup-scaled-bottom | 1450x900 / 1.5 | [PNG](screenshots/head-popup-scaled-bottom-1450x900-1.5-light.png) | [PNG](screenshots/head-popup-scaled-bottom-1450x900-1.5-dark.png) |

## Ripetizione e limiti

Buildpreview: `cargo build -p stick-optimizer-desktop --features eframe/x11,ui-preview --locked`.
Esempio di acquisizione con displayX11 disponibile:

```sh
DISPLAY=:121 LIBGL_ALWAYS_SOFTWARE=1 \
STICK_OPTIMIZER_PREVIEW=results STICK_OPTIMIZER_PREVIEW_THEME=dark \
STICK_OPTIMIZER_PREVIEW_SIZE=1450x900 STICK_OPTIMIZER_PREVIEW_SCALE=1 \
STICK_OPTIMIZER_SCREENSHOT_TO=/tmp/results-dark.png \
target/debug/stick-optimizer-desktop
```

Le altre righe usano le modalità/dimensioni/zoom del manifest; Xvfb della prova
aveva schermo1600×1100×24. I percorsi dell'eseguibile/cache locali non sono
necessari per leggere queste evidenze persistite. Le anteprime non leggono o
scrivono preferenze personali; gli studi usati sono l'esempio incorporato.

**Windows nativo non eseguito.** MSVC sopra è solo controllo di compilazione.
Mancano esecuzione desktop Windows, input fisico, file picker, chiusura nativa e
più monitor/DPI per questo ticket: gate umano da gestire dalla coordinatrice;
nessuna rinuncia storica trasferita da altre PR. Linux/Xvfb e input egui sintetici
non certificano questi controlli. Nessun merge/deploy o nuova automazione.

## Revisione

Ciclo1 avviato dalla coordinatrice, revisori separati fresh `fork_turns:none`:
`/root/leo21_spec_review` e `/root/leo21_standards_review`, distinti da
`/root/leo21_implement`. Gli esiti e la conferma sull'head finale saranno
registrati nel checkpoint; questo rapporto non inventa approvazioni.
