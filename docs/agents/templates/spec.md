# Specifica: <titolo>

Ticket Linear: <URL>
Stato: bozza / concordata
Baseline: <branch e SHA verificati>

## Problema

<Trigger concreto e problema dal punto di vista dell'utente; comportamento attuale.>

## Soluzione e risultato desiderato

<Comportamento atteso dal punto di vista dell'utente nel perimetro accettato.>

## Storie utente e criteri di accettazione

1. Come <attore>, voglio <comportamento>, per <beneficio>.

<Elenco numerato completo dei casi concordati, inclusi errori e vincoli negativi.
Associare le storie ai criteri osservabili; non aggiungere funzioni per allungarlo.>

- [ ] <Dato ..., quando ..., allora ...>

## Decisioni

<Interfacce coinvolte, vincoli, unità, precisione e compatibilità.
Moduli, architettura, contratti/schema e interazioni concordate.
Separare decisioni concordate, assunzioni e domande aperte.
Evitare percorsi/codice nel contratto, salvo parti decisionali di un prototipo
con provenienza esplicita; URL/ID/SHA restano metadati di tracciabilità.>

## Verifica

<Comportamento esterno; confine pubblico più alto, preferibilmente esistente,
moduli e test analoghi nel repo. Fonte dell'accordo dell'utente sui confini
oppure proposta ancora da confermare. Esempi indipendenti, comandi e prove desktop.
Indicare cosa richiede Windows o intervento umano e cosa è solo compilazione.>

## Tracciabilità delle decisioni

| Risposta/decisione del grill | Storia/criterio | Verifica |
| --- | --- | --- |
| <Fonte e revisione, compresi numeri/unità ed esclusioni> | <Riferimento> | <Caso osservabile> |

## Fuori perimetro

<Esclusioni esplicite.>

## Dipendenze e rischi

<Ticket bloccanti, base PR, migrazioni e ripristino se pertinenti.>

## Note ulteriori

<Solo informazioni pertinenti non già coperte, senza nuove decisioni implicite.>

## Definizione accettata

<Fonte reale dell'accettazione dell'utente, data e revisione; altrimenti bozza.
La pubblicazione della specifica non applica ready-for-agent automaticamente.>

## Dimensionamento

<Stima totale T in token, motivazione da codice/contratti/test/review, incertezza,
data/revisione. Target 150.000. N = max(1, ceil(T / 150000)). Piano di split e
stime dei figli; giustificare N=1. Riferimento: docs/agents/token-sizing.md.>
