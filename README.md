# Stick Optimizer

## Migrazione Rust in corso

La migrazione ha base sul branch `rust-rewrite` e contiene un workspace con tre
componenti: motore (`crates/core`), salvataggi (`crates/io`) e applicazione
(`crates/desktop`). Il passo 4 implementa il motore e il primo flusso reale:
stick e formati modificabili, geometria, pesi, ricerca in background,
risultati Pareto, ordinamento, filtro e dettagli. La disposizione conserva la versione Python: input
nella sidebar sinistra, risultati e dettagli nell'area principale destra.
La fase 5 aggiunge studi `.stic`, preferenze, CSV, filtri per colonna e una
vista geometrica per formato, mantenendo la stessa area di lavoro.

La versione Python di riferimento, comprese le modifiche locali presenti prima
della migrazione, è conservata nel tag `python-beta-reference`. Il branch `main`
non viene sostituito durante questa preparazione. I file Python restano
utilizzabili nel branch di migrazione.

Da una shell con Rust e gli strumenti C++ Windows disponibili:

```powershell
cargo build --workspace --locked
cargo run -p stick-optimizer-desktop --locked
```

Il secondo comando apre la finestra «Stick Optimizer — Beta Rust».
Inizia con dati di esempio: modifica gli stick e i formati nella tabella
gerarchica a sinistra, controlla i parametri da **Modifica** e premi
**Calcola** a destra. **File** permette di creare uno studio vuoto
o ricaricare l'esempio. Seleziona una riga e premi **Rimuovi**, oppure usa il
clic destro; eliminare uno stick richiede conferma per i formati associati. **Agg. Formato**
aggiunge un formato allo stick selezionato. Il punto e la virgola sono accettati come
separatori decimali; quantità e conteggi richiedono interi positivi e MT almeno
due canali. Dal menu **Modifica** sono disponibili geometria, giochi e gli otto
pesi del punteggio. Il dialogo usa **Applica** e **Annulla**: i risultati vengono
invalidati solo dopo l'applicazione di parametri validi e diversi.
**Annulla calcolo** interrompe il lavoro senza mostrare risultati
parziali. La selezione di una soluzione popola i dettagli per formato, tipi di
cassetto e tipi di testa robot. Ogni modifica agli input cancella i risultati
e scarta il calcolo precedente.

Da **File** puoi aprire e salvare studi `.stic` con input, parametri e filtri.
Se erano presenti risultati, Apri li ricalcola in background e ripristina la
selezione. Salva richiede input completi e validi. La chiusura segnala le
modifiche non salvate. **File → Esporta** scrive le soluzioni visibili o il
dettaglio selezionato in CSV. **Filtri…** permette di limitare passo, cassetti,
teste, strati e punteggio. Nella **Panoramica formati**, il doppio clic su una
riga apre quote complete e disegno nello stesso popup; **Apri dettaglio…**
apre il formato selezionato, oppure il primo. Il popup permette di cambiare
formato ed è chiudibile con Esc. La vista grafica non è una scheda separata.
Ctrl+O apre, Ctrl+S salva, Ctrl+Maiusc+S salva con nome e Ctrl+N crea un nuovo
studio, rispettando le conferme per modifiche non salvate. Rinominare lo studio
conserva i risultati già calcolati. Da **Modifica** puoi salvare i parametri predefiniti
per i nuovi studi. I vecchi `.sop` e `user_defaults.json` non sono usati dalla
versione Rust. Formati e limiti sono descritti in
[docs/rust-files.md](docs/rust-files.md).

L'esempio incorporato esplora 40 combinazioni e produce 16 soluzioni Pareto.
La ricerca esatta ha limiti operativi espliciti: oltre un milione di combinazioni
o 10.000 soluzioni in frontiera interrompe la ricerca e segnala il limite,
senza presentare un risultato incompleto. Precisione e limiti sono descritti in
[docs/rust-engine.md](docs/rust-engine.md).

Vedere [il piano](docs/rust-migration.md) e
[le istruzioni di sviluppo](docs/development.md) per fasi e verifiche,
[il riferimento UI](docs/ui-design.md) per disposizione e stile.

## Versione Python di riferimento

Applicazione desktop di supporto alla progettazione del trasferimento degli stick dalla macchina stickpack (MT) all'astucciatrice. Cerca configurazioni geometricamente ammissibili per più formati commerciali e preferisce quelle che condividono cassetti e teste robot, riducendo penalità di strati, riporto, raggruppamento, stabilità e rapporto A/B dell'astuccio.

