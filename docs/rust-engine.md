# Motore Rust della fase 4

Il core è indipendente dalla GUI e dai file. `models.rs` definisce gli input
posseduti, candidati e soluzioni; `validation.rs` verifica gli input;
`search.rs` esegue ricerca e filtro Pareto. La fase 5 aggiunge IO e
`geometry.rs` per le coordinate dei disegni, senza dipendenze file nel core.
Le formule e gli otto pesi incorporati sono quelli del riferimento Python.

## Precisione e differenze intenzionali

- Lunghezze e penalità sono `f64`, finiti. Il cassetto usa arrotondamento
  all’intero superiore del minimo fisico effettivo (LEO-18); non può ridurre
  i giochi minimi arrotondando verso il basso.
- Passi MT arrotondati a sei decimali: le chiavi `Pitch(i64)` sono milionesimi
  di millimetro. Anche il passo astucciatrice viene canonizzato sulla stessa
  griglia; una soluzione usa esattamente la stessa chiave per tutti i formati.
  Il limite numerico mantiene i tick entro 2^53, rappresentabili esattamente in
  f64. Incrementi inferiori a 0,000001 mm vengono rifiutati esplicitamente.
  Le semantiche binarie dei mezzi decimali possono differire da `round(x, 6)`
  Python: la griglia Rust è intenzionale, non una garanzia su ogni input IEEE.
- Tolleranza geometrica 1e-9 mm e tolleranza del rapporto passo/incremento
  1e-6, come Python. Pareto usa confronti esatti su metriche finite, senza
  introdurre un epsilon che cambierebbe la dominanza.
- MT richiede almeno due canali: con uno il denominatore dell'offset Python
  era zero. Il nuovo comportamento è un errore di validazione, non una nuova
  formula per quel caso indefinito. Valori non finiti vengono rifiutati anche
  quando il validatore Python li avrebbe lasciati passare.
- I tipi testa escludono grouping 1. I tipi cassetto includono larghezza,
  lunghezza, divisori e cassetti per passo. Le metriche equivalenti restano
  distinte: la ricerca non elimina configurazioni geometriche per un pareggio.
