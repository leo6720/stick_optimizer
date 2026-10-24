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
