# File e funzioni della beta Rust (fase 5)

Gli studi Rust usano l'estensione `.stic` e contengono JSON UTF-8. Non sono i
vecchi `.sop` Python. Non si migrano automaticamente file o preferenze legacy.

## Studio, schema 2

- `schema`: `stick-optimizer-study`; `version`: `2`; `units`: `mm`.
- `name`, `sticks`, `formats`, `settings`, `weights`: nome e input completi.
  Gli ID sono univoci nello studio, i formati riferiscono gli stick tramite ID.
- `engine`: `rust-final-heads-v2`, contratto di calcolo che ha creato lo studio.
- `recalculate`: vero quando lo studio salvato aveva risultati reali.
- `view`: cinque intervalli inclusivi (passo, tipi cassetto, tipi testa,
  strati massimi, punteggio), filtro una testa, ordinamento e posizione selezionata.

Il file conserva gli input e la ricetta dei risultati, **non una cache di
candidati o punteggi**. Se `recalculate` è vero, Apri avvia la ricerca corrente
in background e ripristina filtri e selezione al suo completamento. Questo
evita risultati importati incoerenti con gli input o con un motore aggiornato.
Con lo stesso motore il round trip riproduce la classifica; con un motore
diverso la GUI segnala il cambio e usa il nuovo calcolo. Un errore o limite
della ricerca viene mostrato senza produrre una classifica parziale.

I numeri JSON conservano la precisione f64; la lettura usa `float_roundtrip`.
Nessun arrotondamento di presentazione viene scritto negli input. Versioni,
unità, campi sconosciuti, riferimenti e filtri invalidi vengono rifiutati prima
di sostituire lo studio corrente. Dimensione massima per file: 8 MiB.
Gli studi vuoti o con input incompleti/non validi non sono salvabili in questa
versione: la GUI mostra i campi da correggere. La fattibilità geometrica non
è richiesta per salvare input validi. Durante il calcolo Apri/Salva sono disabilitati.

Le scritture preparano un file temporaneo nella stessa cartella, sincronizzano
il contenuto e sostituiscono il file finale tramite rename. Gli errori non
troncano il precedente file; il temporaneo viene eliminato dopo un fallimento
gestito. Non è una garanzia contro ogni guasto hardware o perdita di alimentazione.
Un errore di lettura conserva lo studio corrente e una cancellazione del
selettore di file non cambia dati o percorso. Le modifiche non salvate sono
segnalate e la chiusura/cambio studio offre salvataggio, abbandono o annullamento.

## Preferenze

**Modifica → Salva parametri come predefiniti** scrive, solo su richiesta,
`%LOCALAPPDATA%/StickOptimizer/preferences-v1.json`: schema interno versione 2
con geometria, canali, limiti, numero risultati e pesi. Le preferenze sono
validate indipendentemente dalle bozze stick/formati. Si applicano al prossimo
avvio e agli studi nuovi; Apri usa sempre i parametri dello studio.
**Ripristina parametri incorporati** modifica lo studio corrente; per conservarli
anche come nuovi predefiniti occorre salvarli esplicitamente. L'esempio caricato
dal menu File resta quello incorporato. `user_defaults.json` non viene usato.

## Preferenza UI globale (LEO-21)

Il toggle **Modifica → Tema scuro** scrive soltanto
`%LOCALAPPDATA%/StickOptimizer/ui-preferences-v1.json` su Windows; su Linux
`$XDG_CONFIG_HOME/StickOptimizer/ui-preferences-v1.json`, oppure
`$HOME/.config/StickOptimizer/ui-preferences-v1.json` se XDG non è impostata.
È un file distinto da `preferences-v1.json`, dagli studi e dai CSV:
`schema: stick-optimizer-ui-preferences`, `version: 1`, `theme: light|dark`.
L'API IO pubblica `load_ui_preferences`/`save_ui_preferences` riceve il percorso;
non applica né salva parametri, non legge gli studi. Stessi limiti di8MiB e
scrittura atomica tramite file fratello, con pulizia in caso di errore.

