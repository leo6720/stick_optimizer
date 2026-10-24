# LEO-22 — area input e cornice A Essenziale

## Contratto e autorizzazione

Solo https://linear.app/leochist/issue/LEO-22, parent LEO-11.
Definizione accettata rev1 e Dimensionamento rev1, 95k (75–120k), accettate
23 ottobre 2026 con «Si perfetto» alla conferma di specifica, verifiche,
suddivisione e sviluppo/prosecuzione dei figli dopo i merge dell’utente.
Spec congelata: 5a3e830ea77073de68c65031c8615856d0d935a1,
`docs/design/leo11/spec-v1.md`; riferimenti A chiaro/scuro varianti-v4
ispezionati. Input sinistra, menu/cornice/stato; risultati/dettagli inline23
e popup24 esclusi. Motore, formule, ranking, geometrie, studi, CSV e parametri
invariati. Nessun merge/deploy/nuova automazione.

## Presa e isolamento

Implementatore `/root/leo22_implement`, nuovo `fork_turns:none`, senza cronologia
della coordinatrice, distinto dall’implementatore21. Coordinatrice `/root`
gestisce claim Linear, PR e due revisori fresh separati. GO ricevuto dopo claim
In Progress, subordinato alla suite baseline: PASS prima di editare prodotto.
Branch `codex/leo-22-async`, checkout isolato `/workspace/stick-leo22-async`.
Baseline e head iniziale `7a2c112e217c4987df7057d5f9b8ada43fcd15b6`.
PR non ancora aperta. **Ciclo review0**, massimo3 senza reset.

## Preflight e dipendenza21

PR27 verificata merged su main, merge7a2c112, head revisionato4d8c77a4096672e8ef53d1d4a59562b108fb2952
raggiungibile e tree identico c4bb02c54eafeea14cbc309bfc56d42f4d3c642a.
Main remoto verificato7a2c112. Nessun duplicato branch/worktree/PR22 prima della
creazione del checkout. Workspace/team/progetto Linear verificati, ticket22
letto con commenti (nessuno) e relazioni: unico prerequisito21. Parent11 letto.
`get_agent_context` non disponibile lato server; nessuna nota inventata.
Manifest21:46PNG verificati per hash PNG/RGBA, dimensioni, exit e provenienza.
Codice/cache toolchain1.99 invariati, Cargo.lock locked. Suite main rieseguita:
**96PASS=31core+52desktop+13IO**, doctest0. Include GUI tema/menu/blocchi e IO
pubblica isolata. Build preview main PASS. Log preservati in QA del ticket.

## Ambiente e limiti

Linux/X11, Xvfb disponibile. Runtime connected, policy enforced con osservazioni
correnti; proxy/CA ereditati conservati. PATH Cargo esplicito nella cache; il
primo tentativo senza PATH ha fallito prima di Cargo, non è un test prodotto.
Windows nativo assente: cross-check MSVC distinto da esecuzione. RinvioWindows
concesso dall’utente per PR27 non esteso a22. Prove input fisico, picker e DPI
native da riportare come mancanti fino a esecuzione effettiva.

## Prossima azione

Test mirati degli eventi editing/focus/azioni e conservazione IO, poi composizioni
A con token/componenti fondazione; QA nativa normale/compatta/minimo/zoom nei due
temi, controlli locked e consegna fixed head ai revisori della coordinatrice.

## Milestone implementazione — ciclo0

Test-first: output delle unità dei quattro campi assente RED, poi GREEN con
editing reale tramite eventi GUI nei due temi e roundtrip IO pubblico isolato.
Nuove composizioni input in `app/inputs.rs`: gruppi dimensionali, righe formato
contigue, quantità esplicita, selezione, campi30punti e azioni32punti. Gli ID
restano quelli precedenti; collasso, rimozione e conferme invariati. Rigenerati
schemi vettoriali simbolici con token condivisi; nessun dato o formula del core.
Cornice con marca, menu e stato; area risultati e popup conservata.

Suite desktop intermedia53/53PASS. Riscontro visivo della coordinatrice
(pre-review, non revisore indipendente): corretta continuità dei gruppi,
misure dei comandi e selezione. Taglio etichettaAs riprodotto con RED sul
boundingbox del testo renderizzato; corretto margine e GREEN alle4dimensioni.
Terza prova aggiunta: wheel+click nel compatto760/720 rende raggiungibili le
azioni, aggiunge uno stick e conserva la conferma/annullamento della rimozione.
Ultimi4test miratiPASS (3nuovi + regressione worker). Fonte corrente pronta al
freeze, controlli finali e QA persistita ancora da completare. Prove Windows
native ancora assenti; compilazione separata da esecuzione.

## Freeze e revisione ciclo1

Source freeze iniziale874cb801c78a45f663563f301ad9b71f83cd9b01, push normale.
Due revisori fresh gestiti dalla coordinatrice, **ciclo corrente1** (non reset).
Fmt/checkPASS; Clippy ha segnalato `single_match` nel solo ramo QA focus.
Correzione equivalente `if let` coordinata con root prima della mutazione;
nessun altro cambiamento prodotto. Nuovo freeze dopo verifica dei tre controlli,
poi suitefinale e catture su talehead. Nessun esito dei revisori anticipato.

## Consegna QA su source congelato — ciclo1

Sorgentef0e4b46b64710c33f271e2a85db1abe3da626ca2, baseline7a2c112.
**99testPASS=31core+55desktop+13IO**, doctest0. Fmt/check/Clippy-Dwarnings,
buildpreview, buildreleaseLinuxX11 e checkMSVC tuttiPASS. Nessuna esecuzione
Windows nativa dichiarata.28catture reali e ispezionate nei due temi, normali,
compatte/minimo/150%, scorse, menu, vuoto/errori, selezione e nomi lunghi.
HashPNG/RGBA, dimensioni, exit0 e provenienza verificati; tutto in
[rapporto QA](../../qa/leo22/README.md), manifest e log relativi.

Revisori fresh `/root/leo22_spec_review` e `/root/leo22_standards_review` sullo
stesso baseline/source. Root ha riferito esito favorevole codice; QA/final-head
in attesa di conferma indipendente. Unico rilievo documentale: frase storica
che descriveva ancora i JPGsidebar incorporati, allineata alla versione corrente
nella passata di sole evidenze/documenti. Nessun’altra mutazione del sorgente
successiva al freeze. **Ciclo1 conservato**, non azzerato dalla correzione lint.

Prossima azione: commit/push sole evidenze, ACK dei revisori tramite root,
PRdraft e aggiornamento InReview/ready-for-human per Windows22 gestiti da root.
PRdraft unica22 aperta da root: https://github.com/leo6720/stick_optimizer/pull/28,
verso main, base7a2c112/sourcef0e4b46. Entrambi i revisori favorevoli sul sorgente;
standard ha inoltre letto i log finali99PASS e controlli, zero difetti concreti.
Root acquisirà i final ACK del pacchetto complessivo dopo questo push di sole
evidenze/documenti. Nessun merge/deploy/Done.
