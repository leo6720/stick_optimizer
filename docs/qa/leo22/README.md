# LEO-22 — rapporto QA persistito

Data24ottobre2026. Baseline main `7a2c112e217c4987df7057d5f9b8ada43fcd15b6`.
Sorgente verificato e congelato: `f0e4b46b64710c33f271e2a85db1abe3da626ca2`.
Solo area input, menu/cornice/stato. Risultati e popup non ridisegnati; core,
IO, Cargo.lock e toolchain1.99 invariati. Fonte e verifica della dipendenza21
in [preflight](preflight.json): tree del merge identico al reviewed head,
96test baseline rieseguiti e46PNG precedenti verificati per PNG/RGBA/dimensioni.

## Verifiche automatiche effettive

**99testPASS:31core (2unitari+29integrazione),55desktop e13IO; doctest0.**
Baseline96PASS prima dell’implementazione. Tre nuove prove desktop osservano:

- Unità visibili e editing attraverso eventi mouse/tastiera di canali, nomi,
  Hs/As/Ss/Bs e quantità nei due temi; invalidazione e salvataggio/riapertura
  tramite IO pubblico su percorsi isolati, con valori e riferimenti reali.
- Legende dimensionali renderizzate con boundingbox dentro il clip a1450×900,
  760×620,720×560 e150%. La prima cattura ha evidenziato As tagliata: RED
  significativo riprodotto, margine corretto, GREEN. Nessuna quota core cambiata.
- Eventi wheel e click nel compatto760/720: tre azioni input raggiungibili,
  aggiunta effettiva di uno stick, rimozione soggetta a conferma e Esc conserva
  lo stick. Le regressioni precedenti coprono Tab e virtualizzazione1000stick.

Menu/shortcut/dirty, blocchi worker e modali, selezioni, parametri, file e CSV
sono verificati dalle regressioni esistenti, tutte verdi. I test sintetici non
certificano tastiera/mouse fisici o picker del sistema.

