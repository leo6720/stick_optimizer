# Tracker di Stick Optimizer

- Provider: Linear. Workspace: [leochist](https://linear.app/leochist).
- Workspace UUID: bbb36fe9-e6a2-44b5-aa0a-f46979dc5733.
- Team: Leochist, chiave LEO, UUID e93e7bef-869c-49c7-b8ea-c4d34fd34992.
- Progetto: [Stick Optimizer](https://linear.app/leochist/project/stick-optimizer-196bf9721c91).
- Progetto UUID: 28c4112e-b260-4ccd-9480-77da5dd48eb2.
- Repository: leo6720/stick_optimizer. Base delle nuove PR: main.
- La migrazione Rust è integrata tramite PR #17; rust-rewrite è storico.
- Configurazione introdotta con LEO-5.

## Selezione e accesso

Prima di leggere o scrivere, selezionare la connessione del workspace leochist
e verificarne l'identità. I link_id delle connessioni sono specifici della
sessione: ricavarli dagli strumenti disponibili, non salvarli nel repo.
Kingsprono è un workspace distinto: non usarlo per questo progetto.
Con più connessioni indicare esplicitamente quella di leochist.
Nel cloud usare il connettore Linear; sul PC usare un client/MCP Linear
autorizzato sullo stesso workspace. Non presumere che l'accesso cloud valga sul PC.
Non chiedere token in chat e non salvarli nel repository.

## Operazioni

Leggere ticket tramite ID o URL, inclusi descrizione, commenti e dipendenze.
Creare ticket nel team LEO e nel progetto UUID sopra, riportando i criteri del
template in docs/agents/templates/ticket.md. Prima di crearli cercare duplicati.
Usare relazioni native blockedBy e parentId, dopo aver risolto gli UUID reali;
non dedurre UUID da identificativi LEO-N e non modificare i blocchi già presenti
senza averli letti. Un ticket può partire solo se i suoi blocchi sono risolti.
L'etichetta ready-for-agent da sola non basta: per la presa asincrona servono
stato Todo, sezioni Definizione accettata e Dimensionamento, dipendenze risolte.
Seguire [token-sizing.md](token-sizing.md): circa 150.000 token per ticket,
parent non esecutivo quando suddiviso, stima/criteri/verifiche per ogni figlio.
Seguire docs/agents/async-workflow.md per presa, checkpoint e revisione.

Gli stati attuali sono Backlog, Todo, In Progress, In Review, Done, Canceled
e Duplicate. Rileggerli prima delle scritture: i nomi possono cambiare.
Nuove idee vanno in Backlog con needs-triage. Un ticket chiarito va in Todo con
ready-for-agent; all'avvio In Progress; con PR aperta In Review.
Done richiede integrazione e verifica dei criteri: non chiudere al solo commit
o all'apertura della PR. Se serve una prova manuale usare ready-for-human.

GitHub ospita codice e PR, Linear è la fonte delle attività: non duplicare
automaticamente i ticket in GitHub Issues. Mettere l'ID LEO-N nel branch e
il link Linear nella PR. L'integrazione GitHub è stata collegata dall'utente;
la mappatura degli stati automatici non è stata verificata.
Prima di usare parole di chiusura come Fixes controllare quando scatterebbe
Done, in particolare per PR su rust-rewrite. Usare Related to e aggiornamento
manuale finché la mappatura non è verificata.
Nessun merge, deploy o avvio di automazioni schedulate è autorizzato dal solo
stato di un ticket. Se Linear non è accessibile, preparare bozze locali in
docs/agents/drafts/ e dichiarare che non sono pubblicate; non cambiare tracker.
