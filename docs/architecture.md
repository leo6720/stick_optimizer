# Architettura e modello di calcolo

## Struttura Rust e prova GUI

La beta Rust su `main` contiene un workspace Cargo con tre componenti:

```text
desktop -> io -> core
desktop ------> core
```

`core` contiene dominio e calcolo, `io` contiene i formati e `desktop` interfaccia e
gestione dei lavori. Il passo 3 aggiunge una GUI egui/eframe in `desktop`:
`draft.rs` mantiene bozze editabili e controlli sintattici (non fattibilità
meccanica), `app.rs` gestisce la sidebar e l'area di output simultanee.
La disposizione riprende `gui/app.py`: Dati MT e input gerarchici a sinistra;
Migliori Soluzioni e Dettagli Soluzione Selezionata a destra; parametri in
cinque finestre separate da Modifica (pesi, numero risultati, obiettivo A/B,
astucciatrice e dati aggiuntivi MT). `app/parameters.rs` presenta solo i campi
del gruppo scelto e applica quel gruppo ai parametri correnti dopo la validazione
del core, preservando gli altri campi. Le immagini originali MT e dimensioni stick sono incorporate
nell'eseguibile e decodificate una sola volta all'avvio. Colori e gerarchia
visiva seguono `ui-design.md`; non ci sono pagine separate per input/output.
Gli ID di stick e formati sono indipendenti dai nomi modificabili. La rimozione
di uno stick elimina i formati associati dopo conferma; la modifica degli input
di calcolo invalida i risultati mostrati; rinominare lo studio li conserva.
Le tabelle rendono soltanto le righe visibili. Il dialogo dei parametri modifica
una bozza: validazione e invalidazione avvengono dopo Applica. Annulla conserva
lo studio. `app/details.rs` presenta i campi del candidato e la geometria nello
stesso popup, aperto con doppio clic o pulsante, senza ricalcolare valori fisici.

Il passo 4 aggiunge il percorso `draft.to_study -> core.validate -> optimize`.
La ricerca genera candidati, interseca passi comuni, enumera combinazioni e
mantiene una frontiera Pareto esatta, conservando le metriche equivalenti.
I risultati vengono ordinati prima del taglio al numero richiesto. Le formule
e i pesi del riferimento Python descritti sotto sono conservati nel core Rust;
precisione e differenze intenzionali sono descritte in `rust-engine.md`.

La GUI avvia un solo worker su uno snapshot posseduto degli input. Progresso e
annullamento usano atomiche; il thread UI controlla il completamento senza
attendere il worker. Ogni modifica incrementa la revisione, cancella i
risultati e annulla il lavoro; esiti di revisioni vecchie vengono scartati.
Chiusura e cambio studio annullano il worker. I tre dettagli mostrano
candidati, tipi cassetto e tipi testa prodotti dal core, senza ricalcolare formule.
La fase 5 implementa persistenza e CSV in IO con DTO serializzabili;
il core resta senza dipendenze da serde o formati di file. Gli studi `.stic`
conservano input e stato della vista, rigenerando in background eventuali
risultati alla riapertura. La sostituzione dello studio avviene solo dopo la
validazione completa del file. Le preferenze vengono scritte esplicitamente
nella cartella locale dell'utente, mai nei dati Python. `core/geometry.rs`
produce le primitive in mm della sezione statica: la GUI le disegna senza
ricalcolare vincoli o metriche. I filtri della vista non modificano il motore.
Schema, CSV, rappresentazione del riporto e limiti in `rust-files.md`.
LEO-17 aggiunge `core/head.rs`, API pubblica per coordinate finali e
compatibilità dell’intera assegnazione ([ADR0001](adr/0001-final-head-geometry.md)).
LEO-18 integra ricerca e aggregazione con teste separate/condivise e cassetti
effettivi ([ADR0002](adr/0002-exact-final-head-search.md)); editor, quote e CSV finali in [ADR0003](adr/0003-chosen-head-details-and-csv.md).

Le sezioni seguenti descrivono le formule del riferimento Python conservato nel
tag `python-beta-reference`, ora tradotte nel motore Rust. Le note sui file e
sulla materializzazione del prodotto cartesiano riguardano ancora Python.

## Scopo e confini

L'app aiuta a scegliere una configurazione statica di trasferimento degli stick tra macchina stickpack (MT) e astucciatrice per più formati commerciali. Ogni `Format` indica un tipo di stick e quanti stick entrano in un cassetto. `GlobalSettings` descrive la macchina e i limiti geometrici. `Weights` controlla l'ordinamento delle soluzioni. Tutte le lunghezze sono in millimetri.

Il risultato è un'ipotesi geometricamente e logicamente compatibile con i vincoli codificati. Il software non simula cinematica, velocità, collisioni, deformazioni, flusso, buffer o prestazioni produttive.

## Flusso dei dati

```text
GUI o chiamante Python
  -> GlobalSettings, StickType[], Format[], Weights
  -> validation.validate_all
  -> optimizer.generate_candidates_for_format (per formato)
  -> optimizer.build_multi_format_solutions (passo comune)
  -> scoring.build_solution (metriche aggregate e score)
  -> pareto.pareto_filter
  -> ordinamento per score e taglio a number_of_results_to_show
  -> GUI / CSV / file .sop
```

`optimize()` restituisce `(solutions, candidates_by_format)`. Un elenco `solutions` vuoto può significare un formato senza candidati oppure l'assenza di un passo astucciatrice comune; `candidates_by_format` serve a distinguere i due casi.

## Candidato di un formato

