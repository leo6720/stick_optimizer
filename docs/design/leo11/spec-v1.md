# LEO-11 — UI funzionante in Rust, direzione A Essenziale

Stato: revisione1 accettata dall’utente il23ottobre2026; sviluppo e prosecuzione autorizzati.
Ticket: https://linear.app/leochist/issue/LEO-11
Baseline prodotto: main `44bb771d7e87a89b08cba07103316c3b333e302c`.
Riferimenti visivi: A-chiaro.png e A-scuro.png in varianti-v4 di questa directory.
Fonte: «Si perfetto» in risposta alla conferma di specifica, verifiche,
suddivisione e autorizzazione a sviluppo/prosecuzione dei figli dopo i merge
dell’utente. Nessun merge/deploy o nuova automazione autorizzati.

## Problema

La UI attuale presenta tabelle, campi e azioni con gerarchia troppo piatta.
Le prime ricolorazioni sono state respinte: occorre un vero ridisegno dei
componenti conservando soltanto l'organizzazione generale e le funzioni.

## Soluzione

Realizzare il design A Essenziale nell'app desktop Rust: input a sinistra,
risultati sopra e dettagli sotto a destra. Componenti, proporzioni, spaziature,
tipografia e raggruppamenti visivi possono essere completamente ripensati.
Palette Marchesini rossa/neutri, superfici continue e contenitori misurati;
coerenza ottenuta attraverso token e componenti condivisi in tutta l'app.
I mockup generati guidano l'aspetto: numeri, schemi o comandi duplicati nelle
immagini non sono requisiti e non sostituiscono dati e funzioni reali.

## Storie e criteri osservabili

1. Come progettista voglio distinguere input, confronto e dettaglio della
   selezione senza cambiare pagina o perdere il contesto.
2. Voglio leggere ed editare canali MT, stick e formati attraverso controlli
   coerenti, conservando aggiunta/rimozione, conferme, validazione e focus.
3. Voglio ordinare, filtrare e selezionare soluzioni reali con la nuova tabella,
   conservando rango, precisione, significato delle metriche e selezione stabile.
4. Voglio consultare tutte le informazioni del dettaglio:15 colonne formati,
   7 cassetti e8 teste, penalità e quote, anche tramite scorrimento.
5. Voglio popup parametri, filtri, formato e testa dello stesso sistema visivo,
   con Applica/Annulla/Esc/X, invalidazione e contenuti reali conservati.
6. Voglio vedere progresso, preparazione, conclusione e annullamento leggibili
   in entrambi i temi, senza cambiare blocco modale o ciclo di vita del worker.
7. Voglio scegliere Modifica → Tema scuro con spunta coerente; primo avvio
   chiaro, cambiamento immediato e preferenza ricordata sul PC al riavvio.
8. Voglio mantenere il tema fra Nuovo/Apri/Esempio, senza sporcare lo studio,
   invalidare risultati o salvare involontariamente i parametri predefiniti.
9. Voglio messaggi, conferme, stati vuoti/errori e controlli disabilitati
   riconoscibili; la sola variazione di colore non deve comunicare lo stato.
10. Voglio utilizzare mouse e tastiera, con focus visibile e stessi comandi,
    in finestra normale, compatta e con zoom, senza azioni irraggiungibili.
11. Come manutentore voglio token, varianti e stati di componenti riutilizzati
    realmente: pulsanti, campi, selettori, menu, tabelle, intestazioni, popup.
12. Voglio aprire/salvare studi, preferenze parametri ed esportare CSV come
    prima, con dati e significato invariati dopo il ridisegno.

Accettazione complessiva: tutte le storie sono implementate in Rust, integrate
e verificate; non bastano immagini, un HTML o una libreria non collegata all'app.
La consegna intermedia può preparare componenti/tema se dichiara ciò che resta
da integrare e presenta già un comportamento pubblico funzionante.

## Decisioni e confini

