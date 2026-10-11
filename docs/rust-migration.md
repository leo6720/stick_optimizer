# Stick Optimizer: piano e specifica della beta Rust

Stato: specifica iniziale e fase 2 completate. La struttura Rust
è presente, mentre motore e GUI non sono ancora implementati.
Data: 11 ottobre 2026.

## Obiettivo e decisioni concordate

Realizzare una nuova applicazione desktop Windows interamente in Rust, con
motore di ricerca separato dall'interfaccia. La versione Python è un riferimento
funzionale e tecnico, non un vincolo di struttura o di compatibilità.

- Conservare il repository `stick_optimizer` e usare un branch di migrazione.
- Nessuna compatibilità richiesta con vecchi file `.sop`, preferenze o CSV.
- Interfaccia e formato di studio possono essere riprogettati.
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

Prima candidata per la GUI: egui/eframe. La scelta diventa definitiva dopo
una prova con tabella editabile, parametri, risultati simulati, tastiera,
ridimensionamento e scaling Windows. Non è ancora una dipendenza adottata.

Nuovo formato proposto: JSON con identificatore del formato e versione dello
schema. Input, preferenze e risultati sono distinti; eventuali risultati salvati
includono versione del motore e input del calcolo. Scrittura tramite file
temporaneo e sostituzione sicura. Estensione e campi finali saranno definiti
durante la fase salvataggi, senza obbligo di ereditare `.sop`.

## Sequenza di lavoro

| Fase | Deliverable | Condizione per passare alla successiva |
| --- | --- | --- |
| 1. Specifica | Questo documento e inventario delle funzioni | Perimetro iniziale esplicito e decisioni proposte distinguibili da quelle concordate |
| 2. Preparazione | Stato Python preservato, branch dedicato, workspace Rust minimo e istruzioni aggiornate | Build minima locale; modifiche preesistenti dell'utente preservate e identificate |
| 3. Prova GUI | Tabella input e risultati simulati | Prova desktop di editing, tastiera, scaling e scelta della libreria |
| 4. Motore | Modelli, validazione, candidati, score e ricerca esatta di riferimento | Casi deterministici, casi limite e confronto con Python dove la semantica è conservata |
| 5. Prestazioni | Benchmark e ricerca ottimizzata con annullamento | Stessi risultati del riferimento su casi piccoli e misure su casi rappresentativi |
| 6. Beta completa | GUI collegata, nuovo formato, preferenze, CSV e vista geometrica | Flussi funzionali e round trip verificati; limitazioni dichiarate |
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

Prima di iniziare l'implementazione creare il branch di migrazione e adattare
AGENTS.md e il prompt dell'automazione alle verifiche Rust e alla destinazione
delle PR. Il vecchio prompt usa ancora il workflow Python e il branch predefinito:
non è il workflow definitivo della migrazione.

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
