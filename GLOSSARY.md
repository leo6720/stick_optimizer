# Glossario

| Termine | Significato |
| --- | --- |
| MT | Macchina stickpack; il numero dei canali è un input della ricerca |
| Stick | Prodotto con lunghezza, larghezza, spessore e pinna, in mm |
| Formato | Associazione fra un tipo di stick e una quantità per cassetto |
| Candidato | Configurazione ammissibile per un singolo formato |
| Soluzione | Un candidato per ogni formato, con passo astucciatrice comune |
| Cassetto | Contenitore di trasferimento, con geometria e possibili divisori |
| Testa robot | Tipo di testa condivisibile; grouping 1 escluso dal conteggio |
| Raggruppamento | Numero di stick raggruppati dalla configurazione candidata |
| Riporto | Carryover previsto dal modello statico; non simulazione dinamica |
| Frontiera Pareto | Soluzioni non dominate nelle metriche tecniche del core |
| Score | Punteggio pesato che ordina le soluzioni; distinto dalla dominanza |
| Bozza | Input editabile, eventualmente incompleto, prima della validazione |
| Snapshot | Input validato e associato a una revisione del calcolo |
| Studio | Dati applicativi salvati in .stic; distinto da un progetto Linear |
| Progetto Linear | Contenitore delle attività di sviluppo Stick Optimizer |
| PR | Proposta di integrazione del codice; apertura distinta dal merge |

Fonti: docs/architecture.md, docs/rust-engine.md, docs/rust-files.md.
Il software supporta la progettazione statica; non certifica fattibilità
produttiva, dinamica, collisioni o tempi ciclo.