Lettura senza file: chiaro e nessuna scrittura; schema/versione/campi/tema
invalidi: errore IO e fallback GUI chiaro con messaggio, senza riscrivere il
sorgente. La scelta esplicita successiva può sostituire il file UI invalido.
Scrittura fallita: il tema scelto rimane nella sessione e un messaggio persistente
segnala la mancata persistenza; dati precedenti conservati. Nuovo, Apri ed Esempio
mantengono il tema, che non segue automaticamente il sistema operativo.
Il comando resta soggetto ai blocchi di ricerca e conferma esistenti.

## Migrazione Rust dello schema 1 (LEO-17)

Studi e preferenze nuovi scrivono la versione 2: `settings.max_extra_spacing`
conserva l'interasse aggiuntivo massimo, finito e non negativo, in mm.
Il valore incorporato è 5 mm; il divisore incorporato è ora 7,5 mm.
Il parametro attraversa DTO e bozze senza arrotondamenti. La ricerca lo usa
dal LEO-18; LEO-19 lo rende modificabile in Dati astucciatrice con Applica/Annulla.

La lettura di un file Rust versione 1 migra **solo in memoria**: aggiunge
`max_extra_spacing: 5.0` e aggiorna la versione interna a 2. Tutti gli altri
campi, incluso un divisore salvato a 2 mm o personalizzato, restano quelli
salvati. Aprire lo studio o avviare l'app non riscrive il file sorgente;
solo un successivo salvataggio esplicito scrive lo schema 2. Le preferenze
continuano a usare il percorso `preferences-v1.json` per ritrovare i dati
esistenti: il suffisso del percorso non indica la versione del contenuto.
La migrazione non legge dati Python né `user_defaults.json`.

Versioni diverse da 1/2, campi sconosciuti, parametri non validi, un campo
nuovo presente nello schema 1 o assente nello schema 2 sono errori espliciti.
Un errore di lettura non sostituisce lo studio corrente e non riscrive le
preferenze. Non si applicano predefiniti al posto di campi esistenti invalidi.
Durante LEO-17 il contratto rimaneva `rust-geometry-v1`; LEO-18 usa
`rust-final-heads-v2`, senza ulteriore modifica dello schema.

## CSV

**File → Esporta soluzioni visibili** esporta le righe dopo i filtri, nell'ordine
visibile, conservando la posizione originaria della classifica. Il CSVv2 conserva le prime11
colonne: posizione, score, passo, conteggi cassetti/teste, strati e cinque penalità.
**Esporta dettaglio selezionato** conserva le prime28 colonne per candidato:
nomi, passi, raggruppamento/divisori, cassetti per passo, dimensioni/ingombri,
pila, A/B, riporto/ciclo, stabilità, depositi, larghezza testa e cinque penalità.
`head_width_mm` resta passo MT × numero canali, non distanza primo–ultimo centro.

Entrambi appendono otto colonne comuni:

| Colonna | Significato |
| --- | --- |
| `csv_version`, `engine` | `2`, `rust-final-heads-v2` |
| `channels` | Numero canali MT |
| `max_extra_spacing_mm` | Limite aggiuntivo su ogni distanza mobile |
| `divider_width_mm`, `wall_width_mm` | Spessori divisore e parete |
| `wall_clearance_min_mm`, `stick_clearance_min_mm` | Giochi minimi impostati |

Il riepilogo ha20colonne: l'ultima, `configuration`, contiene JSON con `heads`
(ID, ID formati assegnati, centri al prelievo/finali per canale) e `formats`
(ID formato/stick/testa, gruppo, divisori, cassetti per passo, passo MT,
larghezza cassetto, centri locali e bordi sinistri divisori). Così alternative
con stesse metriche restano identificabili senza affidarsi ai nomi.

Il dettaglio ha56colonne; dopo le otto comuni aggiunge:

| Colonne | Significato |
| --- | --- |
| `rank` | Rango originale1-based della soluzione scelta, indipendente dall'ordine visibile |
| `format_id`, `stick_id`, `stick_width_mm` | Identità nello studio e larghezza fisica |
| `head_id`, `head_format_ids`, `head_shared` | ID testa0-based locale alla soluzione, ID assegnati ordinati, true se condivisa |
| `approach_spacing_mm`, `minimum_approach_spacing_mm`, `extra_spacing_mm` | Interasse mobile effettivo, minimo del formato, differenza |
| `head_pickup_centres_mm`, `head_final_centres_mm` | Coordinate di tutti i canali, nell'ordine canale0…N−1; origine al primo canale al prelievo |
| `pocket_stick_centres_mm`, `divider_lefts_mm` | Primo strato locale, origine alla parete interna sinistra; bordi sinistri dei divisori |
| `left_wall_clearance_mm`, `right_wall_clearance_mm` | Giochi effettivi ai due lati del cassetto |
| `gap_centre_spacings_mm`, `gap_stick_clearances_mm` | Interassi e distanze fra superfici stick adiacenti (includono il divisore se presente) |
| `gap_divider_clearances_mm` | Una coppia sinistra/destra per gap con divisore; null per gap senza divisore |
| `gap_structural` | true solo per centrale gruppo4, escluso dal limite aggiuntivo |

