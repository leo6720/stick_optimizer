# LEO-21 — fondazione UI e tema globale

## Contratto e autorizzazione

Solo LEO-21 del piano LEO-11, Definizione accettata revisione1 e Dimensionamento
revisione1 (120k, intervallo100–145k). Fonte utente23ottobre2026: «Si perfetto»
alla conferma di specifica, verifiche, quattro ticket e sviluppo/prosecuzione dopo
merge. Specifica congelata: `5a3e830ea77073de68c65031c8615856d0d935a1`,
`docs/design/leo11/spec-v1.md` nel branch `codex/leo-11-design`;
documento Linear `f10d260e-d6cc-48c5-95ce-9a723b54aff0`.
Ticket: https://linear.app/leochist/issue/LEO-21 . Nessuna dipendenza.
LEO-22/23/24 e ridisegno completo A di input/risultati/popup esclusi.
Nessuna modifica motore, formule, ranking, geometrie, .stic, CSV o parametri.
Nessun merge, deploy, automazione o Done autorizzato.

## Esecuzione

Implementatore nuovo `/root/leo21_implement`, creato con `fork_turns:none` senza
cronologia coordinatrice. Claim Linear e GO gestiti dalla coordinatrice dopo
rilettura di Todo+ready, assenza di altre prese e controllo duplicati.
Baseline main remoto verificata: `44bb771d7e87a89b08cba07103316c3b333e302c`.
Branch unico `codex/leo-21-async`, worktree `/workspace/stick-leo21-async`.
Nessun branch/PR precedente per LEO-21. Cicli di review iniziali: **0**.
Due revisori nuovi separati sullo stesso head verranno gestiti dalla coordinatrice.

## Avanzamento

Preflight completo; Rust1.99 e Xvfb disponibili. Test baseline avviati.
Prima prova RED: API IO pubblica per preferenza UI separata, schema/versione,
round trip, assenza file, input invalidi e scrittura fallita senza danni.
Prossimo: completare IO, eventi menu e immutabilità studio, componenti reali e
migrazione colori, poi controlli completi e anteprime della GUI e dei popup.

## Limiti ambiente

Linux con backend X11 e Xvfb. Windows nativo non disponibile: cross compilation
MSVC distinta da esecuzione. Input/picker/DPI Windows richiedono prove native
umane; non è trasferita alcuna rinuncia da ticket precedenti.
Il tool Linear get_agent_context risponde Tool not found; ticket/spec letti
integralmente tramite connettore. Nessuna credenziale conservata nel repository.

### Milestone implementazione

Baseline completa PASS: **89 test** (31core/47desktop/11IO).
IO test-first: RED API mancanti osservato, poi GREEN2 nuovi test pubblici.
GUI test-first: RED constructor/menu mancanti osservato, poi GREEN3 prove a
mouse/eventi; conservazione studio/view/risultati/CSV e file parametri,
riavvio/Nuovo/Apri/Esempio/errori, blocchi worker/conferme. La prima bozza
harness usava ctx.style e Result CSV senza unwrap, corretti prima del RED
significativo; non sono stati conteggiati come difetti prodotto.
Font InterVariable.ttf4.1 originale con OFL e hash in assets/README.
ui.rs collegato ad azioni primarie/secondarie/quiete, campi/tabelle/titoli e
contenitori. Migrazione di tutti override, progresso e disegni; test coordinate
testa conservati e ripetuti nei due temi. Nessuna modifica core o formati dati.
Contrasto iniziale del bordo chiaro insufficiente su hover rilevato dal test
oggettivo; palette corretta prima della suite finale.
Prima cattura reale Xvfb results-dark1450x900 acquisita e ispezionata;
matrice completa e controlli finali ancora da completare. **Ciclo0**.

### Correzioni pre-review (ciclo0)

L'ispezione della coordinatrice sulla prima principale chiara ha rilevato
intestazioni input quasi bianche. Causa: egui usa `widgets.active.fg_stroke`
anche per `strong_text_color`; la prima mappa poneva il bianco previsto per
azioni primarie su superfici ordinarie. Correzione semantica globale: active
neutro/hover con testo leggibile e bordo accent; Primary conserva on_primary
esplicito. Regressione sul testo strong realmente renderizzato RED→GREEN
in entrambi i temi. Prime catture obsolete, rigenerazione integrale prevista.
Questo controllo della coordinatrice non è una review indipendente.

La prima suite integrata ha trovato due assunzioni del test harness:
identificazione primitive tramite altezza42 esatta in f32, ora tolleranza0.001
mantenendo tutti i12 stick/prelevatori e asserzioni coordinate/larghezze;
quote basse nel popup fuori clip con le metriche Inter, ora controllate dopo
un vero evento wheel sul contenuto. Nessuna asserzione geometrica o quota
eliminata. Test popup mirato GREEN. Correzione harness menu: clic noto sulla
checkbox dietro conferma, non semplice apertura del menu.