Il programma **non** valida la fattibilità produttiva: non modella cinematica, tempi ciclo, collisioni, dinamica del prodotto o capacità effettiva della macchina. Ogni soluzione richiede verifica ingegneristica.

## Avvio rapido

Richiede Python 3.10 o superiore, Tkinter disponibile nella distribuzione Python e [Pillow](https://pillow.readthedocs.io/) per le immagini della GUI.

```powershell
python -m venv .venv
.\.venv\Scripts\python -m pip install -r requirements.txt
.\.venv\Scripts\python main.py
```

L'ingresso normale per lavorare sul codice è `main.py`. `launcher.py` può scaricare il branch remoto `main` e sostituire file locali prima di avviare la GUI; non usarlo con modifiche non salvate. Il core (`optimizer.py` e moduli collegati) può essere importato senza Pillow né display.

## Come si usa

1. Nella schermata iniziale scegliere **Nuovo Progetto** oppure aprire uno studio `.sop`.
2. Impostare numero di canali MT e tabella gerarchica dei tipi di stick e dei formati. Dal menu **Modifica** regolare parametri astucciatrice, offset massimo, pesi, obiettivo A/B e numero di risultati.
3. Premere **Calcola**. Ogni riga dei risultati rappresenta una configurazione completa con un candidato per formato e un unico passo astucciatrice comune.
4. Selezionare una soluzione per vederne i dettagli; filtrare i risultati dalle intestazioni e usare **File → Esporta** per i CSV.
5. Salvare lo studio con **File → Salva progetto** in formato `.sop`.

Il “progetto” `.sop` è uno studio dell'app, distinto dal progetto Codex associato a questa cartella. `user_defaults.json` contiene preferenze caricate all'avvio; alcune azioni della GUI possono sovrascriverlo. È un file tracciato in Git.

## Modello in breve

Un `StickType` definisce lunghezza, larghezza, spessore e lunghezza pinna dello stick. Un `Format` lega un tipo di stick a un numero di stick per cassetto. `GlobalSettings` specifica geometria, giochi e vincoli della macchina; `Weights` definisce il punteggio.

Per ogni formato, il motore prova passi stick discreti, raggruppamenti da 1 a 4, divisori ammessi e uno o due cassetti per passo. Scarta i candidati che violano divisibilità, limite di passo, ingombro e, se impostato, limite massimo di strati. Combina soltanto candidati con lo stesso passo astucciatrice. Poi applica il filtro Pareto alle metriche tecniche, calcola il punteggio pesato, ordina e mostra i primi N risultati.

Il conteggio delle teste robot esclude i candidati con raggruppamento 1. Il limite N controlla solo quanti risultati vengono restituiti: non limita il numero di combinazioni esplorate. Una ricerca con molti formati può quindi richiedere molta memoria e tempo.

Formule, definizioni dei tipi, penalità e limiti sono spiegati in [docs/architecture.md](docs/architecture.md).

## Struttura

| Percorso | Responsabilità |
| --- | --- |
| `models.py` | Dataclass di input, candidati e soluzioni |
| `defaults.py` | Costanti e dati di esempio incorporati |
| `validation.py` | Verifica degli input |
| `optimizer.py` | Generazione e combinazione dei candidati |
| `scoring.py` | Penalità, metriche aggregate e punteggio |
| `pareto.py` | Filtro delle soluzioni dominate |
| `project_io.py` | Lettura e scrittura dello stato `.sop` |
| `export.py` | Esportazione CSV |
| `gui/` | Interfaccia Tkinter, input e risultati |
| `main.py` | Avvio normale della GUI |
| `launcher.py` | Aggiornamento da GitHub e avvio |
| `tests.py` | Report dimostrativo storico; non è una suite di regressione affidabile |

## Sviluppo con Codex e verifiche

[AGENTS.md](AGENTS.md) è il punto d'ingresso per gli agenti che lavorano nel repository: indica la mappa del codice, le regole per cambiare formule e formati, e le verifiche richieste. [docs/development.md](docs/development.md) contiene setup, flusso della GUI, contratti dei file e smoke test del core.

`python tests.py` sui dati incorporati attuali mostra che il formato `B_50` genera zero candidati con `max_allowed_layers=12`, poi termina con codice 0 senza eseguire le asserzioni finali. Non usarlo come unico indicatore di successo. Per una prova che richiede una soluzione, seguire il comando in `docs/development.md`.

## Licenza

Il repository non contiene attualmente un file di licenza. Per riuso o ridistribuzione occorre chiarire i diritti con il titolare del progetto.
