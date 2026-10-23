# LEO-20 — metodo originale per grill, specifica e ticket

## Ripresa su richiesta utente — revisione 2

Dopo la prima consegna l'utente ha chiesto se il metodo originale comprendesse
anche i ticket e ha risposto «facciamolo» alla proposta di allineare to-spec e
to-tickets mantenendo Linear, sizing150k e autorizzazioni. Estensione esplicita
persistita nel ticket il 23 ottobre 2026: supera il perimetro iniziale per queste
due skill e i template/riferimenti connessi. Stima rivista T45000/N1, motivazione
e incertezza nel ticket; nessun prodotto e nessuna nuova automazione.

Stesso branch/PR26 e baseline main `074634d28a3d57433f87a3524a15b528cf0da1e1`;
head della prima consegna `762f1eafa06109a54ea4e95af733a42b0e75dc25`.
Coordinatrice modifica solo istruzioni/documentazione nel checkout isolato.
Fase corrente: istruzioni verificate e revisionate, PR26 pronta per la decisione
di merge dell'utente. Round1 PASS storico conservato sotto; round2 concluso PASS,
nessun rilievo aperto. Head delle istruzioni revisionato:
`4e68d5d9230b68c56c789a55522d195e1a8a2992`; il commit successivo modifica solo
questo checkpoint, con SHA finale registrato in PR/Linear dopo push.

To-spec/to-tickets includono fonti integrali fissate, metodo operativo completo
e precedenze locali esplicite. Template includono storie/decisioni/confini dei
test, copertura dei criteri, revisione approvata del piano o delega esplicita.
Nessun ready dalla sola specifica, nessun figlio bloccato pronto, parent solo
contenitore quando suddiviso, prosecuzione autorizzata preservata.
Test Rust non pertinenti; nessuna certificazione di caricamento automatico.

Controlli della revisione2: due nuovi riferimenti identici ai blob upstream
`3f52599ae2a4347aee5a07432c2707518e691a7f` (to-spec) e
`9b77a01db9460c99c22d03c910c349f890f12c0e` (to-tickets); quattro copie precedenti
invariate, frontmatter delle dieci skill, riferimenti operativi e diff whitespace
PASS. Nessuna suite Rust eseguita per un cambiamento solo documentale.

Due nuovi revisori indipendenti `fork_turns: "none"` sul diff completo
baseline/head sopra e sul delta dalla prima consegna:

- SPEC `/root/leo20_spec_tickets_review`: PASS, zero rilievi; lettura completa
  tramite GitHub/Linear e identità dei due blob confermata. Shell non utilizzabile
  nel suo contesto per errore di trasporto; non dichiara controlli shell eseguiti.
- Standard/correttezza `/root/leo20_spec_tickets_standards`: PASS, zero rilievi;
  controllati originali, precedenze, template e gate; diff whitespace PASS.

Round2 cumulativo, zero cicli irrisolti. Non sono approvazioni per conto
dell'utente. Nessuna modifica a prodotto, automazioni, permessi o tracker di
altre attività; prossima azione decisione di merge dell'utente, non Done.

---

## Prima consegna — revisione 1, preservata

## Richiesta e perimetro

Il 23 ottobre 2026 l'utente ha accettato di conservare il comportamento della
grill-me originale aggiungendo la documentazione specifica del progetto.
Definizione e dimensionamento revisione1 persistiti in
[LEO-20](https://linear.app/leochist/issue/LEO-20/ripristinare-il-metodo-grilling-originale-nelle-skill-di-intervista).
Manutenzione delle istruzioni richiesta direttamente; nessuna presa di altro
lavoro dalla coda, implementazione di prodotto o nuova automazione.

## Esecuzione

- Baseline main: `074634d28a3d57433f87a3524a15b528cf0da1e1`.
- Branch: `codex/leo-20-grilling`; checkout isolato `/workspace/stick-grilling-upstream`.
- Autore: coordinatrice `/root`, solo documentazione e skill, nessun prodotto.
- Head delle istruzioni verificato: `14601c4b18f25a337b4aba784eb1ccf328101864`.
- PR: https://github.com/leo6720/stick_optimizer/pull/26.
- Fase: modifiche verificate e revisionate; aggiornamento finale solo di questo
  checkpoint. SHA finale dopo push registrato nella PR e nel ticket.
- Cicli revisione: 1, due PASS, zero rilievi aperti.
- Prossima azione: decisione di merge dell'utente, non Done prima dell'integrazione.

## Contenuto e controlli

Quattro file copiati integralmente dalla revisione upstream fissata:
grilling, domain-modeling e i due formati. Provenienza/licenza in
[UPSTREAM.md](../../../.agents/skills/UPSTREAM.md).
Ingressi grill-me e grill-with-docs con lettura esplicita delle dipendenze;
round sulla frontiera, raccomandazioni, risposte attese e conferma finale.
Il primo non scrive artefatti; il secondo conserva termini, ADR qualificati e
decisioni del ticket. Gate di specifica, dimensionamento e ready invariati.

Controlli eseguiti: identità byte per byte dei quattro file importati, frontmatter
di tutte le dieci skill, riferimenti locali delle skill coinvolte e
`git diff --check`: PASS. Nessuna suite Rust eseguita, nessun codice modificato.

Revisioni indipendenti con contesti nuovi `fork_turns: "none"`, sul diff completo
baseline/head sopra: `/root/leo20_grill_spec` e `/root/leo20_grill_standards`,
entrambi PASS senza rilievi; rieseguiti controlli documentali pertinenti.
Non sono approvazioni a nome dell'utente.

## Limiti

La presenza dei file non certifica indicizzazione automatica, registrazione nel
catalogo o comandi slash; lettura esplicita supportata quando manca lo strumento
Skill. Nessuna installazione globale o modifica al runtime/client.
`get_agent_context` Linear indisponibile lato servizio; workspace leochist
verificato e fonti repo/ticket accessibili direttamente.
Nessun merge, deploy, cambio di automazioni o dato utente.