Le colonne plurali sono liste JSON; i gap seguono gli slot adiacenti del primo
strato (0–1,1–2,…), le coordinate hanno precisione completa. Per gruppo1:
`head_id`, `head_shared` e i tre interassi sono vuoti, le liste testa/gap sono`[]`;
nessuna testa contribuisce al conteggio. Una testa separata ha un solo assegnato,
condivisa ne ha almeno due. Teste separate con stessa geometria hanno ID diversi.
I parametri salvati nei file restano schema2: il CSV ha una versione distinta.

UTF-8 con BOM, delimitatore virgola, decimali con punto, terminatori CRLF;
virgole, doppi apici e ritorni a capo nei nomi e nelle liste JSON vengono quotati.
I numeri conservano precisione completa. Le lunghezze hanno suffisso `_mm`.
La GUI costruisce il CSV prima del selettore/scrittura; un errore non produce un
file parziale. Il conteggio teste esclude grouping1; tali candidati restano nel
dettaglio. Non si presume la vecchia struttura CSV Python.

## Filtri e disegno

I cinque intervalli sono combinati con AND e con il filtro Una sola testa.
Estremi vuoti non impongono limiti; minimo uguale a massimo seleziona un
valore esatto. I filtri agiscono sulle prime N soluzioni già restituite dal
motore: non estendono la ricerca e non cambiano Pareto o score.

Il **popup del dettaglio formato**, aperto dal doppio clic nella Panoramica
formati o da Apri dettaglio, raccoglie quote e disegno nella stessa finestra.
Permette di scegliere un formato della soluzione di origine e si chiude con Esc.
Il core produce rettangoli in mm;
la GUI li scala e disegna. La sezione rappresenta cassetti per passo,
compartimenti, divisori e un solo strato completo per ogni cassetto. Il contorno verticale usa B astuccio come
riferimento, non un'altezza fisica del cassetto non definita dal modello.
Il disegno mostra il primo strato completo anche quando l’ultimo strato reale
è parziale: uno stick affiancato per ogni posizione del raggruppamento, in
entrambi i cassetti quando sono due. Lo spazio sopra resta vuoto senza ridurre B.
I dettagli mantengono quantità, strati reali, altezza pila e riporto. Non c’è
un limite grafico di 32 strati né relativo avviso: viene rappresentato sempre
un solo strato. Il ciclo di riporto è indicato, non simulato. Arrotondamento della
larghezza cassetto e formule del motore restano invariati: LEO-18 usa
coordinate effettive e arrotondamento per eccesso per conservare i giochi fisici.

## Verifiche residue

Test di file e GUI verificano round trip con risultati reali, precisione,
filtri/selezione, errori, sovrascrittura atomica, preferenze isolate e CSV
multilinea. Le catture native verificano disposizione e nuovi pannelli.
Restano le prove fisiche dei selettori file Windows (apertura, annullamento,
sovrascrittura), tastiera/mouse, chiusura della finestra e più monitor/DPI.
Non sono ancora stati svolti benchmark o preparati installer e release.

## Cambio motore LEO-18

Schema2 invariato. Il nuovo contratto `rust-final-heads-v2` enumera teste
separate/condivise e usa dimensioni effettive. Lettura di studi con
`rust-geometry-v1` conserva il marcatore originale e non riscrive il file;
la GUI segnala il cambio e ricalcola tutti i risultati dagli input. Un nuovo
salvataggio esplicito usa il contratto corrente. Posizioni/identità vengono
ricostruite deterministicamente, senza cache ambigue della vecchia ricerca.
LEO-19 aggiunge CSVv2, editor e quote complete secondo ADR0003, conservando
schema2/contratto motore e il significato delle colonne originali.
