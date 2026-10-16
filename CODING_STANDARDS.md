# Criteri per la revisione

Le regole operative e i vincoli di correttezza restano in AGENTS.md e nella
documentazione tecnica. Questo file raccoglie le valutazioni del revisore;
fmt, compilatore e Clippy controllano gli aspetti meccanici.

- Le interfacce espongono concetti di dominio con nomi coerenti al glossario.
- Ogni responsabilità vive nel componente appropriato: dominio nel core,
  persistenza in io, interazione in desktop. La GUI non duplica formule.
- Modifiche di schema, precisione e ordinamento sono intenzionali, documentate
  e valutate su tutti i consumatori.
- Le astrazioni risolvono necessità della specifica; evitare generalizzazioni
  speculative e refactoring estranei al ticket.
- I test osservano comportamento e contratti con attesi indipendenti dalla
  logica sotto test; non devono cristallizzare la struttura interna.
- Errori e annullamenti conservano dati e coerenza dei risultati; le prove
  documentano anche ciò che non certificano.

Distinguere violazioni documentate, difetti concreti e suggerimenti.
Per ogni rilievo citare file/riga, conseguenza e criterio interessato.
