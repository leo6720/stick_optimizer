# Istruzioni per Codex

## Contesto

Questo repository contiene un'app desktop Tkinter per confrontare configurazioni geometriche di trasferimento degli stick dalla macchina stickpack all'astucciatrice. Il codice produce candidati per ciascun formato, combina quelli con lo stesso passo astucciatrice, applica un filtro Pareto e ordina le soluzioni con un punteggio pesato. È uno strumento di supporto alla progettazione: una soluzione calcolata richiede comunque verifica meccanica e di processo.

Prima di modificare il codice, leggere `README.md`, `docs/architecture.md` e la sezione pertinente di `docs/development.md`. Le istruzioni valgono per l'intero repository. Le istruzioni esplicite dell'utente prevalgono.

## Mappa del codice

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
