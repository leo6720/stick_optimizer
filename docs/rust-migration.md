# Stick Optimizer: piano e specifica della beta Rust

Stato: fasi 1–5 implementate, con motore reale, studi, preferenze, CSV e sezioni
geometriche. Rifinitura delle interazioni GUI in corso sul branch
`codex/ui-refinements`, basato sulla beta della PR #14. Le PR restano da integrare.
Data: 11 ottobre 2026.

## Obiettivo e decisioni concordate

Realizzare una nuova applicazione desktop Windows interamente in Rust, con
motore di ricerca separato dall'interfaccia. La versione Python è un riferimento
funzionale, tecnico e di disposizione dell'interfaccia. La struttura interna
del codice e la compatibilità dei file possono cambiare.

- Conservare il repository `stick_optimizer` e usare un branch di migrazione.
- Nessuna compatibilità richiesta con vecchi file `.sop`, preferenze o CSV.
- Conservare sidebar input e area principale di output della GUI Python.
  Lo stile usa il sito Marchesini Group come riferimento; il formato di studio
  può essere riprogettato.
- Conservare inizialmente significato delle formule, unità e vincoli; eventuali
  correzioni devono essere motivate e verificate separatamente.
- Preservare lo stato Python nella cronologia prima della sostituzione finale.
- Sviluppo, test e build locali; nessun workflow GitHub Actions.
- Nessun obiettivo numerico di accelerazione finché non esistono benchmark.

## Perimetro della prima beta completa

| Area | Comportamento richiesto | Criterio di accettazione |
| --- | --- | --- |
| Studio | Creare uno studio vuoto o da esempio e assegnargli un nome | Modificare e distinguere più studi senza confondere le preferenze globali |
| Stick e formati | Aggiungere, modificare e rimuovere stick e formati associati | Validare nomi, riferimenti, dimensioni e quantità prima del calcolo |
| Macchina | Impostare canali MT, geometria, giochi, passi e limite di strati | Mostrare unità, messaggi specifici e valori non validi |
| Preferenze di ricerca | Impostare pesi, obiettivo A/B e numero di risultati | Documentare effetto sul ranking e differenza tra vincoli e penalità |
| Ricerca | Generare candidati, comporre soluzioni con passo comune e applicare Pareto | Ogni soluzione completa contiene un candidato per ogni formato |
| Diagnostica | Distinguere input invalidi, formato senza candidati e assenza di passo comune | Ogni caso ha un messaggio diverso e utile |
| Avanzamento | Calcolo in background con avanzamento e annullamento | La finestra rimane utilizzabile; annullamento e risultati parziali sono espliciti |
| Risultati | Tabella ordinata, filtri, selezione e dettaglio | Filtri e metriche corrispondono ai dati del motore e non ricalcolano formule |
| Condivisione componenti | Mostrare tipi di cassetto e teste condivisi tra formati | Le teste con raggruppamento 1 sono escluse dal conteggio |
| Vista geometrica | Rappresentazione schematica del cassetto e degli stick | Quote e raggruppamenti corrispondono al candidato, senza suggerire una simulazione dinamica |
| Salvataggio | Salva, Salva con nome e apertura nel nuovo formato | Round trip dello studio; errore leggibile su file invalido o versione non supportata |
| CSV | Esportare riepilogo e dettaglio della soluzione scelta | Colonne e unità documentate, verifica di valori e caratteri italiani |
| Preferenze | Ripristinare valori incorporati e salvare preferenze utente | Dati utente fuori dalla cartella del programma; prove isolate dai dati personali |
| Distribuzione | Eseguibile Windows con icona e risorse | Flusso completo verificato su un ambiente senza Python |

Interfaccia inizialmente in italiano, con unità visibili e input numerici che
gestiscono il separatore decimale italiano in modo documentato.

Non fanno parte della prima beta: importazione dei vecchi `.sop`, aggiornamento
automatico, servizi cloud, collaborazione, supporto web/mobile e simulazione
di cinematica, tempi ciclo, collisioni o fattibilità produttiva. L'eventuale
installer è successivo alla verifica dell'eseguibile portabile.

## Contratto iniziale del motore

Riferimenti: `docs/architecture.md`, `models.py`, `validation.py`,
`optimizer.py`, `scoring.py` e `pareto.py` della versione Python preservata.

