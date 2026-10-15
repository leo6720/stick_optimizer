# Workflow di sviluppo con agenti

Configurazione del 15 ottobre 2026 per LEO-5. È un adattamento locale,
non l'installazione integrale delle skill di Matt Pocock né una trascrizione
del video. Riferimento consultato:
https://github.com/mattpocock/skills/tree/49dd158d1076134a641b33efb035946536778336

## Uso quotidiano

Aggiungi in Linear un'idea, un bug o un esempio nel progetto Stick Optimizer.
Poi chiedi, per esempio:
- «Chiarisci LEO-N con grill-with-docs».
- «Trasforma questa richiesta in una specifica con to-spec».
- «Dividi la specifica in ticket con to-tickets».
- «Implementa LEO-N».
- «Rivedi la PR rispetto alla specifica e agli standard».
- «Fai la retrospettiva della sessione».

Le sette skill locali sono in .agents/skills/: grill-with-docs, to-spec,
to-tickets, implement, tdd, code-review, retro. Aprire il relativo SKILL.md.
Su client che le rilevano automaticamente si possono usare i comandi del client.
Se il cloud non le mostra nel catalogo, chiedere esplicitamente di leggere il
file: la presenza nel repo non garantisce registrazione o comandi slash.

## Flusso

1. Chiarire solo le decisioni mancanti che cambiano comportamento o perimetro.
   Prima leggere codice, glossario e documentazione; non chiedere fatti reperibili.
2. Scrivere la specifica con criteri osservabili e confini di test.
3. Scomporre in ticket verticali e dipendenze native Linear; non creare backlog
   di nuove funzioni non richieste. Segnare pronte solo attività definite.
4. Lavorare una modifica alla volta; test-first per comportamento e bug,
   controlli proporzionati per documentazione e cambi cosmetici.
5. Rivedere il diff rispetto a uno SHA fisso su due assi: specifica e standard.
   Un'autorevisione va dichiarata e non sostituisce un revisore indipendente.
6. Aprire una PR draft con ID Linear, test e limiti; In Review fino
   all'integrazione e alla verifica dell'accettazione. Non eseguire merge.
7. Su richiesta fare retrospettiva: proporre miglioramenti a strumenti,
   accesso, documentazione e controlli basati su difficoltà realmente osservate.

## Cloud e PC

La configurazione è versionata nel repo. Ogni ambiente deve usare il branch
che la contiene e avere connessioni proprie a GitHub e Linear.
Non installare una seconda copia delle stesse skill globalmente.
Per il PC basta un checkout aggiornato e un client compatibile con skill locali;
l'accesso ai servizi e la rilevazione delle skill vanno verificati sul PC.

Per le verifiche: python scripts/check.py --plan mostra i comandi senza eseguirli.
python scripts/check.py esegue fmt, check, Clippy, test e build release
con il workspace e tutte le feature; richiede Python 3.10+ solo nello sviluppo,
Rust della toolchain versionata e gli strumenti di compilazione del sistema.
python scripts/check.py --scope core verifica soltanto core e io quando il
desktop non è compilabile nell'ambiente. Non equivale a verifica workspace.
Le prove native Windows restano quelle di docs/development.md; non dichiararle
superate da un cloud privo di desktop. Non cambiare toolchain per aggirare
problemi di rete o dipendenze: registrarli come blocchi.

Le PR mantengono base rust-rewrite. LEO-5 parte dalla baseline
codex/ui-refinements a3d62ef: il diff della sola configurazione va revisionato
contro quello SHA, distinto dal diff complessivo della migrazione.
L'integrazione GitHub–Linear e gli eventuali job ogni tre ore già documentati
sono configurazioni separate: questa attività non li abilita né li modifica.
