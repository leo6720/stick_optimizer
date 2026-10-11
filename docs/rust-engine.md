# Motore Rust della fase 4

Il core è indipendente dalla GUI e dai file. `models.rs` definisce gli input
posseduti, candidati e soluzioni; `validation.rs` verifica gli input;
`search.rs` esegue ricerca e filtro Pareto. La fase 5 aggiunge IO e
`geometry.rs` per le coordinate dei disegni, senza dipendenze file nel core.
Le formule e gli otto pesi incorporati sono quelli del riferimento Python.

## Precisione e differenze intenzionali

- Lunghezze e penalità sono `f64`, finiti. Il cassetto usa arrotondamento
  all'intero più vicino con pareggio al pari, come `round()` Python.
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
solo worker e controlla le atomiche di progresso. Ogni modifica annulla lo
snapshot precedente; risultati di revisioni obsolete non vengono installati.

## Verifica e limiti della fase

Le fixture sono generate da `scripts/python-reference.py` senza dati utente.
Quattro studi producono 40/2/10/10 combinazioni e 16/2/4/4 risultati Pareto.
La suite richiede risultati non vuoti, confronta metriche, geometria e ordine,
controlla vincoli, dominanza, mezzi da arrotondare, riferimenti, valori non
finiti, output N, cancellazione e limiti. La GUI verifica anche worker,
revisioni e il click Calcola tramite eventi sintetici.

Le catture native mostrano i flussi e la disposizione. Non sostituiscono una
prova manuale completa con mouse e tastiera fisici né verifiche su più monitor.
Persistenza, CSV e disegno schematico sono aggiunti nella fase 5, descritta in
`rust-files.md`. Benchmark e simulazione dinamica restano assenti. Le soluzioni
richiedono verifica meccanica e di processo.