1. Tutte le lunghezze sono in millimetri. Rifiutare valori non finiti,
   riferimenti invalidi e dati fuori dal dominio prima della ricerca.
2. Mantenere raggruppamenti, divisori e numero di cassetti ammessi, con i
   vincoli di divisibilità e geometria descritti nell'architettura attuale.
3. Una soluzione completa usa esattamente un candidato per ogni formato e
   un unico passo astucciatrice comune.
4. Il conteggio dei tipi di testa esclude i candidati con `grouping == 1`.
5. Pareto minimizza le sette metriche attuali. Il passo entra nello score,
   non nella dominanza. Una soluzione domina un'altra solo se non è peggiore
   in nessuna metrica ed è strettamente migliore in almeno una.
6. Definire un ordinamento deterministico anche in presenza di pareggi.
   Soluzioni diverse con metriche uguali non vanno eliminate senza una regola
   esplicita che stabilisca quale informazione conservare.
7. Definire precisione, tolleranze e arrotondamenti prima della traduzione.
   Il `round()` di Python sui mezzi segue il criterio del pari; non sostituirlo
   implicitamente con un arrotondamento Rust di semantica diversa.
8. Definire una rappresentazione canonica dei passi per raggruppamento e
   confronto, evitando che errori numerici spezzino un passo comune.
9. Il limite di risultati limita la restituzione, non autorizza a troncare
   arbitrariamente la ricerca. Separare stato completato, annullato ed
   eventualmente limitato; non presentare una ricerca parziale come completa.
10. Il software resta un supporto alla progettazione statica: una soluzione
    richiede verifica meccanica e di processo.

## Struttura e scelte tecniche proposte

Workspace Rust con tre componenti: `core` (dominio e ricerca), `io`
(studi, preferenze e CSV), `desktop` (GUI e gestione dei lavori).
Il core non dipende dalla libreria grafica. La GUI usa uno snapshot degli input
per ciascun calcolo e non associa risultati vecchi a input modificati.

GUI del prototipo: egui/eframe 0.36.2 con egui_extras per le tabelle e backend
Glow. La fase 3 prova editing, parametri, risultati simulati, tastiera,
ridimensionamento e zoom. La libreria è adottata per proseguire la migrazione;
restano da raccogliere feedback d'uso e prove su altri monitor/DPI Windows.

Formato implementato nella fase 5: JSON `.stic` con schema/versione/unità,
input, parametri, filtri e contratto del motore. Gli eventuali risultati sono
rigenerati in background alla riapertura, evitando cache importate incoerenti.
Preferenze separate nella cartella locale dell'utente; scritture tramite file
temporaneo e sostituzione. Campi e limiti in `rust-files.md`; nessuna eredità `.sop`.

## Sequenza di lavoro

| Fase | Deliverable | Condizione per passare alla successiva |
| --- | --- | --- |
| 1. Specifica | Questo documento e inventario delle funzioni | Perimetro iniziale esplicito e decisioni proposte distinguibili da quelle concordate |
| 2. Preparazione | Stato Python preservato, branch dedicato, workspace Rust minimo e istruzioni aggiornate | Build minima locale; modifiche preesistenti dell'utente preservate e identificate |
| 3. Prova GUI | Tabella input e risultati simulati | Prova desktop di editing, tastiera, scaling e scelta della libreria |
| 4. Motore | Modelli, validazione, candidati, score e ricerca esatta di riferimento | Casi deterministici, casi limite e confronto con Python dove la semantica è conservata |
| 5. Funzioni della beta | Recupero delle funzioni Python, nuovo formato, preferenze, CSV e vista geometrica | Flussi funzionali e round trip verificati; limitazioni dichiarate |
| 6. Prestazioni e rifiniture | Benchmark, ricerca ottimizzata e rifinitura grafica | Stessi risultati del riferimento su casi piccoli e misure su casi rappresentativi |
| 7. Distribuzione | Eseguibile Windows ottimizzato e documentazione utente | Prova senza Python, rapporto di verifica e passaggio intenzionale di Rust sul branch principale |

Primo traguardo utilizzabile dopo la fase 4: inserire un piccolo studio nella
GUI Rust, calcolarlo e visualizzare soluzioni e metriche. Persistenza e funzioni
avanzate arrivano dopo questo percorso completo minimo.

## Verifiche e benchmark

