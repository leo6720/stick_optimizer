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

Baseline completa PASS: **87 test** (29core/47desktop/11IO).
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