## Consegna host verificata e review in corso

**Ciclo corrente1**, avviato dalla coordinatrice con due contesti nuovi
`fork_turns:none`: `/root/leo21_spec_review` e `/root/leo21_standards_review`.
Implementatore sempre `/root/leo21_implement`, distinto dai revisori.
Non è un reset dei cicli: ciclo0 indica soltanto i milestone pre-review sopra.
Massimo3cicli irrisolti; nessun cambiamento del contratto per far passare test.

Prodotto congelato `b6c78c80b08dee35088cc93ba09aee8c9cd7aa92`.
Ultimo sorgente `1c4615fbfb74e0ee4fc8e0a3970f53cf1de3aef0`: solo la gallery
QA cfg(ui-preview) confina due celle a24punti, senza alterare helper/callsite
prodotto. Correzione autorizzata dalla coordinatrice dopo ispezione della fixture;
i revisori devono confermare questo head e il successivo commit di sole evidenze.
Nessuna ulteriore mutazione prodotto mentre le review sono in corso.

Esiti finali: **96test=31core (2unitari+29integrazione)+52desktop+13IO**,
doctest0. Baseline effettiva **89=31+47+11**. I riepiloghi iniziali87/94
omettevano erroneamente i2unitari core: aritmetica corretta, log non modificati.
Fmt/check/Clippy-Dwarnings/test/buildreleaseLinuxX11/checkMSVC tutti PASS su b6.
Dopo la sola correzione gallery: fmt/check/Clippy/2testUI/buildpreview/checkMSVC
PASS. Non dichiarata una seconda suite completa dopo questa sola fixture.
Cargo.lock, toolchain1.99, core, schema studio/CSV e parametri invariati.

**46catture reali** lanciate e ispezionate: principali, menu, parametri, filtri,
progresso/quattro stati, formato/quote scorse, testa, componenti e prove compatte,
zoom150%, minimo720×560, in entrambi i temi. Tutte exit0. Le44catture prodotto
sono su b6; le2gallery rigenerate su1c. Fixture progresso distinta dal worker.
Tutte persistite nel branch, ricompressione PNG senza modificare pixel verificata;
provenienza, percorsi relativi, hash, dimensioni e log nel
[rapporto QA](../../qa/leo21/README.md) e relativo manifest.

Resta **gate umano Windows nativo**: desktop/input fisico/file picker/chiusura e
più monitor/DPI non eseguiti. MSVC è compilazione soltanto. Nessun waiver
inventato. Coordinatrice gestisce PR draft, rilettura/conferma dei revisori,
Linear InReview/ready-for-human e rimozione ready-for-agent. Nessun Done/merge.
Prossimo: pubblicare source/evidenze sul solo branch dedicato; coordinatrice
crea PR, poi registrare esclusivamente link/stato/esiti reali e final-head ack.

## PR e resoconti del ciclo1

PR draft verso main: https://github.com/leo6720/stick_optimizer/pull/27 .
Head pubblicato con codice e QA: `b80983a199bf69fcbfdcdc2c5f95eb604e271f2b`.
Entrambi i revisori fresh separati, `/root/leo21_spec_review` e
`/root/leo21_standards_review`, hanno dato esito favorevole e ACK su questo
head, senza difetti concreti aperti. Il revisore standard ha rieseguito7test
mirati unici; entrambi hanno confermato96test dai log e verificato
indipendentemente hashPNG/RGBA, dimensioni e provenienza delle46catture.
I resoconti sono revisioni di agenti, non approvazioni GitHub a nome dell'utente.
Nota non bloccante: i log stdout/stderr originali contengono righe vuote finali;
non sono stati normalizzati per preservare l'evidenza originale.

Questo ultimo aggiornamento modifica solo checkpoint/rapporto con link PR ed
esiti reali. Prodotto e QA restano quelli già revisionati; la coordinatrice
chiederà conferma sullo SHA conclusivo dopo push. Da questo momento **zero
mutazioni** in attesa del final ACK.

Stato operativo atteso dopo final ACK: **In Review + ready-for-human**, rimuovendo
ready-for-agent per il gate Windows nativo residuo. Prossima azione utente:
collaudo Windows di tema/persistenza e interazioni, input fisico, picker e DPI;
poi decisione distinta sul merge. Nessun Done al push e nessun merge automatico.
LEO-22/23/24 restano bloccati finché LEO-21 non sarà integrato e verificato su
main secondo il registro di prosecuzione: questa PR non risolve la dipendenza.