- Preparare casi validi piccoli, casi senza candidati e senza passo comune,
  confini geometrici, mezzi da arrotondare, riporto, raggruppamento 1,
  dominanza, equivalenza delle metriche e spareggi.
- Controllare le soluzioni contro i vincoli, non soltanto contro output Python.
- Non usare `python tests.py` come certificazione: può terminare prima delle
  asserzioni. Conservare input ed esiti dei casi di riferimento espliciti.
- Per le ottimizzazioni usare un enumeratore semplice su casi piccoli come
  oracolo; deduplicazione e pruning devono preservare il contratto.
- Misurare separatamente generazione candidati, combinazioni e Pareto, in
  build Rust release, con stessi input e hardware. Registrare candidati,
  combinazioni, tempo e memoria; stimare le combinazioni prima di prove grandi.
- La frontiera Pareto può comunque crescere molto: non promettere memoria
  costante né scalabilità illimitata. Parallelismo dopo misure e correttezza.
- Prove di GUI, file e distribuzione non sono sostituite dai soli test del core.

## Git, PR e automazione

Il branch di migrazione è `rust-rewrite`. AGENTS.md e il prompt
dell'automazione sono stati adattati alle verifiche Rust e alle PR verso quel
branch nella fase 2. L'automazione mantiene la cadenza di 3 ore e rispetta
dipendenze e criteri di accettazione delle issue pronte.

Preparare issue piccole con dipendenze e criteri di accettazione; una PR per
attività coerente verso il branch di migrazione. L'automazione non decide da
sola nuove formule fisiche e non esegue merge o deploy. Il branch principale
diventa Rust solo dopo la verifica della beta; evitare una PR unica per tutta
la riscrittura.

## Stato operativo della fase 1

La fase 1 ha creato soltanto questo documento, senza modificare codice o dati
utente. Le operazioni successive appartengono alla fase 2.

## Preparazione della fase 2

- Branch di migrazione: `rust-rewrite`.
- Riferimento Python: tag `python-beta-reference`, commit `0eef599`.
- Il riferimento include le modifiche locali a README e .gitignore e i file
  AGENTS.md, requirements.txt e docs presenti prima dello scaffold Rust.
- `main` resta al commit `3214abe` durante la preparazione.
- Workspace creato con `crates/core`, `crates/io` e `crates/desktop`, senza
  dipendenze esterne. Le librerie sono soltanto predisposte; il binario stampa
  un messaggio di stato e termina.
- Istruzioni e automazione aggiornate per Rust e PR verso `rust-rewrite`, con
  la cadenza precedente di 3 ore.
- Nessuna issue creata e nessuna fase successiva iniziata automaticamente.
- Rust 1.99.0 e Visual Studio Build Tools 2022 con strumenti C++ e Windows SDK
  installati sul PC. Toolchain fissata in `rust-toolchain.toml`.
- Build, controlli statici e avvio dell'eseguibile iniziale verificati.

### Verifica locale della fase 2 (11 ottobre 2026)

Eseguiti dalla radice del repository, con Rust 1.99.0 e target Windows x64 MSVC:

| Comando | Esito |
| --- | --- |
| `cargo generate-lockfile --offline` | Generato Cargo.lock, sole dipendenze locali |
| `cargo fmt --all -- --check` | Superato |
| `cargo check --workspace --all-targets --locked` | Superato per i tre componenti |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Superato senza warning |
| `cargo test --workspace --locked` | Completato; zero test perché il motore è ancora assente |
| `cargo build --workspace --release --locked` | Esito positivo, eseguibile Windows generato |
| `cargo run -p stick-optimizer-desktop --release --locked` | Messaggio di struttura iniziale, uscita con codice 0 |
| `git diff --check` | Nessun errore di whitespace |

Le verifiche certificano solo lo scaffold e il collegamento dell'eseguibile.
Non è stata verificata una GUI (non ancora presente), né un motore di calcolo.
Il minimo Rust 1.85 dichiarato nei manifest non è stato provato separatamente.
Il file utente `user_defaults.json` e i sorgenti Python non sono stati modificati
durante la preparazione Rust. Il prossimo passo è la prova GUI della fase 3.

## Prototipo della fase 3

