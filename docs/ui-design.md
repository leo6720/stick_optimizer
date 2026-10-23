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
  il doppio clic su un formato apre il dettaglio completo con la sezione
  geometrica nello stesso popup, come nel riferimento Python. La grafica non
  occupa una scheda separata. Il pulsante Apri dettaglio offre la stessa azione.
- Menu File per cambiare, aprire/salvare studi ed esportare CSV; Modifica per
  i cinque popup dei parametri e i predefiniti. Filtri apre gli intervalli per
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

## Rifiniture delle interazioni

Le righe dei risultati e dei dettagli sono alte 24 punti logici, quelle degli
input 26, con intestazioni di 26. Testo centrato verticalmente, numeri a destra
e nomi a sinistra; intestazioni e valori troncati mantengono il testo completo
nel tooltip. I testi delle celle di output non intercettano clic e doppio clic
per la selezione del testo: il gesto seleziona la riga. Nel popup, invece,
il testo resta copiabile. Scorrimento e virtualizzazione restano disponibili.

I parametri vengono modificati in una bozza locale. Applica valida prima di
sostituire lo studio; Annulla, Esc e la X scartano la bozza, senza perdere
risultati. I canali MT rimangono nella sidebar. La rinomina dello studio
segnala modifiche da salvare ma non richiede di ripetere il calcolo.
Il popup del formato identifica la soluzione di origine e viene chiuso quando
si modificano i dati di calcolo, evitando dettagli obsoleti. Contenuti lunghi
scorrono separatamente dal pulsante Chiudi; la sezione adatta la propria altezza
alle proporzioni del disegno. File mostra le scorciatoie e raggruppa i CSV.

## Strato completo nel popup formato (LEO-8)

La sezione mostra un solo strato orizzontale completo in ogni cassetto, con tutti
gli stick del raggruppamento; se ci sono due cassetti li conserva entrambi.
Contorni, divisori, B astuccio e passo mantengono i valori precedenti; lo spazio
sopra lo strato resta vuoto. L’etichetta della vista dichiara la semplificazione.
Strati reali, altezza pila e riporto restano nei dettagli, anche con ultimo
strato parziale o più di 32 strati. Nessuna nuova navigazione o scelta di strato.

## Confronto funzionale con Python

| Flusso | Stato Rust |
| --- | --- |
| Stick e formati gerarchici, aggiunta/rimozione | Presente; rimozione esplicita e menu contestuale |
| Parametri MT, astucciatrice, otto pesi, numero risultati e A/B | Cinque popup separati da Modifica come in Python, con Applica/Annulla e ripristino del gruppo |
| Calcolo, Pareto, ordine e filtro una testa | Presenti, risultati reali e annullamento del worker |
| Dettagli formati, cassetti e teste | Presenti; grouping 1 escluso dal conteggio teste |
| Popup formato con quote e disegno | Ripristinato; anche riporto, stabilità e penalità |
| Studio, preferenze e CSV | Presenti con i nuovi formati; nessuna compatibilità legacy richiesta |
| Filtri risultati | Intervalli inclusivi su cinque colonne e filtro una testa; non replica la lista Python di valori distinti con checkbox |
| Dettagli del punteggio | Penalità aggregate espandibili e penalità del candidato nel popup; non replica il toggle delle colonne Python |

Restano il collaudo fisico completo su Windows, più monitor/DPI e la preparazione
di installer/release. Salva richiede input completi e validi; le bozze incomplete
non vengono serializzate. La grafica rappresenta una sezione statica, non simula
il ciclo di riporto e usa B astuccio come riferimento verticale: non introduce
un'altezza fisica del cassetto assente dal modello. Benchmark e nuovi requisiti
fisici richiedono un'attività separata; questa rifinitura non ne modifica le formule.

La tabella teste apre con doppio clic una vista frontale statica prima/dopo.
Titolo e assegnati identificano testa e soluzione; per teste condivise il formato
rappresentato è esplicito e selezionabile. Tutti i canali hanno un prelevatore
nero e uno stick rosso. Quote dei centri dal core, zoom del disegno e scorrimento
orizzontale/verticale; Chiudi resta fuori dal contenuto scorrevole. La vista
si chiude con Esc/X/Chiudi o invalidazione, senza alterare la soluzione.
