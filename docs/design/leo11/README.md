# LEO-11 — prime alternative visive, non approvate

## Ripartenza3 — componenti ridisegnati

L'utente ha respinto anche la revisione2: la richiesta mantiene soltanto la
disposizione generale, non gli asset o le composizioni della vecchia UI.
I componenti e i raggruppamenti visivi possono essere completamente ripensati.
La palette della prima alternativa resta il riferimento, ma le proposte1/2
non sono approvate e non devono essere trattate come base accettata.

[concept-v3.png](concept-v3.png) è una nuova esplorazione visiva generata con
le tre fotografie fornite come riferimenti di stile: organizzazione generale
input a sinistra, risultati sopra e dettagli sotto a destra. Introduce un nuovo
linguaggio di componenti: gruppi stick/formati, controlli, tavola risultati,
riepilogo e selettori dei dettagli, con nuove proporzioni e gerarchie.

È un concept grafico, non una schermata implementata o una specifica funzionale.
Numeri e geometrie sono illustrativi, non risultati verificati. Eventuali
azioni ripetute nei gruppi e testi abbreviati dell'immagine non approvano
cambi ai flussi, duplicazioni di comandi o perdita di dati: richiedono verifica
prima del mockup fedele e del contratto finale. Tema scuro da sviluppare dopo
il feedback sulla direzione. Nessun codice prodotto o ready.

Le esplorazioni precedenti rimangono sotto solo come storico dei tentativi.

## Revisione2 — misure e gerarchia, non soltanto palette

L'utente ha preferito la palette della prima alternativa, criticando le tre
proposte perché troppo simili alla UI attuale salvo i colori. Ha fornito tre
fotografie di reference: dashboard scura con contenitori delicati, tabella chiara
con righe leggibili e componenti/menu chiari con padding, raggi e ombre leggere.
Palette A confermata; composizione estetica e misure non ancora approvate.
Il vincolo di organizzazione funzionale non congela coordinate, proporzioni,
dimensioni e spaziature: questo chiarimento prevale sulla prima interpretazione.

Aprire [proposta-v2.html](proposta-v2.html), autonoma, o le catture
[chiaro](v2-chiaro.png), [scuro](v2-scuro.png) e [confronto](v2-confronto.png).
La tavola dei componenti è nello stesso HTML; screenshot in v2-componenti-*.png.

Cambiamenti concreti proposti:

- Inter Variable incorporata, con licenza SIL OFL1.1 e origine/hash in
  font-source.json; solo artefatto HTML, non font integrata nel prodotto Rust.
- Titolo principale20px, controllo34px invece di27, righe risultati35px invece
  di26, spazi interni più leggibili e intestazioni tabelle35px invece di27.
- Sidebar400px invece di430, margine principale26px invece di18;
  stessi gruppi e comandi, senza nuove funzioni o navigazione.
- Risultati in un contenitore scorrevole di212px: circa5 righe visibili invece
  di7 nel primo mockup, mantenendo dati, riepilogo e dettagli sullo schermo.
  Questo compromesso di densità resta da valutare dall'utente.
- Tabelle con contorni9–10px, griglia delicata, riepilogo su superficie tenue,
  pulsanti7px e icone lineari inline; nessuna nuova azione.

Controlli ChromiumLinux: font Inter effettivamente caricato, menu/tema
dimostrativi, quattro catture e confronto, nessun erroreJS o overflow laterale
indesiderato nel pannello principale. Alla misura1450×900 il contenuto principale
sta nei suoi820px; tabella risultati scorre210px su281px, dettaglio mantiene
lo scorrimento orizzontale. Catture chiara/scura ispezionate.
Nessuna verifica compatta/zoom/nativa egui o implementazione del tema nel prodotto.

Rigenerazione aggiuntiva: build_v2.py dopo build_mockups.py, poi render_v2.py.
Il font rimane nel file locale InterVariable.woff2 e viene incorporato nell'HTML.

## Prima esplorazione — conservata come storico

Esplorazione richiesta dall'utente durante il grill. Non è implementazione
di prodotto, specifica accettata o autorizzazione ready. Nessuna modifica ai
crate Rust, a dati utente o ai flussi applicativi.

Fonte: documento UI_UX_Design_Guidelines_Desktop_v1.docx fornito dall'utente,
conservato nel [design language di LEO-11](https://linear.app/leochist/document/leo-11-design-language-bozza-01-e-linee-guida-6be94d2538ac).
App di riferimento main `44bb771d7e87a89b08cba07103316c3b333e302c`;
lettura del codice e screenshot nativi precedenti usati per mantenere struttura,
comandi, tabella gerarchica, colonne dei risultati e dettagli.

## Aprire e confrontare

Scaricare e aprire [confronto-ui.html](confronto-ui.html) in un browser.
È autonomo: immagini incorporate, nessuna rete o dipendenza esterna.

- A — Linea: neutri freddi, gerarchia essenziale, carattere vicino a Linear.
- B — Materia: neutri caldi, superfici morbide, carattere più editoriale.
- C — Precisione: struttura netta, neutri grafite/blu, carattere più tecnico.

Ogni alternativa ha chiaro/scuro, con stessi dati e struttura. Il selettore
esterno cambia variante/tema; il menu Modifica dimostra il toggle concordato.
I comandi restanti sono rappresentazioni, non funzioni del motore. La tavola
dei componenti usa gli stessi token e classi dei mockup, aggiornandosi con tema
e variante; hover e focus sono dimostrabili. Non è ancora la libreria Rust.

Screenshot: A/B/C-chiaro.png, A/B/C-scuro.png; confronto-6-varianti.png;
componenti-A-scuro.png.

## Limiti e prossima decisione

Canvas1450×900; testo e dati illustrativi, non output certificato del motore.
Valori/candidati non devono essere usati come fixture di correttezza.
Numeri/tabulazione/font del browser non certificano la resa nativa egui;
il font è di sistema, non Inter distribuita. Le immagini tecniche originali
mantengono il loro sfondo; la resa finale nei temi resta da valutare.
Nessuna persistenza della preferenza implementata dal mockup: è soltanto
un requisito concordato per la futura implementazione. Nessun tema di sistema
o altra funzione aggiunta. Compatta/zoom, popup e audit completo di contrasto
non verificati da questa prima tavola della schermata principale.

Controlli effettuati in Chromium Linux: sei catture, tavola componenti e board;
menu/tema dimostrativi funzionanti, coordinate delle aree principali identiche
tra varianti entro tolleranza3px, nessun errore JavaScript. Catture A chiaro/scuro,
board complessiva e tavola componenti ispezionate. Nessuna suite Rust eseguita.

Prossima azione dell'utente: scegliere una direzione o indicare elementi da
combinare/correggere. Poi raffinare sistema e stati, confermare definizione,
specifica/verifiche, dimensionamento e autorizzazione degli eventuali ticket.
Nessun merge/deploy o avvio di sviluppo autorizzato dalla sola esplorazione.

## Rigenerare

`python docs/design/leo11/build_mockups.py` ricrea l'HTML con gli asset del repo.
`python docs/design/leo11/render_mockups.py` produce catture e controlli del
dimostratore; richiede Playwright Python e Chromium in `/usr/bin/chromium`.
Questi strumenti servono solo agli artefatti di design, non alla build Rust.