- Raggruppamenti ammessi: 1, 2, 3, 4, purché `sticks_per_beat % grouping == 0`.
- Numero cassetti per passo: 1 oppure 2.
- Divisori ammessi: 0 per gruppo 1; 0/1 per 2; 0/2 per 3; 0/1 per 4. La disposizione dei compartimenti è in `defaults.COMPARTMENT_LAYOUT`.
- Passo nominale stick: `2 * stick_width_mm + 2 * fin_length_mm`.
- Semifinestra di ricerca effettiva: `ceil(max_pitch_shift_mm * 2 / (sticks_per_beat - 1))`. I passi candidati sono multipli di `pitch_step_mm` entro quella finestra attorno al passo nominale. Questo **non** equivale a usare direttamente `max_pitch_shift_mm` come tolleranza del singolo passo.
- `pocket_pitch = grouping * adjusted_input_pitch`; `cartoner_pitch = pockets_per_pitch * pocket_pitch`. Il passo astucciatrice deve essere entro il massimo e multiplo di `pitch_step_mm`.
- La larghezza cassetto somma larghezze degli stick, divisori, giochi tra stick nello stesso compartimento e due giochi parete/divisore per compartimento; il valore è arrotondato a intero con `round()`.
- Ingombro: `pockets_per_pitch * (pocket_width + 2 * pocket_wall_width_mm)`. Un candidato è scartato se supera `cartoner_pitch`.
- Strati: `ceil(sticks_per_pocket / grouping)`. Altezza pila: strati per spessore stick. Se `max_allowed_layers` non è `None`, il superamento è un vincolo rigido.
- Quota A astuccio: larghezza cassetto. Quota B: altezza pila più `carton_B_extra_mm`. Il rapporto A/B contribuisce alla penalità.
- `pocket_type = (pocket_width, pocket_length, dividers, pockets_per_pitch)`; `robot_head_type = (grouping, adjusted_input_pitch)`. La larghezza testa calcolata è `adjusted_input_pitch * sticks_per_beat`.

## Penalità e selezione

Le penalità in `scoring.py` sono:

- Strati sopra la soglia di comfort: `max(0, layers - threshold)^2`, con soglia 7 senza divisori e 9 con divisori.
- Riporto: richiesto se `sticks_per_pocket % grouping != 0`; ciclo `grouping / gcd(sticks_per_pocket, grouping)` e penalità `2 + ciclo`.
- Raggruppamento: 0, 1, 2, 2 rispettivamente per 1, 2, 3, 4.
- Stabilità: 0 fino a `effective_unsupported_width / stick_length_mm <= 0.8`, poi `((ratio - 0.8) / 0.1)^2`.
- Rapporto A/B: `((carton_AB_ratio - carton_AB_target) / carton_AB_target)^2`.

Una soluzione contiene un candidato per ogni formato e tutti condividono esattamente lo stesso `cartoner_pitch`. Aggrega i tipi di cassetto distinti, i tipi di testa distinti **escludendo i candidati con grouping 1**, il massimo numero di strati e la somma delle penalità. Lo score somma i conteggi e le penalità moltiplicati per i rispettivi pesi, più `cartoner_pitch_weight * cartoner_pitch`.

Il filtro Pareto confronta sette metriche da minimizzare: numero tipi cassetto, numero tipi testa, penalità strati, riporto, raggruppamento, stabilità e A/B. Il passo astucciatrice non partecipa al confronto Pareto, ma entra nello score e nel criterio finale di spareggio. I pesi **non** sono una priorità lessicografica garantita: cambiandoli si possono invertire le preferenze tra metriche.

## Complessità e punti di attenzione

`build_multi_format_solutions()` raggruppa i candidati per passo comune e genera il prodotto cartesiano per ogni passo. Tutte le soluzioni così ottenute vengono mantenute prima del filtro Pareto; con molti formati il tempo e la memoria possono crescere rapidamente. `number_of_results_to_show` limita solo l'output finale, non la ricerca. Non esiste un limite preventivo delle combinazioni.

`project_io.py` salva input, risultati, filtri e selezione in JSON `.sop`. L'interfaccia salva e apre questo stato; non è un formato versionato o dotato di migrazioni. I CSV sono scritti in UTF-8 con BOM (`utf-8-sig`). Il riepilogo esporta tutte le soluzioni mostrate, il dettaglio una soluzione selezionata.

## Geometria finale integrata (LEO-18)

L’API preparatoria LEO-17 è ora usata dalla ricerca: ogni tupla di candidati
al passo comune enumera tutte le partizioni delle teste compatibili per
passo/raggruppamento e verifica congiuntamente gli assegnati. Coordinate,
identità soluzione-locali e geometria cassetto sono prodotte nel core;
tabelle e schematico non ricostruiscono classi per la vecchia chiave.
I tipi cassetto includono posizioni divisori. Pareto e score usano le stesse
sette metriche e formule con dimensioni effettive, gruppo1 escluso.
[ADR0002](adr/0002-exact-final-head-search.md) descrive arrotondamento fisico,
spareggi, partizioni, progresso/limiti e contratto motore versionato.


## Consumatori della configurazione scelta (LEO-19)

`core/dimensions.rs` proietta le coordinate effettive in quote e giochi fisici;
GUI e CSV leggono la stessa API, senza aritmetica fisica nei presentatori.
La tabella formati identifica la testa assegnata, quella delle teste mostra
ID, separate/condivise e interasse finale; i cassetti includono posizioni divisori.
Il popup formato conserva quote precedenti e aggiunge minimi, eccessi e giochi
per gap, distinguendo il centrale strutturale del gruppo4. I file conservano la
ricetta schema2; il rango originale ricostruisce la configurazione scelta anche
con filtri/ordinamento diversi. CSVv2 descrive esplicitamente gli assegnati e le
coordinate. Nessuna modifica a ricerca, formule di penalità o head_width.