- Token semantici centralizzati e due mappe chiaro/scuro; niente colori isolati
  per schermata che aggirino il sistema. Stesse varianti/stati nei punti d'uso.
- Riutilizzare egui/eframe e renderer correnti; mantenere dominio e IO separati
  dalla presentazione. Nessuna formula, vincolo, ranking o geometria ricalcolata.
- Font proposto Inter con asset compatibile egui incorporato e licenza OFL
  distribuita; il WOFF2 del mockup non è già un asset Rust. Icone coerenti,
  etichette per operazioni ambigue, nessun download di risorse all'avvio.
- Preferenza UI locale versionata separata dai parametri predefiniti. Il toggle
  non usa il comando che salva i parametri dello studio corrente. Nessun cambio
  allo schema .stic, ai CSV o alla politica di salvataggio dei parametri.
- File tema mancante: chiaro. File non valido: fallback sicuro senza corrompere
  dati. Errore di scrittura: mantenere usabile la sessione e segnalare che la
  preferenza non è stata salvata. Percorso iniettabile per prove isolate.
- Il toggle rispetta i blocchi modali esistenti; non sblocca menu/input durante
  il calcolo e non aggiunge nuove scorciatoie o scelta automatica del sistema.
- Migrare anche override chiari di popup/progresso e colori dei disegni: nessun
  contenuto nero invisibile su nero o popup bianco inatteso. Quote/coordinate
  restano quelle del core; gli schemi generati nei concept non sono fixture.
- Lavoro applicato a finestre interne. File picker e decorazioni native del
  sistema non vengono sostituiti con una nuova UX per imitarne lo stile.
- Conservare unità e precisione attuali. Troncamenti solo con contenuto completo
  accessibile, allineamenti numerici corretti e selezione inequivocabile.
- Target visivi proposti: testo normale almeno4.5:1; testo grande, focus e
  elementi non testuali essenziali almeno3:1 dove applicabile. Nessun effetto
  luminoso o animazione nuova necessaria alla comprensione.

## Verifica proposta — da concordare

Confine principale: comportamento esterno della GUI egui tramite eventi reali
mouse/tastiera e output di rendering, usando i percorsi già presenti. Non
richiede una nuova API pubblica del binario o test di dettagli di implementazione.
Per la preferenza: API IO pubblica con directory temporanee e percorsi iniettati.

- Toggle/menu/riavvio/Nuovo/Apri/Esempio: tema coerente, studio e risultati
  conservati, zero scritture ai predefiniti parametri o dati utente.
- Editing, Tab, selezione, filtri/ordine, Applica/Annulla/Esc e scorciatoie;
  real worker/progresso/cancel/blocco modale conservati.
- Confronti di studi/risultati/CSV reali prima e dopo azioni solo visive;
  geometrie pubbliche invariate, test che dipendono da BLACK resi indipendenti
  dal tema senza eliminare le asserzioni di posizione/dimensione.
- Catture e ispezione della vera GUI nativa e dei componenti nei due temi:
  normale1450×900, compatta760×620, zoom150%; minimo720×560 senza comandi
  irraggiungibili tramite scorrimenti esistenti. Nessuna nuova navigazione.
- Suite/check/fmt/Clippy/build pertinenti, toolchain e Cargo.lock invariati.
  Linux desktop richiede backend X11; compilazione MSVC è solo compilazione.
- Prove native Windows/input/file picker/monitorDPI distinte e registrate;
  le vecchie rinunce relative ad altre PR non si trasferiscono a LEO-11.
  Le fixture congelate del progresso verificano aspetto, non avanzamento reale.
- Ogni ticket include controlli, anteprime native e due revisori fresh separati;
  nessun ticket finale che rinvii tutta la verifica alla fine.

## Tracciabilità

