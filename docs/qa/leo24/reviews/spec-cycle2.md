# LEO-24 — revisione indipendente SPEC, ciclo 2

Revisore `/root/leo24_spec_cycle2`, creato `fork_turns:none`, separato da coordinatrice e implementatore. Nessuna modifica al prodotto o al tracker. Baseline fissata `7f74aad53c540eeebc36897f8f084727b83f499a`; head sorgente fissato `46bd2ca55f6856b9767f8165a24ac5a0e4ed56b9`. Esaminato il diff SHA→SHA completo dei nove file Rust e della documentazione pertinente; nessun diff core/IO/Cargo.lock/toolchain. Contatore persistente **2**: ciclo 1 interrotto senza verdetto, non passato né azzerato.

Fonti: AGENTS.md, workflow/issue-tracker/async-workflow/token-sizing, skill locale `.agents/skills/code-review/SKILL.md`, CODING_STANDARDS.md, checkpoint LEO-24, documentazione Rust. Linear letto tramite connessione Leochist verificando workspace UUID `bbb36fe9-e6a2-44b5-aa0a-f46979dc5733`, issue UUID `3de3ee09-91e5-4afa-ba77-99bee4460378`, rev1/stima100k/commenti vuoti e dipendenza solo LEO-21. Specifica `docs/design/leo11/spec-v1.md` letta da commit congelato `5a3e830ea77073de68c65031c8615856d0d935a1`; A-chiaro/A-scuro varianti-v4 entrambi aperti con view_image.

## Riscontro sul sorgente

- Editor cinque/filtri/formato/testa/conferme/errori/progresso usano realmente superfici, gruppi, pulsanti, notice e footer condivisi in ui.rs. Palette/token e gerarchia coerenti con A; stato errore esplicito oltre al colore e spunta dei selettori.
- `parameters.rs:130–197`: layout adattivo e scroll del corpo con footer fuori scroll. Nuovo default_height lascia alle finestre le dimensioni ricordate. Applica/Annulla/Esc/X e reset del solo gruppo conservano i percorsi precedenti; applicazione/validazione/invalidazione non cambiano. Nessun salvataggio automatico dei parametri.
- `details.rs:79`, `heads.rs:88`: Window rimangono Window; disegni, quote e coordinate dal core non ricalcolati; viste prima/dopo, tutti i prelevatori e strato completo conservati. Contenuti aggiunti ai gruppi scrollabili, non eliminati.
- `progress.rs:29–107`, `app.rs:944–1013`: Modal rimangono Modal; nessun delta al lifecycle worker, atomiche, blocco input/menu/shortcut o installazione risultati. Stati indeterminati e percentuale reale mantengono i criteri precedenti. File picker OS non sostituiti.
- Test aggiunti osservano rendering/eventi per apertura di cinque editor×due temi, footer e uscite con errori/nomi lunghi al minimo/150%, drag e altezza ricordata; regressione Interasse non indebolita. Test di geometrie pubbliche, filtri, dati/CSV e worker reale conservati.

## Rilievi

**Difetti bloccanti introdotti nel diff: nessuno individuato.**

Suggerimento non bloccante, `crates/desktop/src/app.rs:3072`: il test di editing chiama `request_focus` internamente prima di inviare testo/Tab; non certifica focus iniziale né ingresso nel campo solo tramite navigazione pubblica. L'implementazione conserva ID/ordine e non introduce gestione del focus; non emerge una regressione dimostrata. Utile una futura prova dall'apertura via menu a Tab/click senza request_focus. Non attribuire alla prova corrente una copertura maggiore.

## Evidenze lette e limiti

Storico4b: archivio135file verificato SHA256, zero mismatch. Manifest64 e suite fallita62desktopPASS/1FAIL conservati correttamente come storici; non sono esito finale46bd. Aperte con view_image le PNG storiche filters-error720dark, detail720light, head-popup-scaled-bottom150dark, unsaved150light, progress-cancelling150dark. Quote, tutti12prelevatori nelle due viste e footer visibili nei campioni. Verificato diff4b→46bd: prodotto cambia solo default_height parametri, le altre famiglie non hanno delta sorgente.

Nuovo46bd finale: manifest **36/36**, tutti36 PNG verificati indipendentemente per SHA256 file, SHA256 RGBA, dimensioni, exit0 e source46bd: zero discrepanze. Ispezionate personalmente con view_image10 PNG46bd: astucciatrice normalechiaro/compattochiaro/150%scuro, pesi normalechiaro/150%scuro, numero risultati compattoscuro, obiettivoAB150%chiaro, MT normalechiaro ed errore720×560/150% nei due temi. Campione comprende tutte5famiglie editor. Interasse e campi iniziali visibili in normale, diagrammi intatti, footer raggiungibili in compatto/zoom/errore; gli altri campi/reset restano nel corpo scorrevole.

Letti checks.json e log finali: fmt/check/Clippy/full suite/release/MSVCcross/previewbuild **exit0**, tutti con source46bd. Suite **109PASS =31core+65desktop+13IO**, zero fail/ignored; verificate in particolare regressione Interasse, apertura editor, drag/riapertura, quote/strato/prelevatori e real worker/cancel. Release9.41s, MSVCcross1.46s. Non ho rilanciato la suite concorrente: questi sono esiti dell'implementatore controllati nei log, insieme a verifica indipendente del diff e delle evidenze. HEAD46bd e nessun delta sorgente rispetto al freeze al controllo finale. Le fixture di progresso/errori certificano aspetto, non avanzamento reale.

**Gate umano distinto dai difetti:** Windows nativo non eseguito, nessun waiver LEO-24; Linux/Xvfb e compilazione MSVC non certificano input/file picker/chiusuraOS/multi-monitorDPI Windows. Richieste prove native o decisione esplicita dell'utente prima dell'accettazione completa; il revisore non approva per l'utente e non effettua merge.

## Verdetto

**SPEC ciclo2: PASS della revisione del diff fissato, nessun rilievo bloccante di prodotto.** Criteri verificabili in questo ambiente coperti da sorgente, test e catture; suggerimento focus non bloccante distinto. **Accettazione completa resta subordinata al gate umano Windows nativo**, non eseguito e senza waiver24. Questo verdetto non approva per l’utente, non chiude il ticket e non autorizza merge. Le evidenze4b restano storiche, separate dal PASS109 e36PNG46bd.