- Classifica stabile: score, sette metriche Pareto, passo e ordine di
  enumerazione (formati nell'ordine di input, passi crescenti, gruppi/divisori/
  cassetti nell'ordine Python). Il taglio N avviene dopo la ricerca completa.

## Ricerca e limiti operativi

Il prodotto cartesiano viene stimato prima di enumerarlo. Si visita una
combinazione alla volta e si mantiene la frontiera Pareto; le combinazioni
dominate non sono materializzate tutte in memoria. I test confrontano questa
frontiera con una ricerca brute force indipendente su casi piccoli.

`SearchLimits::default()` impone un milione di combinazioni, 100.000 passi di
griglia per formato, 100.000 candidati complessivi e 10.000 elementi in
frontiera. Sono limiti computazionali della prima implementazione, non vincoli
fisici né un metodo per scegliere le migliori configurazioni. Se superati, la
ricerca restituisce un errore e nessuna classifica parziale. Non promette ancora
prestazioni su studi grandi; Pareto può richiedere tempo quadratico.

Gli esiti separano input invalidi, formati senza candidati, assenza di passo
comune, annullamento, limiti operativi ed errori numerici. La GUI conserva un
solo worker e controlla le atomiche di progresso. `estimated` è il numero esatto
di assegnazioni strutturali sui passi comuni, disponibile dopo la generazione
candidati e la stima delle partizioni; `examined` cresce dopo la verifica
completa di ciascuna partizione, inclusi scarti e aggiornamento Pareto. `finalizing`
segnala l’ordinamento e la preparazione del report. La percentuale riguarda
l’enumerazione completata, non il tempo o il completamento del report. Ogni modifica annulla lo
snapshot precedente; risultati di revisioni obsolete non vengono installati.

## Verifica e limiti della fase

Le fixture sono generate da `scripts/python-reference.py` senza dati utente.
Prima di LEO-18 quattro fixture storiche producevano40/2/10/10 combinazioni
e16/2/4/4 risultati Pareto. Queste classifiche compatte non sono il contratto
attuale; restano fixture di provenienza per i campi invariati.
La suite richiede risultati non vuoti, confronta metriche, geometria e ordine,
controlla vincoli, dominanza, mezzi da arrotondare, riferimenti, valori non
finiti, output N, cancellazione e limiti. La GUI verifica anche worker,
revisioni e il click Calcola tramite eventi sintetici.

Le catture native mostrano i flussi e la disposizione. Non sostituiscono una
prova manuale completa con mouse e tastiera fisici né verifiche su più monitor.
Persistenza, CSV e disegno schematico sono aggiunti nella fase 5, descritta in
`rust-files.md`. Benchmark e simulazione dinamica restano assenti. Le soluzioni
richiedono verifica meccanica e di processo.

## API preparatoria delle posizioni finali (LEO-17)

`final_head_geometry(&Settings, &[HeadFormat])` è una API pubblica, introdotta
separatamente in LEO-17 e ora consumata dalla ricerca LEO-18. Ogni formato specifica ID, larghezza, raggruppamento, divisori
e passo di prelievo `Pitch`. Restituisce coordinate dei centri per tutti i
canali MT, identità di canale/gruppo a partire da zero, interasse comune e ID
formati ordinati. Il primo canale al prelievo ha x=0; tutti gli altri hanno
x=canale×passo. Stesso input, anche permutato, produce la stessa geometria.
Il chiamante fornisce l'intero insieme di formati assegnati; ID duplicati,
insieme vuoto e raggruppamento 1 sono rifiutati. Le teste di gruppo 1 restano
escluse dal conteggio, come prima.

Il contratto di coordinate, divisori, precisione e assegnazioni è in
[ADR 0001](adr/0001-final-head-geometry.md). L'API verifica congiuntamente i
minimi e il limite aggiuntivo di ogni formato, il passo/raggruppamento comune,
la divisibilità dei canali e gli spazi fissi. L’API da sola non enumera assegnazioni né verifica cassetti: la ricerca
LEO-18 integra queste verifiche come descritto sotto.

L'API controlla anche gli intervalli nelle coordinate f64 effettivamente
restituite, inclusi i gruppi ripetuti. Se un passo/origine molto grande fa
perdere più di 1e-9 mm a un gap mobile o fisso, oppure viola i limiti del
formato, restituisce errore esplicito di precisione invece di una geometria
incoerente. Il solo valore astratto dell'interasse comune non basta.

## Integrazione LEO-18: contratto rust-final-heads-v2

La ricerca ora consuma l’API e sostituisce il vecchio accorpamento per sola
chiave `(gruppo,passo)`. Enumera tutte le partizioni separate/condivise entro
ogni chiave e verifica l’insieme completo assegnato. Ricostruisce cassetti,
divisori, ingombri e penalità esistenti con le posizioni effettive; gruppo4
include il passo strutturale fra le coppie. Il candidato espone `head_id` e
coordinate locali; la soluzione espone identità e geometria delle assegnazioni.
La larghezza testa resta passo MT×canali. Nessuna nuova penalità o peso.

Il limite combinazioni ora include tutte le partizioni strutturali, anche
quelle incompatibili. Stima esatta tramite numeri di Bell, prima della ricerca;
avanzamento per partizione completamente verificata, compresi gli scarti.
Durante preparazione/stima il totale è0. I pareggi mantengono l’ordine input,
poi partizioni condivise prima delle separate. Limiti e annullamento non
restituiscono classifiche parziali. Nessuna ottimizzazione approssimata.

Dimensioni intere arrotondate per eccesso per rispettare i giochi effettivi;
stabilità sulla massima luce reale dei compartimenti. Tipi cassetto includono
posizioni divisori, tipi testa includono assegnazioni esplicite. Contratto,
formule geometriche e motivazioni: [ADR0002](adr/0002-exact-final-head-search.md).
Le note preparatorie LEO-17 sopra descrivono lo stato storico precedente;
ricerca e compatibilità sono ora integrate, editor e consumatori completati in LEO-19, [ADR0003](adr/0003-chosen-head-details-and-csv.md).
