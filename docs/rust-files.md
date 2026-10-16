# File e funzioni della beta Rust (fase 5)

Gli studi Rust usano l'estensione `.stic` e contengono JSON UTF-8. Non sono i
vecchi `.sop` Python. Non si migrano automaticamente file o preferenze legacy.

## Studio, schema 1

- `schema`: `stick-optimizer-study`; `version`: `1`; `units`: `mm`.
- `name`, `sticks`, `formats`, `settings`, `weights`: nome e input completi.
  Gli ID sono univoci nello studio, i formati riferiscono gli stick tramite ID.
- `engine`: `rust-geometry-v1`, contratto di calcolo che ha creato lo studio.
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
`%LOCALAPPDATA%/StickOptimizer/preferences-v1.json`: schema interno versione 1
con geometria, canali, limiti, numero risultati e pesi. Le preferenze sono
validate indipendentemente dalle bozze stick/formati. Si applicano al prossimo
avvio e agli studi nuovi; Apri usa sempre i parametri dello studio.
**Ripristina parametri incorporati** modifica lo studio corrente; per conservarli
anche come nuovi predefiniti occorre salvarli esplicitamente. L'esempio caricato
dal menu File resta quello incorporato. `user_defaults.json` non viene usato.

## CSV

**File → Esporta soluzioni visibili** esporta le righe dopo i filtri, nell'ordine
visibile, conservando la posizione originaria della classifica. Il CSV ha 11
colonne: posizione, score, passo, conteggi cassetti/teste, strati e cinque penalità.
**Esporta dettaglio selezionato** esporta 28 colonne per candidato: nomi, passi,
raggruppamento/divisori, cassetti per passo, dimensioni/ingombri, pila, A/B,
riporto/ciclo, stabilità, depositi, larghezza testa e cinque penalità.

UTF-8 con BOM, delimitatore virgola, decimali con punto, terminatori CRLF;
virgole, doppi apici e ritorni a capo nei nomi vengono quotati correttamente.
I numeri conservano precisione completa. Le lunghezze hanno suffisso `_mm`.
Il conteggio teste esclude grouping 1; i candidati di gruppo 1 restano nel
dettaglio come nel dominio. Non si presume la vecchia struttura CSV Python.

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
compartimenti, divisori e strati. Il contorno verticale usa B astuccio come
riferimento, non un'altezza fisica del cassetto non definita dal modello.
L'ultimo strato è parziale quando la quantità non è divisibile per il gruppo;
il ciclo di riporto è indicato, non simulato. Si disegnano al massimo 32 strati
con avviso, mantenendo quote e risultati completi. Arrotondamento della
larghezza cassetto e formule del motore restano invariati.

## Verifiche residue

Test di file e GUI verificano round trip con risultati reali, precisione,
filtri/selezione, errori, sovrascrittura atomica, preferenze isolate e CSV
multilinea. Le catture native verificano disposizione e nuovi pannelli.
Restano le prove fisiche dei selettori file Windows (apertura, annullamento,
sovrascrittura), tastiera/mouse, chiusura della finestra e più monitor/DPI.
Non sono ancora stati svolti benchmark o preparati installer e release.
