---
name: implement
description: "Implementare un ticket della beta Rust con verifiche e consegna tramite PR."
---

# implement

Adattamento locale per Stick Optimizer. Riferimento: docs/agents/workflow.md.

Leggere AGENTS.md, il ticket completo e docs/agents/issue-tracker.md.
Dichiarare titolo, criteri, dipendenze e baseline Git verificata prima di editare.
Partire da main aggiornato (beta Rust); preservare modifiche dell'utente.
Usare un branch dedicato con ID LEO e base PR main.
Portare il ticket In Progress. Applicare tdd per comportamento e bug.
Usare controlli proporzionati per documentazione o modifiche cosmetiche.
Eseguire prove mirate durante il lavoro e controlli pertinenti alla consegna.
Leggere code-review e revisionare il diff prima della PR.
Nel flusso asincrono seguire docs/agents/async-workflow.md: presa esplicita,
checkpoint, revisori separati e correzioni autonome. Se il runtime non permette
esecuzione o revisione indipendente registrare il blocco, senza inventare esiti.
Allegare test, limiti e link Linear alla PR draft; portare In Review.
Non eseguire merge, deploy o chiusura del ticket al solo commit.
Non iniziare altre funzioni perché appaiono nel piano futuro.

Prima della presa verificare Dimensionamento secondo docs/agents/token-sizing.md:
non implementare macro da scomporre, ticket senza stima o oltre circa 150.000 token.
Controllare il piano parent e non sostituire la fase di definizione. Implementare
il ticket in una sessione/sub-agente nuovo senza cronologia completa ereditata;
se impossibile registrare il blocco di isolamento.