- Branch di lavoro: `codex/phase-3-gui`, base PR `rust-rewrite`.
- Finestra Windows nativa egui/eframe 0.36.2 con sidebar input a sinistra e
  risultati/dettagli a destra, come la versione Python. Parametri dal menu
  Modifica; studio vuoto o esempio dal menu File. Lo stile riprende il sito
  Marchesini Group; riferimenti e colori in `ui-design.md`.
- Tabelle editabili, associazioni tramite ID stabili, aggiunta e rimozione.
  La rimozione di stick e il cambio di studio richiedono conferma modale.
- Input numerici con virgola o punto, interi positivi per conteggi e quantità,
  rifiuto di valori non finiti, nomi duplicati e riferimenti mancanti.
- Tre risultati fissi, filtro sul numero di tipi di testa, ordinamento per
  passo/punteggio e dettaglio selezionato. Ogni modifica agli input li cancella.
- Scorrimento orizzontale nelle tabelle e verticale nel contenuto; soltanto
  le righe visibili vengono renderizzate. Colonne adattabili anche dopo lo zoom
  e la prima inizializzazione nativa della finestra.
- Toolchain sempre Rust 1.99.0; minimo manifest aggiornato a 1.95 per egui 0.36.
- Core e IO restano predisposti. Nessuna formula fisica è stata tradotta, nessun
  salvataggio o CSV implementato; nessun dato utente o sorgente Python modificato.

### Verifica locale della fase 3 (11 ottobre 2026)

| Comando | Esito |
| --- | --- |
| `cargo fmt --all -- --check` | Superato |
| `cargo check --workspace --all-targets --all-features --locked` | Superato |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | Superato senza warning |
| `cargo test --workspace --all-features --locked` | 11 test desktop superati; core e IO senza test perché ancora assenti |
| `.\scripts\preview-gui.ps1` | Sei PNG nativi acquisiti e controllati: input, parametri, risultati, 760×620, zoom 150%, 120 stick e 121 formati |
| `cargo build --workspace --release --locked` | Eseguibile Windows ottimizzato generato |
| Avvio di `target/release/stick-optimizer-desktop.exe` | Finestra nativa aperta sul desktop |
| `git diff --check` | Nessun errore di whitespace |

I test della tastiera inviano eventi egui sintetici e verificano sostituzione
del testo con Ctrl+A, inserimento del decimale e passaggio alla cella seguente
con Tab. La prova di virtualizzazione usa 1000 stick e 1001 formati. Un test
di regressione verifica che le colonne numeriche restino visibili dopo il
passaggio dalle dimensioni provvisorie di avvio a quelle effettive con zoom.
La regressione di disposizione verifica input a sinistra, risultati a destra
e dettagli sotto anche nella finestra compatta; la gerarchia mantiene i formati
sotto il loro stick e conserva i dati quando si comprimono le righe.
Le catture usano l'API pubblica del viewport dopo che il layout si è stabilizzato;
la funzione `ui-preview` e la scrittura PNG sono escluse dalla build normale.

La resa nativa è stata verificata sul PC corrente. Non è stata eseguita una
prova manuale completa con tastiera/mouse fisici, né il passaggio tra monitor
con DPI differenti. Il prototipo serve a raccogliere questo feedback prima
di stabilizzare la GUI; non certifica correttezza meccanica o prestazioni.
Questo era lo stato al completamento della fase 3.

## Fase 4: motore e primo flusso reale

- Branch `codex/phase-4-engine`, dipendente dalla GUI della PR #12, base PR
  `rust-rewrite`. Nessun merge delle PR è stato eseguito automaticamente.
- Modelli, validazione, generazione candidati, aggregazione, score, Pareto e
  classifiche esatte; precisione e limiti in `rust-engine.md`.
- Worker GUI con progresso e annullamento, snapshot degli input e scarto di
  revisioni obsolete. Geometria e pesi modificabili da Modifica; dettagli
  reali per formato, cassetti e teste, nella disposizione Python concordata.
- Quattro casi Python stimati prima dell'esecuzione: 40, 2, 10 e 10
  combinazioni; rispettivamente 16, 2, 4 e 4 soluzioni Pareto.
- La successiva priorità concordata è completare le funzioni della UI originale
  (file, preferenze, CSV, filtri e vista geometrica), poi benchmark e rifiniture.
  Il nuovo flusso minimo non equivale ancora alla beta completa.

### Verifica locale della fase 4 (11 ottobre 2026)