Comandi, durata, exit code e percorsi: [validation.json](validation.json).
Log completo: [test.log](logs/test.log). Tutti PASS sul sorgente congelato:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets --features eframe/x11,ui-preview --locked
cargo clippy --workspace --all-targets --features eframe/x11,ui-preview --locked -- -D warnings
cargo build -p stick-optimizer-desktop --features eframe/x11,ui-preview --locked
cargo test --workspace --features eframe/x11,ui-preview --locked
cargo build --workspace --release --features eframe/x11 --locked
cargo check --workspace --all-targets --all-features --target x86_64-pc-windows-msvc --locked
```

Il primo Clippy sul precedente head874cb80 ha segnalato `single_match` nel
solo ramo QA focus; [log originale](logs/initial-clippy.log) conservato.
Correzione equivalente `if let` coordinata prima del nuovo freezef0e4b46;
non è un warning soppresso. I log RED delle unità e di As restano separati
in [logs](logs); non sono errori presenti nel sorgente finale.

## Matrice nativa

**28catture reali egui/eframe Glow su Linux/Xvfb**, tutte exit0, acquisite dal
medesimo sorgentef0e4b46 e ispezionate singolarmente dall’implementatore.
PNG ricompressi senza perdite soltanto dopo confronto RGBA byte per byte.
[Manifest](manifest.json): percorsi relativi, dimensioni, tema/zoom, SHA sorgente,
log di avvio, hash PNG/RGBA ed esito di ispezione. Non sono mockup o HTML.

Input leggibili nei due temi; quote Hs/As/Ss/Bs senza tagli. Menu e nomi dello
studio restano raggiungibili. Le viste scorse mostrano i comandi input anche
nel minimo e al150%; scrolling reale con mouse è verificato separatamente nei
test. La cornice non sostituisce decorazioni o picker nativi. Nomi lunghi restano
editabili con scorrimento del campo e contenuto completo nel tooltip.

| Modalità | Dimensioni / zoom | Chiaro | Scuro |
| --- | --- | --- | --- |
| results | 1450x900 / 1 | [PNG](screenshots/results-1450x900-1-light.png) | [PNG](screenshots/results-1450x900-1-dark.png) |
| compact | 760x620 / 1 | [PNG](screenshots/compact-760x620-1-light.png) | [PNG](screenshots/compact-760x620-1-dark.png) |
| results | 720x560 / 1 | [PNG](screenshots/results-720x560-1-light.png) | [PNG](screenshots/results-720x560-1-dark.png) |
| scaled | 1450x900 / 1.5 | [PNG](screenshots/scaled-1450x900-1.5-light.png) | [PNG](screenshots/scaled-1450x900-1.5-dark.png) |
| inputs-scrolled | 760x620 / 1 | [PNG](screenshots/inputs-scrolled-760x620-1-light.png) | [PNG](screenshots/inputs-scrolled-760x620-1-dark.png) |
| inputs-scrolled | 720x560 / 1 | [PNG](screenshots/inputs-scrolled-720x560-1-light.png) | [PNG](screenshots/inputs-scrolled-720x560-1-dark.png) |
| inputs-scrolled | 1450x900 / 1.5 | [PNG](screenshots/inputs-scrolled-1450x900-1.5-light.png) | [PNG](screenshots/inputs-scrolled-1450x900-1.5-dark.png) |
| file-menu | 1450x900 / 1 | [PNG](screenshots/file-menu-1450x900-1-light.png) | [PNG](screenshots/file-menu-1450x900-1-dark.png) |
| edit-menu | 1450x900 / 1 | [PNG](screenshots/edit-menu-1450x900-1-light.png) | [PNG](screenshots/edit-menu-1450x900-1-dark.png) |
| inputs-focus | 1450x900 / 1 | [PNG](screenshots/inputs-focus-1450x900-1-light.png) | [PNG](screenshots/inputs-focus-1450x900-1-dark.png) |
| inputs-error | 1450x900 / 1 | [PNG](screenshots/inputs-error-1450x900-1-light.png) | [PNG](screenshots/inputs-error-1450x900-1-dark.png) |
| inputs-empty | 1450x900 / 1 | [PNG](screenshots/inputs-empty-1450x900-1-light.png) | [PNG](screenshots/inputs-empty-1450x900-1-dark.png) |
| inputs-long | 1450x900 / 1 | [PNG](screenshots/inputs-long-1450x900-1-light.png) | [PNG](screenshots/inputs-long-1450x900-1-dark.png) |
| inputs-long | 760x620 / 1 | [PNG](screenshots/inputs-long-760x620-1-light.png) | [PNG](screenshots/inputs-long-760x620-1-dark.png) |

Le modalità `inputs-*` sono fixture QA dichiarate: vuoto, errori di input,
nomi lunghi con più gruppi e selezione/focus. `inputs-scrolled` richiede allo
ScrollArea corrente di mostrare il fondo: verifica la resa, mentre il test
wheel prova il comportamento utente. Le fixture non leggono/scrivono preferenze
personali e non aggiungono comandi o pagine all’app normale. I risultati nelle
viste `results`/`compact`/`scaled` provengono dal worker reale sull’esempio.
Non è stata ripetuta la matrice dei popup21, che non sono cambiati.

## Ripetizione e limiti

Dopo la build preview sopra, con display X11 disponibile:

```sh
DISPLAY=:122 LIBGL_ALWAYS_SOFTWARE=1 \
STICK_OPTIMIZER_PREVIEW=results STICK_OPTIMIZER_PREVIEW_THEME=dark \
STICK_OPTIMIZER_PREVIEW_SIZE=1450x900 STICK_OPTIMIZER_PREVIEW_SCALE=1 \
STICK_OPTIMIZER_SCREENSHOT_TO=/tmp/leo22-results-dark.png \
target/debug/stick-optimizer-desktop
```

Le altre viste ripetono mode/size/scale/theme del manifest. La prova ha usato
Xvfb1600×1100×24; i percorsi locali delle cache non servono per leggere il QA.
I file qui presenti sono gli artefatti consegnati, non dipendenze da /tmp.

**Windows nativo NON eseguito.** Il controllo MSVC sopra è compilazione soltanto.
Restano da provare fisicamente input/Tab, menu e scorciatoie, picker di apertura/
salvataggio/annullamento, chiusura e DPI/monitor sul PC Windows per22. Il rinvio
concesso dall’utente per PR27 riguarda21 e non viene trasferito a questo ticket.
Il gate e lo stato Linear sono gestiti dalla coordinatrice; nessun merge o Done
al solo push. Nessuna nuova automazione o modifica a dati utente.

## Revisione

PR draft verso main: https://github.com/leo6720/stick_optimizer/pull/28 .

Ciclo1, due contesti fresh `fork_turns:none`, distinti dall’implementatore:
`/root/leo22_spec_review` e `/root/leo22_standards_review`. La coordinatrice ha
ricevuto esito favorevole sul sorgentef0e4b46 da entrambi, senza difetti aperti;
il revisore standard ha letto i log finali99PASS e i controlli. Conferma di questo pacchetto QA e
final-head ancora da registrare dopo la sua verifica. Nessuna autorevisione è
presentata come indipendente e nessuna approvazione GitHub è espressa per l’utente.
