# Riferimento dell'interfaccia Rust

La versione Python è il riferimento per la disposizione dell'app. La migrazione
di linguaggio non autorizza a sostituire il flusso con pagine separate per input
e output. Questa indicazione dell'utente prevale sulla precedente formulazione
generica che consentiva una riprogettazione dell'interfaccia.

## Disposizione

- Sidebar sinistra ridimensionabile: Dati MT, canali, schemi originali e unica
  tabella gerarchica. Gli stick sono righe padre; i formati sono figli con quantità.
- Area principale destra: pulsante di calcolo in alto, Migliori Soluzioni,
  riepilogo della soluzione selezionata e dettagli sotto la tabella risultati.
- I dettagli mantengono Panoramica formati, Tipi cassetti usati e Tipi teste
  robot usate. Il passo 4 popola le tabelle con metriche reali del core;
  la fase 5 aggiunge Vista geometrica, sempre sotto la classifica.
- Menu File per cambiare, aprire/salvare studi ed esportare CSV; Modifica per
  la finestra dei parametri e i predefiniti. Filtri apre gli intervalli per
  le cinque colonne senza sostituire input o output.
  L'apertura dei parametri e la visualizzazione dei risultati conservano il
  contesto della sidebar. Gli input non diventano una scheda alternativa.
- Scorrimento dei contenuti e delle tabelle, senza trasformare la finestra
  compatta in una diversa struttura di navigazione.

Riferimenti nel codice Python: `gui/app.py` (`_build_layout`,
`_build_output_tables`), `gui/forms.py`, `gui/tables.py` e `gui/results.py`.
Gli schemi `img/dati_mt.jpg` e `img/stick_dim.jpg` sono incorporati nella GUI
Rust. Non dipendono da cartelle accanto all'eseguibile.

## Stile

Riferimento iniziale osservato l'11 ottobre 2026:
[sito ufficiale Marchesini Group](https://www.marchesini.com/it).

La homepage usa bianco, rosso e grigi, titoli sans serif e azioni essenziali.
Dagli stili effettivi del sito: rosso `#B51F20`, testo `#505555`, HKGrotesk
per i titoli e Work Sans per il testo. Il prototipo riprende palette e gerarchia
con font sans serif incorporati in egui, senza distribuire quei font del sito.

Nell'app i titoli sono compatti e i dati densi: il riferimento web orienta
colori e tipografia, non impone spaziature da homepage a uno strumento tecnico.
Sfondo bianco nell'output, grigio chiaro nella sidebar, rosso per titoli,
selezioni e azione principale; bordi sottili e arrotondamenti contenuti.

Le ulteriori modifiche di disposizione richiedono un'indicazione esplicita
dell'utente; non sono una conseguenza automatica del cambio di tecnologia.