| Comando | Esito |
| --- | --- |
| `python scripts/python-reference.py` | Quattro casi limitati, stima preventiva e risultati non vuoti |
| `cargo fmt --all -- --check` | Superato |
| `cargo check --workspace --all-targets --all-features --locked` | Superato |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | Superato |
| `cargo test --workspace --all-features --locked` | 24 test: 9 core, 15 desktop; IO ancora senza implementazione |
| `cargo build --workspace --release --locked` | Eseguibile ottimizzato senza funzione di cattura |
| `.\scripts\preview-gui.ps1` | Dieci viste native, inclusi risultati reali, dettagli e diagnosi |
| `git diff --check` | Nessun errore di whitespace |

Il click Calcola e la tastiera sono verificati con eventi egui sintetici; le
catture usano il viewport nativo e richiedono il completamento del worker.
Resta una prova manuale completa con dispositivi fisici e monitor multipli.
Non sono stati effettuati benchmark su studi grandi o verifiche produttive.

## Funzioni della fase 5 (11 ottobre 2026)

Ramo `codex/phase-5-beta`, basato sul motore della PR #13. La PR ha base
`rust-rewrite` e dipende dalle PR #12 e #13, ancora da integrare; non viene
eseguito alcun merge automatico.

- Apertura e salvataggio di studi JSON `.stic`, schema 1 e unità mm. Salva
  richiede input completi e validi; non è richiesta una soluzione fattibile.
- Alla riapertura, eventuali risultati vengono ricalcolati con il motore
  corrente e vengono ripristinati filtri, ordine e selezione. Le modifiche
  durante il caricamento rendono obsoleto il worker precedente.
- Preferenze esplicite in LocalAppData, CSV delle soluzioni visibili e dettaglio
  selezionato, intervalli su cinque colonne e conferma modifiche non salvate.
- Sezioni schematiche prodotte dal core e disegnate sotto i risultati, con
  scelta del formato, divisori, strati e quote. Riporto indicato senza simularlo;
  B è quota dell'astuccio, non un'altezza fisica del cassetto assunta.
- Formule, pesi incorporati, vincoli e disposizione sinistra/destra conservati.

Verifica: 40 test (10 core, 24 desktop, 6 IO); controlli fmt, check, clippy senza
warning e build release. Sedici viste native acquisite; test di round trip
richiedono risultati reali e classifica identica, verificano precisione f64,
CSV multilinea, errori e sovrascritture senza perdita del file precedente.

Restano prove manuali dei selettori file Windows, tastiera/mouse fisici,
chiusura e più monitor/DPI. Per questo la PR rimane draft. Studi vuoti o
bozze numeriche incomplete non vengono salvati. Nessun benchmark, installer,
release pubblicata, migrazione Python o modifica dei dati utente. Non si
avvia automaticamente la fase 6; occorre un perimetro autorizzato o una issue pronta.

## Rifinitura GUI richiesta dall'utente

Branch `codex/ui-refinements`, basato su `f10fd00` della PR #14, con PR su
`rust-rewrite`. Il perimetro riguarda interazioni e presentazione; non avvia
benchmark, nuove formule fisiche o distribuzione della release.

- Righe compatte e testo allineato, tooltip sui contenuti troncati,
  selezione delle righe anche cliccando sul testo.
- Doppio clic nella Panoramica formati: popup con quote complete e disegno
  insieme, pulsante equivalente, scelta formato ed Esc. Nessuna scheda grafica
  separata; invalidazione del popup dopo modifiche agli input.
- Parametri in una bozza con Applica/Annulla e validazione; rimozione esplicita
  della riga di input, menu e scorciatoie, rinomina senza perdita dei risultati.
- Confronto funzionale e differenze rispetto a Python in `ui-design.md`.

Verifica: 45 test (10 core, 29 desktop, 6 IO), inclusi eventi egui per
doppio clic, dialoghi, rimozione, selezione e scorciatoie. Il set di anteprime
produce 21 viste native, includendo menu, popup normale/compatto/zoomato,
scorrimento delle quote e riporto. La cattura del viewport verifica il layout,
non sostituisce i test fisici dei selettori Windows o di più monitor/DPI.
Le limitazioni residue di salvataggio e rappresentazione geometrica sono
documentate in `rust-files.md`; schema e motore non vengono modificati.
