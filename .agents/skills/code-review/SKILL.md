---
name: code-review
description: "Revisionare un diff separando rispetto della specifica e qualità del codice."
---

# code-review

Adattamento locale per Stick Optimizer. Riferimento: docs/agents/workflow.md.

Fissare baseline SHA e head SHA; leggere il diff e verificare che sia completo.
Identificare specifica/ticket e fonti degli standard, incluso CODING_STANDARDS.md.
Se manca la specifica dichiararlo; non inventare requisiti.
Esaminare due assi distinti: Spec (requisiti, errori, lavoro estraneo) e
Standard (contratti, responsabilità, coerenza e regole documentate).
Quando il contesto consente ed è autorizzato, affidare le due letture a revisori
separati con lo stesso diff e solo i riferimenti necessari.
Se eseguite dallo stesso agente dichiarare autorevisione; non chiamarla indipendente.
Per ogni rilievo citare posizione, effetto e criterio. Separare difetti da
suggerimenti; non ripetere rilievi già coperti da strumenti superati.
Correggere i difetti e rieseguire i controlli interessati; registrare limiti.
Non approvare a nome dell'utente e non effettuare merge.
