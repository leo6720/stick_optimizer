# LEO-23 — evidenze e checkpoint di consegna

24 ottobre 2026. PR draft: https://github.com/leo6720/stick_optimizer/pull/29 .
Baseline main `b985d30588f4d4dce696254803bb84e1a761d5d2`.
Sorgente finale congelato `3ae1ffcb2aea1bc36dbcfe496eca015d72fb4d95`.

Risultati, filtri inline, riepilogo, schede e penalità sono ridisegnati in Rust.
Input e popup restano nei rispettivi ticket; core, IO, formule, precisione,
identità e formati dei dati sono invariati.

## Verifiche effettive e provenienza

- Baseline: 99 test PASS, prima delle modifiche23.
- Suite di ripresa: 103 PASS (31 core, 59 desktop, 13 IO),
  [log completo](logs/workspace-resume-test.log). Eseguita sul WIP prima degli
  ultimi interventi header e del fix P2; non è una suite completa104 sul sorgente
  finale. La nota dell'implementatore è in [source-provenance](logs/source-provenance.txt).
- Sul sorgente finale: sei test mirati PASS. Selezione formato con marker e
  target di apertura nei due temi [1](logs/selected-format-green.log),
  doppio clic formato/testa [2](logs/cycle1-double_click.log),
  15/7/8 colonne e scorrimento compatto [1](logs/cycle1-output_detail_columns.log),
  dataset denso [1](logs/cycle1-dense_real.log), filtri/rango/CSV e assenza di
  ricalcolo [1](logs/cycle1-results_toolbar.log).
- Sul sorgente finale: fmt, check e Clippy con tutte le feature/target,
  build preview/release Linux e check MSVC PASS, come riferito dall'implementatore
  e conservato nei log `cycle1-*`. MSVC certifica compilazione, non esecuzione Windows.

I log RED, la suite intermedia fallita e gli altri log storici sono conservati
come tali. Non sono fallimenti attuali né verifiche finali aggiuntive.

## Catture accettate

[manifest.json](manifest.json) contiene **28 viewport reali egui/eframe Glow**,
tutti dal sorgente finale3ae1ff, con hash PNG/RGBA, dimensioni ed exit code.
Coprono risultati, cassetti, teste, normale/760/720/zoom150%, dati densi e nomi
lunghi nei due temi. I PNG sono in [screenshots](screenshots).

La coordinatrice ha controllato tutti28 hash, dimensioni, provenienza ed exit0:
[coordinator-integrity.json](coordinator-integrity.json). È un controllo di
integrità della coordinatrice, non una revisione indipendente sostitutiva.
Le28 viewport precedenti ce554 sono archiviate in [snapshots](snapshots),
con la propria provenienza storica; non vengono attribuite al sorgente finale.

**Acquisizioni respinte:**18 letture XCB nere e8 viewport con gli stati XTest
richiesti non applicati. Non fanno parte delle28 prove accettate e non
certificano selezione, tooltip, scroll o penalità dopo eventi esterni.
Metadati espliciti: [framebuffer](rejected-framebuffer.json),
[stati nativi](rejected-native-state.json),
[limiti del metodo](logs/native-method-limitations.txt).
Gli eventuali `exit0` nei loro log riguardano soltanto il processo di acquisizione.
Il comportamento di selezione/scorrimento/apertura è verificato separatamente
dai test a eventi e output egui sopra, non da quelle immagini respinte.

## Revisioni e residui

Ciclo1: `/root/leo23_spec_review` e `/root/leo23_standards_review`, creati
separatamente con `fork_turns:none`, hanno trovato un P2: il formato selezionato
era indicato solo dal colore. Correzione minima marker✓/peso e regressione
RED→GREEN. Ciclo2: entrambi hanno confermato il fix e dato ACK favorevole al
codice3ae1ff, senza altri difetti concreti.

La conferma indipendente di questo pacchetto finale QA/documentazione resta
da completare: un limite di utilizzo ha interrotto l'implementatore di ripresa
e il revisore SPEC. La coordinatrice conserva soltanto evidenze e checkpoint,
senza modificare il prodotto. Non dichiara concluso il gate indipendente finale.
La disponibilità di un nuovo revisore SPEC verrà verificata sul commit pubblicato.

**Windows nativo non eseguito**: input fisico, picker, chiusura, DPI/monitor
restano da provare. Nessun rinvio viene dedotto da altre PR. Nessuna approvazione
GitHub è espressa per l'utente, nessun merge/deploy/Done eseguito.

Prossima azione: conferma dei revisori sul commit contenente queste evidenze;
poi mantenere esplicito il residuo Windows e la decisione di merge dell'utente.