| Decisione utente | Copertura | Verifica |
| --- | --- | --- |
| Solo UI; disposizione generale conservata, asset liberamente ridisegnabili |1–6,9,10,12 | Confronto reale e regressioni delle interazioni |
| Palette Marchesini, moderno Notion/Linear; scelta prima variante A |1–6,9,11 | Tavola componenti e schermate native A chiaro/scuro |
| Modularizzare e riutilizzare il bottone e altri componenti |11 | Stesse varianti/stati effettivamente usati in più superfici |
| Toggle, preferenza globale persistente, default chiaro |7,8 | GUI+IO isolato, riavvio/studi/dati invariati |
| UI funzionante in Rust | Tutte | Implementazione integrata e verificata, non soli mockup |

## Esclusioni

Niente nuova navigazione, funzioni analitiche o editing delle soluzioni,
undo, nuovi filtri, nuove regole fisiche, simulazione, installer o icona Windows.
Non inglobare LEO-10 o altri bug/task; non cambiare dati utente per le prove.
Nessun merge/deploy o nuova automazione. La stima/suddivisione non avvia codice.

## Dimensionamento e piano proposto

Stima iniziale T400.000 token inclusi analisi, sviluppo, test, review/correzioni.
N iniziale=max(1,ceil(400000/150000))=3. Il taglio tecnico richiede4 figli per
separare tema trasversale e superfici, senza superare il target individuale.
Somma dopo split425.000: overhead25.000 per PR/revisioni e riconciliazione.
Intervallo complessivo345–530k; stime centrali, non contatore o hard cap.

| Figlio proposto | Consegna verificabile | Stima centrale (intervallo) | Bloccato da |
| --- | --- | --- | --- |
|LEO-21 Tema globale e componenti condivisi | Toggle e persistenza completi, tema leggibile ovunque, componenti usati davvero; ridisegno delle aree ancora da completare |120k (100–145k) | Nessuno |
|LEO-22 Area input e cornice A | MT, stick/formati, menu e stato ridisegnati; editing, azioni e focus reali nei due temi |95k (75–120k) |LEO-21 integrato/verificato su main |
|LEO-23 Risultati e dettagli inline A | Tabelle, selezione, riepilogo, filtri inline e penalità, con tutte le colonne e valori reali |110k (90–140k) |LEO-21 integrato/verificato su main |
|LEO-24 Popup e stati trasversali A | Editor, filtri, formato/testa, progresso, conferme e diagnostica coerenti con comportamenti originali |100k (80–125k) |LEO-21 integrato/verificato su main |

Il runner resta su un ticket alla volta;2/3/4 non hanno blocchi funzionali
reciproci inventati. La fondazione stabilizza i contratti condivisi. Ogni figlio
avrà criteri propri, sorgente della definizione, stima e verifiche. Se emergono
nuovi contratti o lavoro oltre target ristimare prima di espandere lo scope.

## Definizione accettata e autorizzazione alla prosecuzione

Approvazione esplicita dell’utente il23ottobre2026: «Si perfetto» alla domanda
«Confermi specifica, verifiche e suddivisione, autorizzando sviluppo e prosecuzione
dei figli dopo i tuoi merge?». Revisione1 di specifica e dimensionamento.
Parent LEO-11 è contenitore, non esecutivo. Solo questi figli sono autorizzati:
LEO-21 (120k), LEO-22 (95k), LEO-23 (110k), LEO-24 (100k).
LEO-21 non ha dipendenze. LEO-22/23/24 conservano dipendenza nativa solo su
LEO-21; possono essere abilitati dal runner esistente dopo merge raggiungibile
su main aggiornato e verifica pertinente dei criteri di LEO-21, senza chiedere
un nuovo consenso per figlio. Un ticket alla volta; nessun blocco umano cancellato,
nessun nuovo lavoro estraneo o revisione del contratto senza decisione.
I registri con UUID reali sono anche nel parent e in ciascun figlio Linear.
Ogni presa/ripresa usa un implementatore nuovo fork:none, branch isolato e
checkpoint, più due revisori fresh separati sullo SHA fissato.
Done solo dopo integrazione e verifica. Nessun merge/deploy o nuova automazione.
