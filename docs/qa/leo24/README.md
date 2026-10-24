# LEO-24 — verifiche e provenienza

Sorgente congelato `46bd2ca55f6856b9767f8165a24ac5a0e4ed56b9`, baseline main
`7f74aad53c540eeebc36897f8f084727b83f499a`. Definizione e dimensionamento rev1
approvati il23ottobre2026, spec `5a3e830ea77073de68c65031c8615856d0d935a1`.

La ripresa corregge soltanto l'altezza iniziale dei cinque editor. Il corpo
scorrevole continua a riservare il footer e l'altezza scelta dall'utente viene
ricordata alla riapertura. Nessun cambiamento a geometrie, worker, IO o schema.

## Prove mirate

Log `resume-*` distinguono i tentativi RED e GREEN. Regressione Interasse
riprodotta prima della correzione, poi PASS; nuovo test apertura dei cinque
editor nei due temi RED per l'ottavo peso nascosto, poi PASS. Sei test parametri,
quattro popup e drag/riapertura PASS prima del freeze. Il primo tentativo del
test drag generava due pressioni anziché un movimento con pulsante tenuto:
fallimento del driver conservato, corretto solo l'evento di test.

L'editing compatto richiede inizialmente focus tramite memoria egui, poi usa
eventi pubblici per Ctrl+A, testo, Tab ed Esc; non dimostra focus automatico
all'apertura. Il drag usa eventi pubblici e osserva il rettangolo finestra
esposto da egui per verificare resize e altezza ricordata.

## Controlli sul sorgente congelato

Tutti7 controlli PASS: fmt, check, Clippy, suiteworkspace **109PASS
(31core+65desktop+13IO)**, buildrelease, cross-checkMSVC e buildpreview.
Comandi, SHA, exit e log in [checks.json](checks.json). Le prove Linux abilitano
`eframe/x11,ui-preview` solo da CLI, senza cambiare Cargo.lock o manifest.
Release usa il backend X11, senza `ui-preview`. MSVC verifica compilazione
workspace/all-targets/all-features: non esegue Windows.

La ricompilazione preview sul freeze ha restituito lo stesso SHA256 binario
prima/dopo: `c5b92f31772bb31e4485f4166bf805ade683e716b1bf701abc9259b3d41db932`.
Log [freeze-preview-build](logs/freeze-preview-build.log). Le36 catture
usano esattamente questo eseguibile, che legge i cinque editor reali.

## Matrice visiva

[manifest.json](manifest.json) attribuisce ogni cattura al proprio SHA, con
exit, dimensioni, hash PNG e pixel RGBA. Catture reali egui/eframe Glow tramite
API pubblica Screenshot su Linux/Xvfb124; PNG ricompressi senza perdita e
confrontati byte per byte sui pixel RGBA originali. Nessuna immagine generata
sostituisce la GUI.

Nuova matrice **36/36**, exit0, dimensioni/hashPNG/hashRGBA verificati.
Ispezione di tutte36 tramite sei tavole per famiglia/minimo e rappresentanti
a dimensione nativa (normale/compatta/zoom/minimo/errore): nessun nuovo difetto
visivo rilevato. Footer accessibili; il corpo conserva lo scorrimento quando
il contenuto supera l’altezza.

Cinque editor × normale1450×900 / compatta760×620 /
1450×900 al150% × due temi =30; altri6 al minimo720×560/150% per
astucciatrice, pesi e validazione lunga. L'errore congelato verifica solo
aspetto. Validazione reale, annullamento e worker sono coperti dai test.

Le64 catture precedenti restano in [historical/4b4a435](historical/4b4a435),
con il manifest originale e archivio SHA256 di135file verificato. Rappresentano
solo quel sorgente, la cui suite completa falliva (62desktop PASS/1FAIL).
Non vengono attribuite al freeze nuovo. Le altre famiglie (filtri, formato,
testa, progresso, conferme e diagnostica) non hanno delta sorgente dalla
matrice storica; si conservano quei riscontri con la loro provenienza.

## Limiti

Windows nativo non disponibile: tastiera/mouse fisici, picker nativi, chiusura
OS e monitor/DPI Windows non eseguiti. Linux e cross-check MSVC non li
certificano. Nessun rinvio di altri ticket trasferito aLEO-24. Le fixture
progresso storiche verificano solo aspetto; i test conservano il worker reale.

Ciclo1 interrotto senza verdetti, contatore conservato. **Ciclo2 concluso:
PASS indipendente specifica e PASS indipendente standard**, zero difetti
prodotto aperti sullo stesso source46bd. Revisori fresh `fork_turns:none`,
separati da implementatore/coordinatrice: `/root/leo24_spec_cycle2` e
`/root/leo24_standards_cycle2`. Rapporti [specifica](reviews/spec-cycle2.md) e
[standard](reviews/standards-cycle2.md). Suggerimento non bloccante: futura
prova di ingresso focus senza request_focus; nessuna nuova funzione aggiunta.

Il PASS di review non certifica le prove Windows mancanti né accetta a nome
dell'utente. Prossimo passo della coordinatrice: PR draft/ready-for-human per
prove native o decisione esplicita dell'utente. Nessun merge/deploy/Done
o nuova automazione.
