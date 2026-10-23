# LEO-11 — prime alternative visive, non approvate

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
