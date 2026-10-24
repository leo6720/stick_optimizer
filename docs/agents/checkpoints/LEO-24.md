# LEO-24 — popup e stati trasversali A

Implementatore nuovo `/root/leo24_implement`, `fork_turns:none`, distinto dalla
coordinatrice e dai futuri revisori. Isolamento effettivo: worktree dedicato
`/workspace/stick-leo24-async`, branch unico `codex/leo-24-async`, da main
`7f74aad53c540eeebc36897f8f084727b83f499a`. Head iniziale uguale alla baseline.
PR assente; cicli indipendenti **0**. Claim Linear e GO gestiti dalla coordinatrice;
GO condizionato soddisfatto da baseline e smoke PASS prima di edit prodotto.

Solo [LEO-24](https://linear.app/leochist/issue/LEO-24), parent LEO-11:
Definizione accettata/Dimensionamento rev1, 100k (80–125k), autorizzazione
esplicita 23 ottobre2026 «Si perfetto», sviluppo e prosecuzione dei soli21–24.
Spec congelata `5a3e830ea77073de68c65031c8615856d0d935a1`,
`docs/design/leo11/spec-v1.md`; viste entrambe A-chiaro/A-scuro varianti-v4.
Scope: cinque editor, filtri, formato/testa, conferme/diagnostica e progresso.
Nessun nuovo comportamento, core/formule/geometrie/ranking/IO-schema/CSV,
input/output22/23, parametri predefiniti, toolchain o lock modificati.

## Preflight effettivo —24 ottobre2026

Workspace Linear leochist UUID bbb36fe9-e6a2-44b5-aa0a-f46979dc5733, team LEO
e progetto verificati tramite connessione Leochist. Ticket completo/commenti
vuoti/relazioni e parent letti: Todo+ready prima del claim, nessun blocco umano
o revoca; solo blockedBy21. Rilettura dopo claim: In Progress della coordinatrice,
stessa revisione/perimetro.22/23 InReview con residui Windows non sono dipendenze24.
Nessun branch locale/remoto/worktree24 preesistente, ricerca PR24 in:title vuota;
ricerca generica restituisce solo citazioni nelle altre PR. Nessuna ownership ambigua.

PR27 API merged, base main, merge `7a2c112e217c4987df7057d5f9b8ada43fcd15b6`
raggiungibile dal main corrente. Main e headPR29 `9d9d8be7a817b3b7640f02f9b302a97be2a5f538`
condividono tree `07bf1b8e66aaccb610d5ba75cc02354aff279f22`.
Fondazione21: nessun delta IO; token/tema/persistenza e regressioni tema/menu/
blocchi/immutabilità passate nella suite corrente. Tutti46 PNG21 verificati
per hash PNG/RGBA, dimensioni ed exit0. Evidenze storiche21/23 lette con le loro
provenienze: non si attribuisce la precedente suite WIP103 al sorgente finale23.

Baseline corrente **104 PASS =31core+60desktop+13IO**, exit0:
`cargo test --workspace --features eframe/x11,ui-preview --locked`.
Log: [baseline-test](../../qa/leo24/logs/baseline-test.log).
Primo tentativo senza X11 rifiutato dal compilatore winit: conservato come
tentativo fallito, non conteggiato PASS. Build preview corrente PASS.
Due smoke reali egui/eframe Glow Linux/Xvfb1450×900 chiaro/scuro, exit0;
catture ispezionate in `docs/qa/leo24/screenshots/baseline-*.png`.

Cloud connected/current/enforced, proxy/CA ereditati conservati. Rust1.99.0
nelle cache esistenti, Xvfb display124. Linear get_agent_context backend
restituisce Tool not found; fonti lette con gli altri strumenti.
Windows nativo non disponibile, MSVC è compilazione soltanto. Nessun rinvio21
trasferito a24. Nessun merge/deploy/Done/nuova automazione o tracker write
dell'implementatore. Root gestisce claim/Linear/PR/revisori.

## WIP iniziale

Solo checkpoint/evidenze di baseline salvati; prodotto24 non ancora implementato.
Nessuna revisione indipendente effettuata. Prossima azione: RED pubblico egui
su comandi/footer e focus dei popup compatti, componenti modulari condivisi,
prime catture intermedie alla coordinatrice, poi verifiche e source freeze.

## WIP intermedio — ciclo0

Componenti condivisi per superfici/dialoghi, gruppi, messaggi espliciti e footer
usati negli editor, filtri, formato/testa, progresso e conferme. Le famiglie
mantengono Window/Modal originali e gli ID/eventi. Geometrie e worker invariati.
RED pubblico significativo: validazione lunga al minimo720×560/150% tagliava
Applica nell'editor Pesi; log compact-parameters-red.log. Spostando l'errore
nel corpo scorrevole e riservando il footer il test passa nei cinque editor
e nei due temi (compact-parameters-green.log). Il caso ordinario era già verde,
conservato come compact-parameters-baseline.log. Nessun esito completo finale
ancora dichiarato. Build/catture intermedie in preparazione per feedback root.
WIP non concluso, nessuna revisione indipendente, cicli sempre0.

## Candidato al congelamento sorgente — ciclo0

Feedback root sulle prime10 viewport native del WIP198ca86: gerarchia leggibile;
richiesto solo allineamento Chiudi alla dimensione32 degli altri footer.
Corretto inoltre il footer parametri nella finestra riaperta con altezza
memorizzata: il corpo rispetta sia il viewport sia lo spazio disponibile.
Regressioni parametri5 PASS sul WIP aggiornato; regressioni popup4 PASS,
inclusi marker dei selettori. Uscite con nomi/errori lunghi di filtri/formato/
testa/conferma al minimo720×560/150% nei due temi PASS. Editing/Tab/Esc dei
cinque editor con errore lungo PASS; il test richiede anche il valore editato.
Il primo tentativo di quest'ultima asserzione ometteva il modificatore egui
command nell'evento Ctrl+A e accodava testo: corretto solo l'evento di test,
nessun comportamento prodotto modificato per compensarlo. Log storici
esplicitamente distinti da esiti finali. Nessuna asserzione geometrica rimossa.

Il commit contenente questa nota è il candidato sorgente per nuove catture
e feedback finale. Le future verifiche complete e la matrice finale saranno
attribuite allo SHA effettivo, non ai WIP precedenti. Nessuna review ancora,
cicli0; nessun esito finale anticipato. Prossima azione: nuove viewport mirate,
freeze confermato, suite/controlli e matrice nativa sullo stesso SHA, due revisori
fresh coordinati da root. Windows nativo ancora non eseguito.

## Ripresa isolata — ciclo1 conservato

Nuovo implementatore `/root/leo24_resume`, creato `fork_turns:none`, nel solo
worktree/branch24 esistente. Baseline main7f74aad e source4b4a435 invariati
alla ripresa; ownership verificata dalla coordinatrice. Lettura corrente Linear
Leochist: In Progress+ready, rev1/stima100k invariati, commenti vuoti, nessuna
pausa. GO alla correzione nel perimetro confermato da root.

Il ciclo indipendente **1** era stato avviato dai precedenti revisori spec e
standard, poi terminati con la sessione senza verdetto finale salvato. Non è
PASS e il contatore non viene azzerato. Root avvierà ciclo2 sul nuovo freeze.

La suite completa source4b ha **62desktop PASS/1FAIL**: regressione
`extra_spacing_editor_is_visible_validated_applied_and_cancelled`, testo
Interasse aggiuntivo massimo non visibile. Fmt/check/Clippy4b PASS; release e
MSVC non eseguiti dopo quel fallimento. Le64 catture4b e relativi manifest/log
sono conservati in `docs/qa/leo24/historical/4b4a435`, con archivio SHA256;
nessuna attribuzione al futuro source. Nessun processo24 attivo salvo Xvfb124.

Prossima azione: RED della regressione, altezza iniziale adattiva dei cinque
editor conservando resize/manual rememberedheight/scroll/footer e disegni,
mirati, nuovo sourcefreeze, suite/catture sullo stesso SHA. Focus nei test di
editing viene richiesto internamente: non certifica focus iniziale automatico.
Windows nativo tuttora non disponibile, nessun waiver24. Tracker/PR a root.

## Nuovo source freeze per ciclo2

Fix prodotto limitato a `default_height` adattiva dei cinque editor: primo
layout con spazio per diagrammi/campi, senza forzare l'altezza dopo resize.
Disegni, body/scroll/footer e Window/Modal invariati. Test pubblico nuovo
sull'apertura utile nei due temi RED (ottavo peso invisibile) → GREEN; test
aggiunto di drag reale del bordo e riapertura verifica altezza ricordata e
azioni raggiungibili. Primo tentativo del test drag usava una seconda pressione
al posto del solo movimento: log conservato; corretto soltanto l'evento del test.
Commento del test focus chiarito: request_focus interno, poi editing/Tab pubblico.

Mirati prima freeze: parametri6PASS; drag/riapertura1PASS; regressione
Interasse1PASS (RED riprodotto prima fix); popup4PASS. Nessuna asserzione rimossa.
Preview nativa intermedia chiara1450×900 ispezionata: diagramma integro, campi
Interasse e successivi visibili, footer accessibile. Non è evidenza del freeze.

Il commit contenente questa nota fissa il sorgente per suite completa, controlli
fmt/check/Clippy/release/MSVC e nuova matrice editor36 viewport in due temi
(normale/compatta/150%, minimo/errore). La matrice storica64 resta attribuita
esclusivamente a4b; le altre famiglie di popup non hanno delta sorgente dal4b.
Ciclo1 interrotto senza verdetti, ciclo2 affidato da root a due revisori fresh;
nessun PASS indipendente anticipato. Windows fisico non eseguito.
